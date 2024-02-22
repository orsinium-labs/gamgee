use crate::consts::*;

pub fn init_memory(data: &mut [u8]) {
    // Init the default color palette
    write_color(&mut data[PALETTE..], 0xe0, 0xf8, 0xcf);
    write_color(&mut data[PALETTE + 4..], 0x86, 0xc0, 0x6c);
    write_color(&mut data[PALETTE + 8..], 0x30, 0x68, 0x50);
    write_color(&mut data[PALETTE + 12..], 0x07, 0x18, 0x21);

    write16le(&mut data[DRAW_COLORS..], 0x0312);
    // write32le(&mut data[MOUSE_X..], 0x7fff_7fff);

    // let frame_buf = FrameBuf::from_memory(data);
}

pub fn clear_frame_buffer(data: &mut [u8]) {
    // https://wasm4.org/docs/reference/memory#system_flags
    if data[SYSTEM_FLAGS] & SYSTEM_PRESERVE_FRAMEBUFFER != 0 {
        return;
    }
    #[allow(clippy::needless_range_loop)]
    for addr in 0x00a0..0x19a0 {
        data[addr] = 0;
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
