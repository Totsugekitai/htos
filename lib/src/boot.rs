#[repr(C)]
#[derive(Default)]
pub struct BootInfo {
    pub vram_base: u64,
    pub vram_width: u16,
    pub vram_height: u16,
    pub vram_stride: u16,
    pub pixel_format: PixelFormat,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PixelFormat {
    Rgb = 0,
    Bgr = 1,
    Bitmask = 2,
    BltOnly = 3,
}

impl Default for PixelFormat {
    fn default() -> Self { PixelFormat::Bgr }
}

#[repr(C)]
pub struct Pixel {
    pub dot: u32,
}

