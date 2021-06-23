#![no_std]
#![no_main]
#![feature(abi_efiapi)]
//#![feature(default_alloc_error_handler)]

//#[macro_use]
//extern crate alloc;
//extern crate rlibc;

use core::{fmt::Write, panic::PanicInfo};
use uefi::prelude::*;
use uefi::proto::console::{gop::GraphicsOutput, text::Output};
use uefi::proto::media::{fs::SimpleFileSystem, file::*};
use uefi::table::boot::{MemoryType, MemoryDescriptor};
use htlib::{boot::*, elf::*, error::*};

#[entry]
fn efi_main(handle: Handle, st: SystemTable<Boot>) -> Status {
    let stdout = st.stdout();
    stdout.reset(false).expect_success("Failed to reset STDOUT");
    writeln!(stdout, "Hello, UEFI!").unwrap_or_else(|_| panic!());

    let mut boot_info: BootInfo = Default::default();

    let bs = st.boot_services();

    // initialize Graphics
    if let Ok(gop) = bs.locate_protocol::<GraphicsOutput>() {
        let gop = gop.unwrap();
        let gop = unsafe { &mut *gop.get() };

        boot_info.vram_base = gop.frame_buffer().as_mut_ptr() as usize as u64;

        let mode_info = gop.current_mode_info();

        let (width, height) = mode_info.resolution();
        boot_info.vram_width = width as u16;
        boot_info.vram_height = height as u16;

        let stride = mode_info.stride();
        boot_info.vram_stride = stride as u16;
    } else {
        writeln!(stdout, "Failed to call GOP protocol").unwrap();
        panic!();
    }

    // test frame buffer
    let vram_base = boot_info.vram_base;
    for i in 0..(boot_info.vram_width as u64 * boot_info.vram_height as u64) {
        let vram = (vram_base + i * 4) as *mut Pixel;
        let pixel = Pixel { blue: 0x50, green: 0x50, red: 0, _reserved: 0 };
        unsafe { core::ptr::write_volatile::<Pixel>(vram, pixel); }
    }

    // load kernel file
    let kernel_addr = match load_kernel(&bs) {
        Ok(kernel_addr) => kernel_addr,
        Err(s) => match s {
            Status::BUFFER_TOO_SMALL => {
                writeln!(stdout, "Failed to load kernel: buffer too small").unwrap();
                panic!();
            },
            Status::NOT_FOUND => {
                writeln!(stdout, "Failed to load kernel: not found").unwrap();
                panic!();
            },
            _ => {
                writeln!(stdout, "Failed to load kernel: unknown reason").unwrap();
                panic!();
            },
        }
    };

    let kernel_ehdr = unsafe { &*(kernel_addr as *const [u8] as *const Elf64Ehdr) };

    if kernel_ehdr.is_valid() {
        writeln!(stdout, "Valid ELF file").unwrap();
    } else {
        writeln!(stdout, "Invalid ELF file").unwrap();
        panic!();
    }

    if let Err(e) = load_segments(kernel_addr, stdout) {
        use htlib::error::ErrorKind::*;
        match e.kind {
            InvalidParameter => {
                writeln!(stdout, "Invalid segment parameter").unwrap();
                panic!();
            }
            //NotFound => {
            //    writeln!(stdout, "Not found LOAD segment").unwrap();
            //    panic!();
            //}
            _ => {
                writeln!(stdout, "Something error").unwrap();
                panic!();
            }
        }
    }

    let kernel_entry = unsafe {
        core::mem::transmute::<u64, extern "sysv64" fn(bi: &BootInfo)>(kernel_ehdr.e_entry)
    };

    writeln!(stdout, "elf file addr: 0x{:x}", kernel_addr as *const [u8] as *const u8 as usize).unwrap();
    writeln!(stdout, "kernel entry: 0x{:x}", kernel_entry as usize).unwrap();

    // for debug
    for i in 0..100 {
        let a = kernel_entry as u64;
        unsafe { write!(stdout, "{:x} ", *((a + i) as *const u8)).unwrap(); }
    }

    // exit boot services
    //let max_mmap_size = bs.memory_map_size() + 8 * core::mem::size_of::<MemoryDescriptor>();
    //let mut mmap_region = vec![0; max_mmap_size].into_boxed_slice();
    //let (_st, _it) = st.exit_boot_services(handle, &mut mmap_region)
    //    .expect_success("Failed to boot services");

    // jump!!!
    kernel_entry(&boot_info);

    //writeln!(stdout, "boot unreachable area").unwrap();

    Status::SUCCESS
}

fn load_kernel(bs: &BootServices) -> Result<&'static [u8], Status> {
    let simple_fs = match bs.locate_protocol::<SimpleFileSystem>() {
        Ok(simple_fs) => simple_fs.unwrap(),
        Err(e) => return Err(e.status()),
    };
    let simple_fs = unsafe { &mut *simple_fs.get() };

    let mut rootdir = match simple_fs.open_volume() {
        Ok(rootdir) => rootdir.unwrap(),
        Err(e) => return Err(e.status()),
    };

    const KERNEL_FILE: &str = "htkernel.elf";
    let kernel_handle = match rootdir.open(KERNEL_FILE, FileMode::Read, FileAttribute::empty()) {
        Ok(kernel_handle) => kernel_handle.unwrap(),
        Err(e) => return Err(e.status()),
    };

    let mut kernel_file = match kernel_handle.into_type().unwrap().unwrap() {
        FileType::Regular(file) => file,
        FileType::Dir(_) => return Err(Status::ACCESS_DENIED),
    };

    if let Err(e) = kernel_file.set_position(RegularFile::END_OF_FILE) {
        return Err(e.status());
    };

    let kernel_file_size = match kernel_file.get_position() {
        Ok(size) => size.unwrap(),
        Err(e) => return Err(e.status()),
    } as usize;

    if let Err(e) = kernel_file.set_position(0) {
        return Err(e.status());
    };

    let pool = match bs.allocate_pool(MemoryType::LOADER_DATA, kernel_file_size) {
        Ok(pool) => pool.unwrap(),
        Err(e) => return Err(e.status()),
    };

    let buf = unsafe { core::slice::from_raw_parts_mut(pool, kernel_file_size) };

    if let Err(e) = kernel_file.read(buf) {
        return Err(e.status());
    };

    Ok(buf)
}

pub fn load_segments(head: &[u8], so: &mut Output) -> Result<(), Error> {
    let ehdr = unsafe { &*(head as *const [u8] as *const Elf64Ehdr) };
    for i in 0..ehdr.e_phnum as u64 {
        let phdr_offset = ehdr.e_phoff + (ehdr.e_phentsize as u64) * i;
        writeln!(so, "phdr_offset: {}", phdr_offset).unwrap();
        writeln!(so, "e_phentsize: {}", ehdr.e_phentsize).unwrap();
        unsafe {
            let phdr = &*((head as *const [u8] as *const u8 as u64 + phdr_offset) as *const Elf64Phdr);
            writeln!(so, "head: 0x{:x}", head as *const [u8] as *const u8 as u64).unwrap();
            writeln!(so, "phdr: 0x{:x}", phdr as *const Elf64Phdr as u64).unwrap();
            writeln!(so, "  Type: {:x}", phdr.p_type).unwrap();
            writeln!(so, "  Flags: {}", phdr.p_flags).unwrap();
            writeln!(so, "  Offset: {}", phdr.p_offset).unwrap();
            writeln!(so, "  Vaddr: {}", phdr.p_vaddr).unwrap();
            writeln!(so, "  Paddr: {}", phdr.p_paddr).unwrap();
            writeln!(so, "  Memsz: {}", phdr.p_memsz).unwrap();
            writeln!(so, "  Filesz: {}", phdr.p_filesz).unwrap();
            writeln!(so, "  Align: {}", phdr.p_align).unwrap();
            if let Err(e) = phdr.load_segmemt(head) {
                use htlib::error::ErrorKind::*;
                match e.kind {
                    NotFound => { writeln!(so, "This segment is not LOAD").unwrap() },
                    _ => { return Err(e); }
                }
            } else {
                writeln!(so, "Load Segment").unwrap();
            }
        }
    }
    Ok(())
}


#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
