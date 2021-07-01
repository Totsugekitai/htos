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

#[inline]
pub fn int3() {
    unsafe {
        asm!("int3");
    }
}
