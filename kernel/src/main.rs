#![no_std]
#![no_main]

use core::panic::PanicInfo;
use htlib::boot::*;
use htkernel::*;

#[no_mangle]
extern "C" fn kernel_entry(bi: &BootInfo) {
    let vram_base = bi.vram_base;
    for i in 0..(bi.vram_width as u64 * bi.vram_height as u64) {
        let vram = (vram_base + i * core::mem::size_of::<Pixel>() as u64) as *mut Pixel;
        let pixel = Pixel { dot: [0x50, 0x50, 0x50, 0] };
        unsafe { core::ptr::write_volatile::<Pixel>(vram, pixel); }
    }

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
