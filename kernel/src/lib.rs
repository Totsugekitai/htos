#![no_std]
#![feature(const_fn_trait_bound)]
#![feature(abi_x86_interrupt)]
#![feature(lang_items)]

#[macro_use]
extern crate bitflags;

pub mod graphics;
pub mod screen;

pub mod arch;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
