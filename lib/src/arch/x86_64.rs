#[inline]
pub fn halt() {
    unsafe {
        asm!("hlt");
    }
}

#[inline]
pub fn nop() {
    unsafe {
        asm!("nop");
    }
}
