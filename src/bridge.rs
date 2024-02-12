use crate::consts::*;
use crate::framebuf::{Color4, FrameBuf};
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Ellipse, Line, PrimitiveStyle, Rectangle, StyledDrawable};
use embedded_graphics::text::Text;
use pybadge_high::PyBadge;

pub struct Bridge {
    pybadge: PyBadge,
    memory:  Option<wasmi::Memory>,
}

impl Bridge {
    pub fn new(pybadge: PyBadge) -> Self {
        Self {
            memory: None,
            pybadge,
        }
    }

    /// Initialize memory and stuff. Called before the application is started.
    pub fn init(&mut self, memory: wasmi::Memory, data: &mut [u8]) {
        self.memory = Some(memory);
        // init the color palette

        // ugly colors for better debugging
        write32le(&mut data[PALETTE..], 0xffffff);
        write32le(&mut data[PALETTE + 4..], 0x0000ff);
        write32le(&mut data[PALETTE + 8..], 0xff0000);
        write32le(&mut data[PALETTE + 12..], 0x00ff00);

        // The original default palette
        // write32le(&mut data[PALETTE..], 0xe0f8cf);
        // write32le(&mut data[PALETTE + 4..], 0x86c06c);
        // write32le(&mut data[PALETTE + 8..], 0x306850);
        // write32le(&mut data[PALETTE + 12..], 0x071821);

        // https://lospec.com/palette-list/ice-cream-gb
        // write32le(&mut data[PALETTE..], 0xfff6d3);
        // write32le(&mut data[PALETTE + 4..], 0xf9a875);
        // write32le(&mut data[PALETTE + 8..], 0xeb6b6f);
        // write32le(&mut data[PALETTE + 12..], 0x7c3f58);

        write16le(&mut data[DRAW_COLORS..], 0x0312);
        write32le(&mut data[MOUSE_X..], 0x7fff_7fff);

        // let frame_buf = FrameBuf::from_memory(data);
    }

    pub fn update(&mut self, data: &mut [u8]) {
        let frame_buf = FrameBuf::from_memory(data);
        // let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
        // self.pybadge.display.clear(Color::BLUE).unwrap();
        // let text = self.command.to_string();
        // let pos = Point::new(20, 30);
        // Text::new(&text, pos, style)
        //     .draw(&mut self.pybadge.display)
        //     .unwrap();
        // self.pybadge.display.write_pixels(frame_buf.iter());
        self.pybadge.display.draw_iter(frame_buf.iter()).unwrap();
        self.clear_frame_buffer(data);
        self.update_gamepad(data);
    }

    fn clear_frame_buffer(&self, data: &mut [u8]) {
        // https://wasm4.org/docs/reference/memory#system_flags
        if data[SYSTEM_FLAGS] & 0b1 == 1 {
            return;
        }
        #[allow(clippy::needless_range_loop)]
        for addr in 0x00a0..0x19a0 {
            data[addr] = 0;
        }
    }

    fn update_gamepad(&self, data: &mut [u8]) {
        // https://wasm4.org/docs/reference/memory#gamepads
        data[GAMEPAD1] = 0;
        if self.pybadge.buttons.a_pressed() {
            data[GAMEPAD1] |= 0b10;
        }
        if self.pybadge.buttons.b_pressed() {
            data[GAMEPAD1] |= 0b01;
        }
        if self.pybadge.buttons.left_pressed() {
            data[GAMEPAD1] |= 0b1 << 4;
        }
        if self.pybadge.buttons.right_pressed() {
            data[GAMEPAD1] |= 0b1 << 5;
        }
        if self.pybadge.buttons.up_pressed() {
            data[GAMEPAD1] |= 0b1 << 6;
        }
        if self.pybadge.buttons.down_pressed() {
            data[GAMEPAD1] |= 0b1 << 7;
        }
    }

    pub fn wasm4_blit(
        &mut self,
        sprite_ptr: i32,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        flags: u32,
    ) {
        // ...
    }

    pub fn wasm4_blit_sub(
        &self,
        sprite_ptr: i32,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        src_x: u32,
        src_y: u32,
        stride: i32,
        flags: u32,
    ) {
        // ...
    }

    pub fn wasm4_hline(&mut self, data: &mut [u8], x: i32, y: i32, len: i32) {
        self.wasm4_line(data, x, y, x + len, y)
    }

    pub fn wasm4_vline(&mut self, data: &mut [u8], x: i32, y: i32, len: i32) {
        self.wasm4_line(data, x, y, x, y + len)
    }

    pub fn wasm4_line(&mut self, data: &mut [u8], x1: i32, y1: i32, x2: i32, y2: i32) {
        let line = Line::new(Point::new(x1, y1), Point::new(x2, y2));
        let Some(color) = get_draw_color(data, 1) else {
            return;
        };
        let style = PrimitiveStyle::with_stroke(color, 1);
        let mut frame_buf = FrameBuf::from_memory(data);
        line.draw_styled(&style, &mut frame_buf).unwrap();
    }

    pub fn wasm4_oval(&mut self, data: &mut [u8], x: i32, y: i32, width: u32, height: u32) {
        let ellipse = Ellipse::new(Point::new(x, y), Size::new(width, height));
        let style = get_shape_style(data);
        let mut frame_buf = FrameBuf::from_memory(data);
        ellipse.draw_styled(&style, &mut frame_buf).unwrap();
    }

    pub fn wasm4_rect(&mut self, data: &mut [u8], x: i32, y: i32, width: u32, height: u32) {
        let rect = Rectangle::new(Point::new(x, y), Size::new(width, height));
        let style = get_shape_style(data);
        let mut frame_buf = FrameBuf::from_memory(data);
        rect.draw_styled(&style, &mut frame_buf).unwrap();
    }

    pub fn wasm4_text(&mut self, data: &mut [u8], text_ptr: i32, x: i32, y: i32) {
        // We need unsafe because we want to use data later as mutable as well
        // to actually draw the text onto the frame buffer.
        // It is safe to do so because the ranges don't intersect.

        // TODO: Specify the correct end range!
        let str_data: &[u8] = unsafe {
            let start_ptr = data.as_ptr().add(text_ptr as usize);
            core::slice::from_raw_parts(start_ptr, 1024)
        };
        let c_str = core::ffi::CStr::from_bytes_until_nul(str_data).unwrap();
        let str = c_str.to_str().unwrap();
        self.write_text(data, str, x, y)
    }

    pub fn wasm4_text_utf8(&mut self, data: &mut [u8], text: i32, byte_len: u32, x: i32, y: i32) {
        // ...
    }

    pub fn wasm4_text_utf16(&mut self, data: &mut [u8], text: i32, byte_len: u32, x: i32, y: i32) {
        // ...
    }

    pub fn write_text(&mut self, data: &mut [u8], text: &str, x: i32, y: i32) {
        let Some(color) = get_draw_color(data, 1) else {
            return;
        };
        let style = MonoTextStyle::new(&FONT_6X10, color);
        let position = Point::new(x, y);
        let text = Text::new(text, position, style);
        let mut frame_buf = FrameBuf::from_memory(data);
        text.draw(&mut frame_buf).unwrap();
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

    pub fn wasm4_diskr(&mut self, data: &mut [u8], dest: i32, size: u32) -> i32 {
        0
    }

    pub fn wasm4_diskw(&mut self, data: &mut [u8], src: i32, size: u32) -> i32 {
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

fn write32le(target: &mut [u8], val: u32) {
    let val = val.to_le();
    target[3] = (val & 0x0000_00ff) as u8;
    target[2] = ((val & 0x0000_ff00) >> 8) as u8;
    target[1] = ((val & 0x00ff_0000) >> 16) as u8;
    target[0] = ((val & 0xff00_0000) >> 24) as u8;
}

fn write16le(target: &mut [u8], val: u16) {
    let val = val.to_le();
    target[1] = (val & 0x00ff) as u8;
    target[0] = ((val & 0xff00) >> 8) as u8;
}

fn get_shape_style(data: &mut [u8]) -> PrimitiveStyle<Color4> {
    let mut style = PrimitiveStyle::new();
    if let Some(color) = get_draw_color(data, 1) {
        style.stroke_width = 1;
        style.stroke_color = Some(color);
    };
    style.fill_color = get_draw_color(data, 2);
    style
}

fn get_draw_color(data: &mut [u8], idx: u8) -> Option<Color4> {
    let color = match idx {
        1 => data[DRAW_COLORS] & 0xf,
        2 => (data[DRAW_COLORS] >> 4) & 0xf,
        3 => data[DRAW_COLORS + 1] & 0xf,
        4 => (data[DRAW_COLORS + 1] >> 4) & 0xf,
        _ => unreachable!("bad draw color index: {}", idx),
    };
    assert!(color <= 4);
    if color == 0 {
        return None;
    }
    Some(Color4(color - 1))
}
