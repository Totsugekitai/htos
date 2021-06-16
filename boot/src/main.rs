#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::fmt::Write;
use core::panic::PanicInfo;

use uefi::prelude::*;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {
    st.stdout().reset(false).expect_success("Failed to reset STDOUT");

    writeln!(st.stdout(), "Hello, UEFI!").unwrap();

    loop {}
}
