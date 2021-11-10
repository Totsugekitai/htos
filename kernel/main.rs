#![no_std]
#![no_main]
#![feature(asm)]
#![feature(const_fn_trait_bound)]
#![feature(abi_x86_interrupt)]
#![feature(lang_items)]
#![feature(const_fn_fn_ptr_basics)]

extern crate bitflags;

use core::panic::PanicInfo;
use htlib::boot::BootInfo;
use htlib::arch::x64;

#[macro_use]
mod graphics;
mod screen;
mod arch;

pub fn init(boot_info: &BootInfo) {
    screen::init_writer(boot_info);
    arch::x64::gdt::init();
    arch::x64::interrupts::init_idt();
    arch::x64::paging::init_kernel_page_table();
}

#[no_mangle]
extern "C" fn kernel_entry(boot_info: &BootInfo) {
    init(boot_info);
    println!("Hello, HTOS!");

    x64::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    x64::hlt_loop();
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

