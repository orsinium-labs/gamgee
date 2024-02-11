pub const SCREEN_WIDTH: u32 = 160;
pub const SCREEN_HEIGHT: u32 = 128;
pub const SCREEN_SIZE: u32 = 160;

pub const PALETTE: usize = 0x04;
pub const DRAW_COLORS: usize = 0x14;
pub const GAMEPAD1: usize = 0x16;
pub const GAMEPAD2: usize = 0x17;
pub const GAMEPAD3: usize = 0x18;
pub const GAMEPAD4: usize = 0x19;
// pub const MOUSE_X: usize = 0x1a;
// pub const MOUSE_Y: usize = 0x1c;
// pub const MOUSE_BUTTONS: usize = 0x1e;
pub const SYSTEM_FLAGS: usize = 0x1f;
pub const NETPLAY: usize = 0x20;
pub const FRAMEBUFFER: usize = 0xa0;

pub const BUTTON_1: u8 = 1;
pub const BUTTON_2: u8 = 2;
pub const BUTTON_LEFT: u8 = 16;
pub const BUTTON_RIGHT: u8 = 32;
pub const BUTTON_UP: u8 = 64;
pub const BUTTON_DOWN: u8 = 128;

pub const MOUSE_LEFT: u8 = 1;
pub const MOUSE_RIGHT: u8 = 2;
pub const MOUSE_MIDDLE: u8 = 4;

pub const SYSTEM_PRESERVE_FRAMEBUFFER: u8 = 1;
pub const SYSTEM_HIDE_GAMEPAD_OVERLAY: u8 = 2;

pub const BLIT_1BPP: u32 = 0;
pub const BLIT_2BPP: u32 = 1;
pub const BLIT_FLIP_X: u32 = 2;
pub const BLIT_FLIP_Y: u32 = 4;
pub const BLIT_ROTATE: u32 = 8;

pub const TONE_PULSE1: u32 = 0;
pub const TONE_PULSE2: u32 = 1;
pub const TONE_TRIANGLE: u32 = 2;
pub const TONE_NOISE: u32 = 3;
pub const TONE_MODE1: u32 = 0;
pub const TONE_MODE2: u32 = 4;
pub const TONE_MODE3: u32 = 8;
pub const TONE_MODE4: u32 = 12;
pub const TONE_PAN_LEFT: u32 = 16;
pub const TONE_PAN_RIGHT: u32 = 32;
