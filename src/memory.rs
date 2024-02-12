use crate::consts::*;

pub struct Memory<'a> {
    pub palette:      &'a mut [u8],
    pub draw_colors:  &'a [u8],
    pub gamepads:     &'a mut [u8],
    pub system_flags: &'a [u8],
    pub frame_buf:    &'a mut [u8],
    pub user_data:    &'a mut [u8],
}

impl<'a> Memory<'a> {
    pub fn from_bytes(data: &'a mut [u8]) -> Memory<'a> {
        let ptr = data.as_mut_ptr();
        let raw = core::slice::from_raw_parts_mut;
        unsafe {
            Self {
                palette:      raw(ptr.add(PALETTE), 16),
                draw_colors:  raw(ptr.add(DRAW_COLORS), 2),
                gamepads:     raw(ptr.add(GAMEPAD1), 4),
                system_flags: raw(ptr.add(SYSTEM_FLAGS), 1),
                frame_buf:    raw(ptr.add(FRAMEBUFFER), 160 * 160 / 4),
                user_data:    raw(ptr.add(USER_DATA), data.len() - USER_DATA),
            }
        }
    }
}
