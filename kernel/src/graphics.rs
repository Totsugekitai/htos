use htlib::boot::{Pixel, PixelFormat};
use crate::fonts;

#[repr(C)]
struct FrameBuffer {
    base: &'static mut [Pixel],
    width: u64,
    height: u64,
    stride: u64,
}
