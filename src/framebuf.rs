use crate::consts::*;
use crate::memory::Memory;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{OriginDimensions, Point};
use embedded_graphics::pixelcolor::raw::{RawData, RawU2};
use embedded_graphics::pixelcolor::{PixelColor, Rgb565};
use embedded_graphics::prelude::{Pixel, Size};

/// Represents one of four colors in the palette with a 0-3 number, taking 2 bits.
#[derive(PartialEq, Copy, Clone)]
pub struct Color4(pub u8);

impl Color4 {
    fn as_byte(&self) -> u8 {
        self.0
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

impl From<RawU2> for Color4 {
    fn from(value: RawU2) -> Self {
        Self(value.into_inner())
    }
}

pub struct FrameBuf<'a> {
    // https://wasm4.org/docs/reference/memory/#palette
    pub palette_raw: &'a [u8],

    // https://wasm4.org/docs/reference/memory/#framebuffer
    pub frame_buf: &'a mut [u8],
}

impl<'a> FrameBuf<'a> {
    pub fn from_memory(memory: &'a mut Memory) -> FrameBuf<'a> {
        Self {
            palette_raw: memory.palette,
            frame_buf:   memory.frame_buf,
        }
    }

    pub fn get_color(&self, color_id: u8) -> Rgb565 {
        let start = (color_id * 4) as usize;
        let raw_color = &self.palette_raw[start..(start + 4)];
        let r = (raw_color[3] as f64 * 32. / 256.) as u8;
        let g = (raw_color[2] as f64 * 64. / 256.) as u8;
        let b = (raw_color[1] as f64 * 32. / 256.) as u8;
        Rgb565::new(r, g, b)
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
        let byte = self.frame_buf[byte_offset];
        let color: u8 = draw_color.as_byte();
        self.frame_buf[byte_offset] = (color << shift) | (byte & !mask);
        Ok(())
    }

    pub fn blit(
        &mut self,
        draw_colors: &[u8],
        sprite: &[u8],
        dst_x: i32,
        dst_y: i32,
        width: i32,
        height: i32,
        src_x: i32,
        src_y: i32,
        src_stride: i32,
        flags: u32,
    ) {
        let bpp2 = flags & BLIT_2BPP != 0;
        let mut flip_x = flags & BLIT_FLIP_X != 0;
        let flip_y = flags & BLIT_FLIP_Y != 0;
        let rotate = flags & BLIT_ROTATE != 0;

        let colors = draw_colors[0] as u16 | ((draw_colors[1] as u16) << 8);

        // Clip rectangle to screen
        let clip_x_min: i32;
        let clip_y_min: i32;
        let clip_x_max: i32;
        let clip_y_max: i32;
        if rotate {
            flip_x = !flip_x;
            clip_x_min = i32::max(0, dst_y) - dst_y;
            clip_y_min = i32::max(0, dst_x) - dst_x;
            clip_x_max = i32::min(width, 160 - dst_y);
            clip_y_max = i32::min(height, 160 - dst_x);
        } else {
            clip_x_min = i32::max(0, dst_x) - dst_x;
            clip_y_min = i32::max(0, dst_y) - dst_y;
            clip_x_max = i32::min(width, 160 - dst_x);
            clip_y_max = i32::min(height, 160 - dst_y);
        }

        // Iterate pixels in rectangle
        for y in clip_y_min..clip_y_max {
            for x in clip_x_min..clip_x_max {
                // Calculate sprite target coords
                let tx = dst_x + if rotate { y } else { x };
                let ty = dst_y + if rotate { x } else { y };

                // Calculate sprite source coords
                let sx = src_x + if flip_x { width - x - 1 } else { x };
                let sy = src_y + if flip_y { height - y - 1 } else { y };

                // Sample the sprite to get a color index
                let bit_idx = sy * src_stride + sx;
                let color_idx = if bpp2 {
                    let byte = sprite[(bit_idx >> 2) as usize];
                    let shift = 6 - ((bit_idx & 0x03) << 1);
                    (byte >> shift) & 0x3
                } else {
                    let byte = sprite[(bit_idx >> 3) as usize];
                    let shift = 7 - (bit_idx & 0x07);
                    (byte >> shift) & 0x1
                };

                // Get the final color using the drawColors indirection
                let dc = (colors >> (color_idx << 2)) & 0x0f;
                if dc != 0 {
                    let color = Color4(((dc - 1) & 0x03) as u8);
                    let point = Point::new(tx, ty);
                    let pixel = Pixel(point, color);
                    self.set_pixel(pixel).unwrap();
                }
            }
        }
    }

    pub fn iter(&'a self) -> PixelIterator<'a> {
        PixelIterator {
            buf:     self,
            pos:     0,
            palette: [
                self.get_color(0),
                self.get_color(1),
                self.get_color(2),
                self.get_color(3),
            ],
        }
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

pub struct PixelIterator<'a> {
    buf:     &'a FrameBuf<'a>,
    pos:     usize,
    palette: [Rgb565; 4],
}

impl<'a> Iterator for PixelIterator<'a> {
    type Item = Rgb565;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= 160 * 160 {
            return None;
        }
        let byte = self.buf.frame_buf[self.pos / 4];
        let bit_offset = self.pos % 4 * 2;
        let color_id = (byte >> bit_offset) & 0b11;
        let color = self.palette[color_id as usize];
        self.pos += 1;
        Some(color)
    }
}
