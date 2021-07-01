#![no_std]
#![no_main]

use core::panic::PanicInfo;
use htkernel::arch::x86_64::interrupts;
use htkernel::println;
use htkernel::screen::init_writer;
use htlib::arch::x86_64;
use htlib::boot::*;

#[no_mangle]
extern "C" fn kernel_entry(boot_info: &BootInfo) {
    init_writer(boot_info);
    println!("Hello, HTOS!");
    interrupts::init_idt();
    println!("init IDT");

    x86_64::int3();

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    println!("panic!");
    loop {}
}
