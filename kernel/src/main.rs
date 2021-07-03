#![no_std]
#![no_main]

use core::panic::PanicInfo;
use htkernel::println;
use htlib::boot::BootInfo;

#[no_mangle]
extern "C" fn kernel_entry(boot_info: &BootInfo) {
    htkernel::init(boot_info);
    println!("Hello, HTOS!");

    unsafe {
        core::ptr::write_volatile(0xdeadbeef as *mut u64, 42);
    };

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
