use crate::fonts;
use htlib::boot::*;

#[repr(C)]
struct FrameBuffer {
    base: &'static mut [Pixel],
    width: u64,
    height: u64,
    stride: u64,
}

impl FrameBuffer {
    fn init(&self, bi: &BootInfo) {
        self.base = core::mem::trbi.vram_base;
    }
}
