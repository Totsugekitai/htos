#![no_std]
#![no_main]

use core::panic::PanicInfo;
use htkernel::println;
use htlib::arch::x86_64;
use htlib::boot::BootInfo;

#[no_mangle]
extern "C" fn kernel_entry(boot_info: &BootInfo) {
    htkernel::init(boot_info);
    println!("Hello, HTOS!");

    x86_64::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    x86_64::hlt_loop();
}
