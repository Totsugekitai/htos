use htlib::boot::*;

#[repr(C)]
#[derive(Default)]
pub struct FrameBuffer {
    base: &'static mut [Pixel],
    width: usize,
    height: usize,
    stride: usize,
    pixel_format: PixelFormat,
}

#[repr(u32)]
#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum Color {
    Black = 0x00000000,
    White = 0xffffff00,
    Red   = 0xff000000,
    Green = 0x00ff0000,
    Blue  = 0x0000ff00,
}

impl FrameBuffer {
    pub fn init(&mut self, bi: &BootInfo) {
        self.base = unsafe { core::slice::from_raw_parts_mut(bi.vram_base as *mut Pixel, bi.vram_width as usize * bi.vram_height as usize) };
        self.width = bi.vram_width as usize;
        self.height = bi.vram_height as usize;
        self.stride = bi.vram_stride as usize;
        self.pixel_format = bi.pixel_format;
    }

    pub fn write_background(&mut self, color: Color) {
        for dy in 0..self.height {
            for dx in 0..self.width {
                self.write(dx, dy, color);
            }
        }
    }

    pub fn write(&mut self, x: usize, y: usize, color: Color) {
        match self.pixel_format {
            PixelFormat::Rgb => { self.write_rgb(x, y, color); },
            PixelFormat::Bgr => { self.write_bgr(x, y, color); },
            _ => (),
        }
    }

    fn write_rgb(&mut self, x: usize, y: usize, color: Color) {
        use Color::*;
        match color {
            Black => { unsafe { self.write_pixel(x, y, Pixel { dot: (color as u32).to_be() }); } },
            White => { unsafe { self.write_pixel(x, y, Pixel { dot: (color as u32).to_be() }); } },
            Red   => { unsafe { self.write_pixel(x, y, Pixel { dot: (color as u32).to_be() }); } },
            Green => { unsafe { self.write_pixel(x, y, Pixel { dot: (color as u32).to_be() }); } },
            Blue  => { unsafe { self.write_pixel(x, y, Pixel { dot: (color as u32).to_be() }); } },
        }
    }

    fn write_bgr(&mut self, x: usize, y: usize, color: Color) {
        use Color::*;
        let rgb_array = (color as u32).to_be_bytes();
        let red = rgb_array[0];
        let green = rgb_array[1];
        let blue = rgb_array[2];
        let bgr_array = [blue, green, red, 0];
        let bgr = unsafe { core::mem::transmute::<[u8; 4], u32>(bgr_array) };
        match color {
            Black => { unsafe { self.write_pixel(x, y, Pixel { dot: bgr }); } },
            White => { unsafe { self.write_pixel(x, y, Pixel { dot: bgr }); } },
            Red   => { unsafe { self.write_pixel(x, y, Pixel { dot: bgr }); } },
            Green => { unsafe { self.write_pixel(x, y, Pixel { dot: bgr }); } },
            Blue  => { unsafe { self.write_pixel(x, y, Pixel { dot: bgr }); } },
        }
    }

    unsafe fn write_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        core::ptr::write_volatile(&mut self.base[x + y * self.stride], pixel);
    }
}
