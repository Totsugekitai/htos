#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::{fmt::Write, panic::PanicInfo};
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::media::{fs::SimpleFileSystem, file::*};
use htlib::boot::BootInfo;

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
    load_kernel(&boot_services);

    loop {}
}

const KERNEL_FILE: &str = "htkernel.elf";

fn load_kernel(bs: &BootServices) {
    if let Ok(simple_fs) = bs.locate_protocol::<SimpleFileSystem>() {
        let simple_fs = simple_fs.unwrap();
        let simple_fs = unsafe { &mut *simple_fs.get() };

        if let Ok(rootdir) = simple_fs.open_volume() {
            let kernel = rootdir.unwrap().open(KERNEL_FILE, FileMode::Read, FileAttribute::empty());
        }
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
