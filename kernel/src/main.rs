#![no_std]
#![no_main]

use core::panic::PanicInfo;
use htkernel::{text::{WRITER, Writer}, thread::main::MainThreadMarker};
use htlib::boot::*;
use htkernel::graphics::*;

#[no_mangle]
extern "C" fn kernel_entry(bi: &BootInfo) {
    let marker = unsafe { MainThreadMarker::new() };
    WRITER.set(marker, Writer::from_boot_info(bi));
    let mut writer = WRITER.borrow_refmut(marker);
    let writer = &mut *writer;
    writer.fb.write_background(Color::Black);
    writer.write_byte(b'A', Color::Green);
    writer.write_byte(b'\n', Color::Green);
    writer.write_byte(b'B', Color::Green);
    writer.write_byte(b'!', Color::Green);

    //FRAME_BUFFER.set(marker, FrameBuffer::from_boot_info(bi));
    //let mut fb = FRAME_BUFFER.borrow_refmut(marker);
    //let fb = &mut *fb;
    //fb.write_background(Color::Black);

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
