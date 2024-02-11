use crate::consts::*;
use crate::framebuf::FrameBuf;
use alloc::string::ToString;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use pybadge_high::{Color, PyBadge};

pub struct Bridge {
    command: i32,
    pybadge: PyBadge,
    memory:  Option<wasmi::Memory>,
}

impl Bridge {
    pub fn new(pybadge: PyBadge) -> Self {
        Self {
            command: 0,
            memory: None,
            pybadge,
        }
    }

    /// Initialize memory and stuff. Called before the application is started.
    pub fn init(&mut self, memory: wasmi::Memory, data: &mut [u8]) {
        self.memory = Some(memory);
        // init the palette
        write32le(&mut data[PALETTE..], 0xe0f8cf);
        write32le(&mut data[PALETTE + 4..], 0x86c06c);
        write32le(&mut data[PALETTE + 8..], 0x306850);
        write32le(&mut data[PALETTE + 12..], 0x071821);
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
        self.clear_frame_buffer(data)
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

    pub fn echo_i32(&mut self, param: i32) {
        self.command = param
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

    pub fn wasm4_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        // ...
    }

    pub fn wasm4_hline(&mut self, frame_buf: FrameBuf, x: i32, y: i32, len: u32) {
        // ...
    }

    pub fn wasm4_vline(&mut self, x: i32, y: i32, len: u32) {
        // ...
    }

    pub fn wasm4_oval(&mut self, x: i32, y: i32, width: u32, height: u32) {
        // ...
    }

    pub fn wasm4_rect(&mut self, x: i32, y: i32, width: u32, height: u32) {
        // ...
    }

    pub fn wasm4_text(&mut self, text: i32, x: i32, y: i32) {
        // ...
    }

    pub fn wasm4_text_utf8(&mut self, text: i32, byte_len: u32, x: i32, y: i32) {
        // ...
    }

    pub fn wasm4_text_utf16(&mut self, text: i32, byte_len: u32, x: i32, y: i32) {
        // ...
    }

    pub fn wasm4_tone(&mut self, frequency: u32, duration: u32, volume: u32, flags: u32) {
        // ...
    }

    pub fn wasm4_diskr(&mut self, dest: i32, size: u32) -> i32 {
        0
    }

    pub fn wasm4_diskw(&mut self, src: i32, size: u32) -> i32 {
        0
    }

    pub fn wasm4_trace(&mut self, str: i32) {
        // ...
    }

    pub fn wasm4_trace_utf8(&mut self, str: i32, byte_len: u32) {
        // ...
    }

    pub fn wasm4_trace_utf16(&mut self, str: i32, byte_len: u32) {
        // ...
    }
}

fn write32le(target: &mut [u8], val: u32) {
    let val = val.to_le();
    target[0] = (val & 0x0000_00ff) as u8;
    target[1] = (val & 0x0000_ff00) as u8;
    target[2] = (val & 0x00ff_0000) as u8;
    target[3] = (val & 0xff00_0000) as u8;
}
