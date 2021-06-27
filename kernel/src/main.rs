#![no_std]
#![no_main]

use core::panic::PanicInfo;
use htkernel::screen::init_writer;
use htlib::boot::*;

#[no_mangle]
extern "C" fn kernel_entry(boot_info: &BootInfo) {
    init_writer(boot_info);

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
