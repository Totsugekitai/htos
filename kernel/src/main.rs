#![no_std]
#![no_main]

use core::panic::PanicInfo;
use htlib::boot::*;
use htkernel::*;

#[no_mangle]
extern "C" fn kernel_entry(bi: &BootInfo) {
    let mut fb: graphics::FrameBuffer = Default::default();
    fb.init(bi);

    fb.write_background(htkernel::graphics::Color::Black);

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
