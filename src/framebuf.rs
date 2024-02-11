use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{OriginDimensions, Point};
use embedded_graphics::pixelcolor::raw::{RawData, RawU2};
use embedded_graphics::pixelcolor::{Bgr888, PixelColor, Rgb565};
use embedded_graphics::prelude::{Pixel, Size};

/// Represents one of four colors in the palette with a 0-3 number, taking 2 bits.
#[derive(PartialEq, Copy, Clone)]
pub struct Color4(RawU2);

impl Color4 {
    fn as_byte(&self) -> u8 {
        self.0.into_inner()
    }

    // pub fn from_byte(byte: &u8) -> &[Color4] {
    //     RawU2::new();
    //     std::mem::transmute(src)
    // }

    // pub fn from_bytes(bytes: &[u8]) -> &[Color4] {
    //     todo!()
    // }
}

impl PixelColor for Color4 {
    type Raw = RawU2;
}

pub struct FrameBuf<'a> {
    // https://wasm4.org/docs/reference/memory/#palette
    palette_raw: &'a [u8],

    // https://wasm4.org/docs/reference/memory/#framebuffer
    data: &'a mut [u8],
}

impl<'a> FrameBuf<'a> {
    pub fn from_memory(data: &'a mut [u8]) -> FrameBuf<'a> {
        // https://doc.rust-lang.org/nomicon/borrow-splitting.html
        let ptr = data.as_mut_ptr();
        let palette_raw: &'a [u8];
        let frame_buffer: &'a mut [u8];
        unsafe {
            palette_raw = core::slice::from_raw_parts(ptr.add(4), 16);
            frame_buffer = core::slice::from_raw_parts_mut(ptr.add(0x19a0), 160 * 160 / 4);
        }
        FrameBuf {
            palette_raw,
            data: frame_buffer,
        }
    }

    fn get_color(&self, color_id: u8) -> Rgb565 {
        let start = (color_id * 4) as usize;
        let raw_color = &self.palette_raw[start..(start + 4)];
        let r = raw_color[1];
        let g = raw_color[2];
        let b = raw_color[3];
        let color = Bgr888::new(r, g, b);
        color.into()
    }

    fn set_pixel(&mut self, pixel: Pixel<Color4>) -> Result<(), ()> {
        let Pixel(Point { x, y }, draw_color) = pixel;
        if !(0..160).contains(&x) {
            return Err(());
        }
        if !(0..160).contains(&y) {
            return Err(());
        }
        let pixel_offset = y as usize * 160 + x as usize;
        let byte_offset = pixel_offset / 4;
        let shift = (x as u8 & 0b11) << 1;
        let mask = 0b11 << shift;
        let byte = self.data[byte_offset];
        let color: u8 = draw_color.as_byte();
        self.data[byte_offset] = (color << shift) | (byte & !mask);
        Ok(())
    }

    pub fn iter(&'a self) -> PixelIterator<'a> {
        PixelIterator { buf: self, pos: 0 }
    }
}

impl<'a> OriginDimensions for FrameBuf<'a> {
    fn size(&self) -> Size {
        Size {
            width:  160,
            height: 160,
        }
    }
}

impl<'a> DrawTarget for FrameBuf<'a> {
    type Color = Color4;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.set_pixel(pixel)?;
        }
        Ok(())
    }
}

struct PixelIterator<'a> {
    buf: &'a FrameBuf<'a>,

    // The next color offset.
    pos: usize,
}

impl<'a> Iterator for PixelIterator<'a> {
    type Item = Pixel<Rgb565>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= 160 * 160 {
            return None;
        }
        let mut y = self.pos as i32 / 4 / 160;
        let mut x = self.pos as i32 / 4 % 160;
        if x >= 160 {
            x = 0;
            y += 1;
        }
        if y >= 160 {
            return None;
        }
        let byte = self.buf.data[self.pos / 4];
        let bit_offset = self.pos % 4 * 2;
        let color_id = (byte >> bit_offset) & 0b11;
        let color = self.buf.get_color(color_id);
        self.pos += 1;
        let point = Point { x, y };
        Some(Pixel(point, color))
    }
}
