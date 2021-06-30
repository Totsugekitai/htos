#![no_std]
#![feature(asm)]
#![feature(const_fn_trait_bound)]
#![feature(abi_x86_interrupt)]
#![feature(lang_items)]
#![feature(const_fn_fn_ptr_basics)]

#[macro_use]
extern crate bitflags;

pub mod graphics;
pub mod screen;

pub mod arch;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
