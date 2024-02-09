const SCREEN_SIZE: u32 = 160;

const PALETTE: usize = 0x04;
const DRAW_COLORS: usize = 0x14;
const GAMEPAD1: usize = 0x16;
const GAMEPAD2: usize = 0x17;
const GAMEPAD3: usize = 0x18;
const GAMEPAD4: usize = 0x19;
// const MOUSE_X: usize = 0x1a;
// const MOUSE_Y: usize = 0x1c;
// const MOUSE_BUTTONS: usize = 0x1e;
const SYSTEM_FLAGS: usize = 0x1f;
const NETPLAY: usize = 0x20;
const FRAMEBUFFER: usize = 0xa0;

const BUTTON_1: u8 = 1;
const BUTTON_2: u8 = 2;
const BUTTON_LEFT: u8 = 16;
const BUTTON_RIGHT: u8 = 32;
const BUTTON_UP: u8 = 64;
const BUTTON_DOWN: u8 = 128;

const MOUSE_LEFT: u8 = 1;
const MOUSE_RIGHT: u8 = 2;
const MOUSE_MIDDLE: u8 = 4;

const SYSTEM_PRESERVE_FRAMEBUFFER: u8 = 1;
const SYSTEM_HIDE_GAMEPAD_OVERLAY: u8 = 2;

const BLIT_1BPP: u32 = 0;
const BLIT_2BPP: u32 = 1;
const BLIT_FLIP_X: u32 = 2;
const BLIT_FLIP_Y: u32 = 4;
const BLIT_ROTATE: u32 = 8;

const TONE_PULSE1: u32 = 0;
const TONE_PULSE2: u32 = 1;
const TONE_TRIANGLE: u32 = 2;
const TONE_NOISE: u32 = 3;
const TONE_MODE1: u32 = 0;
const TONE_MODE2: u32 = 4;
const TONE_MODE3: u32 = 8;
const TONE_MODE4: u32 = 12;
const TONE_PAN_LEFT: u32 = 16;
const TONE_PAN_RIGHT: u32 = 32;
