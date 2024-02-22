use embedded_graphics::geometry::Point;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use gamgee_core::*;
use pybadge_high::PyBadge;

pub struct Bridge {
    pybadge: PyBadge,
    frame:   usize,
}

impl Bridge {
    pub fn new(pybadge: PyBadge) -> Self {
        Self { pybadge, frame: 0 }
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
        clear_frame_buffer(data);
        self.update_gamepad(data);
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

    pub fn wasm4_tone(
        &mut self,
        _data: &mut [u8],
        _frequency: u32,
        _duration: u32,
        _volume: u32,
        _flags: u32,
    ) {
        // ...
    }

    pub fn wasm4_diskr(&mut self, _data: &mut [u8], _dest: i32, _size: u32) -> u32 {
        0
    }

    pub fn wasm4_diskw(&mut self, _data: &mut [u8], _src: i32, _size: u32) -> u32 {
        0
    }

    pub fn wasm4_trace(&mut self, _data: &mut [u8], _str: i32) {
        // ...
    }

    pub fn wasm4_trace_utf8(&mut self, _data: &mut [u8], _str: i32, _byte_len: u32) {
        // ...
    }

    pub fn wasm4_trace_utf16(&mut self, _data: &mut [u8], _str: i32, _byte_len: u32) {
        // ...
    }
}
