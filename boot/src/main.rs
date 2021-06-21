#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[macro_use]
extern crate alloc;

use core::{fmt::Write, panic::PanicInfo};
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::media::{fs::SimpleFileSystem, file::*};
use htlib::boot::BootInfo;
use uefi::table::boot::MemoryType;

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {
    let stdout = st.stdout();
    stdout.reset(false).expect_success("Failed to reset STDOUT");
    writeln!(stdout, "Hello, UEFI!").unwrap_or_else(|_| panic!());

    let mut boot_info : BootInfo = Default::default();

    let boot_services = st.boot_services();

    // initialize Graphics
    if let Ok(gop) = boot_services.locate_protocol::<GraphicsOutput>() {
        let gop = gop.unwrap();
        let gop = unsafe { &mut *gop.get() };

        let mut frame_buffer = gop.frame_buffer();
        boot_info.vram_base = frame_buffer.as_mut_ptr() as usize as u64;

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

    // load kernel file
    let kernel = match load_kernel(&boot_services) {
        Ok(kernel_addr) => kernel_addr,
        Err(s) => match s {
            Status::BUFFER_TOO_SMALL => {
                writeln!(stdout, "Failed to load kernel: buffer too small");
                panic!();
            },
            Status::NOT_FOUND => {
                writeln!(stdout, "Failed to load kernel: not found");
                panic!();
            },
            _ => {
                writeln!(stdout, "Failed to load kernel: unknown reason");
                panic!();
            },
        }
    };

    loop {}
}

const KERNEL_FILE: &str = "htkernel.elf";

fn load_kernel(bs: &BootServices) -> Result<&'static mut [u8], Status> {
    let simple_fs = match bs.locate_protocol::<SimpleFileSystem>() {
        Ok(simple_fs) => simple_fs.unwrap(),
        Err(e) => return Err(e.status()),
    };
    let simple_fs = unsafe { &mut *simple_fs.get() };

    let rootdir = match simple_fs.open_volume() {
        Ok(rootdir) => rootdir.unwrap(),
        Err(e) => return Err(e.status()),
    };

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

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
