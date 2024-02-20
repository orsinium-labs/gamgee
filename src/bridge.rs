use crate::consts::*;
use crate::framebuf::{Color4, FrameBuf};
use crate::memory::Memory;
use embedded_graphics::geometry::Point;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Ellipse, Line, PrimitiveStyle, Rectangle, StyledDrawable};
use pybadge_high::PyBadge;

static FONT: &[u8; 1792] = include_bytes!("font.bin");

pub struct Bridge {
    pybadge: PyBadge,
    memory:  Option<wasmi::Memory>,
    frame:   usize,
}

impl Bridge {
    pub fn new(pybadge: PyBadge) -> Self {
        Self {
            memory: None,
            pybadge,
            frame: 0,
        }
    }

    /// Initialize memory and stuff. Called before the application is started.
    pub fn init(&mut self, memory: wasmi::Memory, data: &mut [u8]) {
        self.memory = Some(memory);
        // init the color palette

        // ugly colors for better debugging
        // write32le(&mut data[PALETTE..], 0xffffff);
        // write32le(&mut data[PALETTE + 4..], 0x0000ff);
        // write32le(&mut data[PALETTE + 8..], 0xff0000);
        // write32le(&mut data[PALETTE + 12..], 0x00ff00);

        // The original default palette
        write32le(&mut data[PALETTE..], 0xe0f8cf);
        write32le(&mut data[PALETTE + 4..], 0x86c06c);
        write32le(&mut data[PALETTE + 8..], 0x306850);
        write32le(&mut data[PALETTE + 12..], 0x071821);

        // https://lospec.com/palette-list/ice-cream-gb
        // write32le(&mut data[PALETTE..], 0xd3f6ff);
        // write32le(&mut data[PALETTE + 4..], 0x75a8f9);
        // write32le(&mut data[PALETTE + 8..], 0x6f6beb);
        // write32le(&mut data[PALETTE + 12..], 0x583f7c);

        write16le(&mut data[DRAW_COLORS..], 0x0312);
        write32le(&mut data[MOUSE_X..], 0x7fff_7fff);

        // let frame_buf = FrameBuf::from_memory(data);
    }

    pub fn update(&mut self, data: &mut [u8]) {
        let mut memory = Memory::from_bytes(data);
        self.frame = (self.frame + 1) % 5;
        if self.frame == 0 {
            let frame_buf = FrameBuf::from_memory(&mut memory);
            let area = Rectangle::new(Point::new(0, 0), Size::new(160, 160));
            self.pybadge
                .display
                .fill_contiguous(&area, frame_buf.iter())
                .unwrap();
        }
        self.clear_frame_buffer(data);
        self.update_gamepad(data);
    }

    fn clear_frame_buffer(&self, data: &mut [u8]) {
        // https://wasm4.org/docs/reference/memory#system_flags
        if data[SYSTEM_FLAGS] & SYSTEM_PRESERVE_FRAMEBUFFER != 0 {
            return;
        }
        #[allow(clippy::needless_range_loop)]
        for addr in 0x00a0..0x19a0 {
            data[addr] = 0;
        }
    }

    fn update_gamepad(&mut self, data: &mut [u8]) {
        self.pybadge.buttons.update();
        // https://wasm4.org/docs/reference/memory#gamepads
        data[GAMEPAD1] = 0;
        if self.pybadge.buttons.a_pressed() {
            data[GAMEPAD1] |= BUTTON_2;
        }
        if self.pybadge.buttons.b_pressed() {
            data[GAMEPAD1] |= BUTTON_1;
        }
        if self.pybadge.buttons.left_pressed() {
            data[GAMEPAD1] |= BUTTON_LEFT;
        }
        if self.pybadge.buttons.right_pressed() {
            data[GAMEPAD1] |= BUTTON_RIGHT;
        }
        if self.pybadge.buttons.up_pressed() {
            data[GAMEPAD1] |= BUTTON_UP;
        }
        if self.pybadge.buttons.down_pressed() {
            data[GAMEPAD1] |= BUTTON_DOWN;
        }
    }

    pub fn wasm4_blit(
        &mut self,
        data: &mut [u8],
        sprite_ptr: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        flags: u32,
    ) {
        let memory = Memory::from_bytes(data);
        let size = (width * height) as u32 / 4;
        let sprite = get_user_data(memory.user_data, sprite_ptr, size);
        let mut frame_buf = FrameBuf {
            palette_raw: memory.palette,
            frame_buf:   memory.frame_buf,
        };
        frame_buf.blit(
            memory.draw_colors,
            sprite,
            x,
            y,
            width,
            height,
            0,
            0,
            width,
            flags,
        )
    }

    pub fn wasm4_blit_sub(
        &self,
        data: &mut [u8],
        sprite_ptr: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        src_x: i32,
        src_y: i32,
        stride: i32,
        flags: u32,
    ) {
        let memory = Memory::from_bytes(data);
        let size = (stride * (src_y + height)) as u32 / 4;
        let sprite = get_user_data(memory.user_data, sprite_ptr, size);
        let mut frame_buf = FrameBuf {
            palette_raw: memory.palette,
            frame_buf:   memory.frame_buf,
        };
        frame_buf.blit(
            memory.draw_colors,
            sprite,
            x,
            y,
            width,
            height,
            src_x,
            src_y,
            stride,
            flags,
        )
    }

    pub fn wasm4_hline(&mut self, data: &mut [u8], x: i32, y: i32, len: i32) {
        self.wasm4_line(data, x, y, x + len, y)
    }

    pub fn wasm4_vline(&mut self, data: &mut [u8], x: i32, y: i32, len: i32) {
        self.wasm4_line(data, x, y, x, y + len)
    }

    pub fn wasm4_line(&mut self, data: &mut [u8], x1: i32, y1: i32, x2: i32, y2: i32) {
        let line = Line::new(Point::new(x1, y1), Point::new(x2, y2));
        let mut memory = Memory::from_bytes(data);
        let Some(color) = get_draw_color(memory.draw_colors, 1) else {
            return;
        };
        let style = PrimitiveStyle::with_stroke(color, 1);
        let mut frame_buf = FrameBuf::from_memory(&mut memory);
        line.draw_styled(&style, &mut frame_buf);
    }

    pub fn wasm4_oval(&mut self, data: &mut [u8], x: i32, y: i32, width: u32, height: u32) {
        let ellipse = Ellipse::new(Point::new(x, y), Size::new(width, height));
        let mut memory = Memory::from_bytes(data);
        let style = get_shape_style(memory.draw_colors);
        let mut frame_buf = FrameBuf::from_memory(&mut memory);
        ellipse.draw_styled(&style, &mut frame_buf);
    }

    pub fn wasm4_rect(&mut self, data: &mut [u8], x: i32, y: i32, width: u32, height: u32) {
        let rect = Rectangle::new(Point::new(x, y), Size::new(width, height));
        let mut memory = Memory::from_bytes(data);
        let style = get_shape_style(memory.draw_colors);
        let mut frame_buf = FrameBuf::from_memory(&mut memory);
        rect.draw_styled(&style, &mut frame_buf);
    }

    pub fn wasm4_text(&mut self, data: &mut [u8], text_ptr: u32, x: i32, y: i32) {
        let memory = Memory::from_bytes(data);
        let str_data: &[u8] = get_user_data(memory.user_data, text_ptr, 1024);
        let mut frame_buf = FrameBuf {
            palette_raw: memory.palette,
            frame_buf:   memory.frame_buf,
        };
        let mut char_x = x;
        let mut char_y = y;
        for char in str_data {
            let char = *char as i32;
            if char == 0 {
                break;
            }
            // newline
            if char == 10 {
                char_y += 8;
                char_x = x;
                continue;
            }
            if char < 32 {
                char_x += 8;
                continue;
            }
            frame_buf.blit(
                memory.draw_colors,
                FONT,
                char_x,
                char_y,
                8,
                8,
                0,
                (char - 32) << 3,
                8,
                0,
            );
            char_x += 8;
        }
    }

    pub fn wasm4_text_utf8(
        &mut self,
        data: &mut [u8],
        text_ptr: u32,
        byte_len: u32,
        x: i32,
        y: i32,
    ) {
        let memory = Memory::from_bytes(data);
        let str_data: &[u8] = get_user_data(memory.user_data, text_ptr, byte_len);
        let mut frame_buf = FrameBuf {
            palette_raw: memory.palette,
            frame_buf:   memory.frame_buf,
        };
        let mut char_x = x;
        let mut char_y = y;
        for char in str_data {
            let char = *char as i32;
            if char == 0 {
                break;
            }
            // newline
            if char == 10 {
                char_y += 8;
                char_x = x;
                continue;
            }
            if char < 32 {
                char_x += 8;
                continue;
            }
            frame_buf.blit(
                memory.draw_colors,
                FONT,
                char_x,
                char_y,
                8,
                8,
                0,
                (char - 32) << 3,
                8,
                0,
            );
            char_x += 8;
        }
    }

    pub fn wasm4_text_utf16(
        &mut self,
        data: &mut [u8],
        text_ptr: u32,
        byte_len: u32,
        x: i32,
        y: i32,
    ) {
        self.wasm4_text_utf8(data, text_ptr, byte_len, x, y);
    }

    pub fn wasm4_tone(
        &mut self,
        data: &mut [u8],
        frequency: u32,
        duration: u32,
        volume: u32,
        flags: u32,
    ) {
        // ...
    }

    pub fn wasm4_diskr(&mut self, data: &mut [u8], dest: i32, size: u32) -> u32 {
        0
    }

    pub fn wasm4_diskw(&mut self, data: &mut [u8], src: i32, size: u32) -> u32 {
        0
    }

    pub fn wasm4_trace(&mut self, data: &mut [u8], str: i32) {
        // ...
    }

    pub fn wasm4_trace_utf8(&mut self, data: &mut [u8], str: i32, byte_len: u32) {
        // ...
    }

    pub fn wasm4_trace_utf16(&mut self, data: &mut [u8], str: i32, byte_len: u32) {
        // ...
    }
}

/// Write the given 32 bits at the beginning of the byte slice.
///
/// Uses little-endian encoding because wasm memory is little-endian.
fn write32le(target: &mut [u8], val: u32) {
    let val = val.to_le();
    target[3] = (val & 0x0000_00ff) as u8;
    target[2] = ((val & 0x0000_ff00) >> 8) as u8;
    target[1] = ((val & 0x00ff_0000) >> 16) as u8;
    target[0] = ((val & 0xff00_0000) >> 24) as u8;
}

/// Write the given 16 bits at the beginning of the byte slice.
///
/// Uses little-endian encoding because wasm memory is little-endian.
fn write16le(target: &mut [u8], val: u16) {
    let val = val.to_le();
    target[1] = (val & 0x00ff) as u8;
    target[0] = ((val & 0xff00) >> 8) as u8;
}

/// Make style for drawing primitives, use draw color 1 for stroke and color 2 for fill.
fn get_shape_style(draw_colors: &[u8]) -> PrimitiveStyle<Color4> {
    let mut style = PrimitiveStyle::new();
    if let Some(color) = get_draw_color(draw_colors, 1) {
        style.stroke_width = 1;
        style.stroke_color = Some(color);
    };
    style.fill_color = get_draw_color(draw_colors, 2);
    style
}

/// Given draw colors and draw color index, get the palette color.
fn get_draw_color(draw_colors: &[u8], idx: u8) -> Option<Color4> {
    let color = match idx {
        1 => draw_colors[0] & 0xf,
        2 => (draw_colors[0] >> 4) & 0xf,
        3 => draw_colors[1] & 0xf,
        4 => (draw_colors[1] >> 4) & 0xf,
        _ => unreachable!("bad draw color index: {}", idx),
    };
    assert!(color <= 4, "draw color has too high palette index");
    if color == 0 {
        return None;
    }
    Some(Color4(color - 1))
}

/// Safely read bytes from the user space of the wasm memory.
pub fn get_user_data(user_data: &[u8], ptr: u32, len: u32) -> &[u8] {
    let ptr = ptr as usize;
    if ptr < USER_DATA {
        return &[];
    }
    let start = ptr - USER_DATA;
    let end = usize::min(user_data.len(), start + len as usize);
    &user_data[start..end]
}
