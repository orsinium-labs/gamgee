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

    /// Initialize memory and stuff. Called before the application is started.
    pub fn init(&mut self, data: &mut [u8]) {
        // Init the default color palette
        write_color(&mut data[PALETTE..], 0xe0, 0xf8, 0xcf);
        write_color(&mut data[PALETTE + 4..], 0x86, 0xc0, 0x6c);
        write_color(&mut data[PALETTE + 8..], 0x30, 0x68, 0x50);
        write_color(&mut data[PALETTE + 12..], 0x07, 0x18, 0x21);

        write16le(&mut data[DRAW_COLORS..], 0x0312);
        // write32le(&mut data[MOUSE_X..], 0x7fff_7fff);

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

// Write u32 RGB color at the beginning of the given byte slice.
fn write_color(target: &mut [u8], r: u8, g: u8, b: u8) {
    target[3] = 0;
    target[2] = r;
    target[1] = g;
    target[0] = b;
}

/// Write the given 16 bits at the beginning of the byte slice.
///
/// Uses little-endian encoding because wasm memory is little-endian.
fn write16le(target: &mut [u8], val: u16) {
    let val = val.to_le();
    target[1] = (val & 0x00ff) as u8;
    target[0] = ((val & 0xff00) >> 8) as u8;
}
