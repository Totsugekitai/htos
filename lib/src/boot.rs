#[repr(C)]
#[derive(Default)]
pub struct BootInfo {
    pub vram_base: u64,
    pub vram_width: u16,
    pub vram_height: u16,
    pub vram_stride: u16,
}
