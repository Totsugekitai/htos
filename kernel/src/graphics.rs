use crate::fonts;

pub struct FrameBuffer {
    base: &'static mut [u8],
    width: u64,
    height: u64,
    stride: u64,
}
