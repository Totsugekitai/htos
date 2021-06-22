#[repr(C)]
#[derive(Default)]
pub struct BootInfo {
    pub vram_base: u64,
    pub vram_width: u16,
    pub vram_height: u16,
    pub vram_stride: u16,
}

#[repr(C)]
pub struct Pixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub _reserved: u8,
}

