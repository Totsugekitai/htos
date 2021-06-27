#![no_std]
#![no_main]

use core::{mem::size_of, panic::PanicInfo};
use htkernel::thread::main::MainThreadMarker;
use htlib::boot::*;
use htkernel::graphics::*;

#[no_mangle]
extern "C" fn kernel_entry(bi: &BootInfo) {
    // test
    fn paint(bi: &BootInfo, dot: u32) {
        let vram = bi.vram_base as usize;
        let height = bi.vram_height as usize;
        let width = bi.vram_width as usize;
        for i in 0..(width * height) {
            unsafe { *((vram + i * size_of::<Pixel>()) as *mut Pixel) = Pixel { dot } };
        }
    }
    paint(bi, 0xffffffff);

    let marker = unsafe { MainThreadMarker::new() };
    FRAME_BUFFER.set(marker, FrameBuffer::from_boot_info(bi));

    let mut fb = FRAME_BUFFER.borrow_refmut(marker);
    let fb = &mut *fb;

    fb.write_background(Color::Black);

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
