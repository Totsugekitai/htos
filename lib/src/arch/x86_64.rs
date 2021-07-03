use x86_64;

#[inline]
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[inline]
pub fn nop_loop() -> ! {
    loop {
        x86_64::instructions::nop();
    }
}
