#![no_std]
#![feature(asm)]
#![feature(const_fn_trait_bound)]
#![feature(abi_x86_interrupt)]
#![feature(lang_items)]
#![feature(const_fn_fn_ptr_basics)]

extern crate bitflags;

use htlib::boot::BootInfo;

pub mod graphics;
pub mod screen;

pub mod arch;

pub fn init(boot_info: &BootInfo) {
    screen::init_writer(boot_info);
    arch::x86_64::gdt::init();
    arch::x86_64::interrupts::init_idt();
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
