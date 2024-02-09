use alloc::string::ToString;
use embedded_graphics::geometry::Point;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use pybadge_high::{Color, PyBadge};

pub struct Bridge {
    command: i32,
    pybadge: PyBadge,
    memory: Option<wasmi::Memory>,
}

impl Bridge {
    pub fn new(pybadge: PyBadge) -> Self {
        Self {
            command: 0,
            pybadge,
            memory: None,
        }
    }

    pub fn set_memory(&mut self, memory: Option<wasmi::Memory>) {
        self.memory = memory;
    }

    pub fn start(&mut self) {
        // 0xE0, 0xF8, 0xCF
    }

    pub fn update(&mut self) {
        let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
        self.pybadge.display.clear(Color::BLUE).unwrap();
        let text = self.command.to_string();
        let pos = Point::new(20, 30);
        Text::new(&text, pos, style)
            .draw(&mut self.pybadge.display)
            .unwrap();
    }

    pub fn echo_i32(&mut self, param: i32) {
        self.command = param
    }

    pub fn wasm4_blit(&self, spritePtr: i32, x: i32, y: i32, width: u32, height: u32, flags: u32) {
        todo!();
    }

    pub fn wasm4_blit_sub(
        &self,
        spritePtr: i32,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        srcX: u32,
        srcY: u32,
        stride: i32,
        flags: u32,
    ) {
        todo!();
    }

    pub fn wasm4_line(&self, x1: i32, y1: i32, x2: i32, y2: i32) {
        todo!();
    }

    pub fn wasm4_hline(&self, x: i32, y: i32, len: u32) {
        todo!();
    }

    pub fn wasm4_vline(&self, x: i32, y: i32, len: u32) {
        todo!();
    }

    pub fn wasm4_oval(&self, x: i32, y: i32, width: u32, height: u32) {
        todo!();
    }

    pub fn wasm4_rect(&self, x: i32, y: i32, width: u32, height: u32) {
        todo!();
    }

    pub fn wasm4_text_utf16(&self, text: i32, byte_len: u32, x: i32, y: i32) {
        todo!();
    }

    pub fn wasm4_tone(&self, frequency: u32, duration: u32, volume: u32, flags: u32) {
        todo!();
    }

    pub fn wasm4_diskr(&self, dest: i32, size: u32) -> i32 {
        todo!();
    }

    pub fn wasm4_diskw(&self, src: i32, size: u32) -> i32 {
        todo!();
    }

    pub fn wasm4_trace_utf16(&self, str: i32, byte_len: u32) {
        todo!();
    }
}
