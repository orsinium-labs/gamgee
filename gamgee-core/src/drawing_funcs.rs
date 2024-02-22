use crate::consts::*;
use crate::framebuf::{Color4, FrameBuf};
use crate::memory::Memory;
use embedded_graphics::geometry::Point;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Ellipse, Line, PrimitiveStyle, Rectangle, StyledDrawable};

static FONT: &[u8; 1792] = include_bytes!("font.bin");

pub(crate) fn wasm4_blit(
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

pub(crate) fn wasm4_blit_sub(
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

pub(crate) fn wasm4_hline(data: &mut [u8], x: i32, y: i32, len: i32) {
    wasm4_line(data, x, y, x + len, y)
}

pub(crate) fn wasm4_vline(data: &mut [u8], x: i32, y: i32, len: i32) {
    wasm4_line(data, x, y, x, y + len)
}

pub(crate) fn wasm4_line(data: &mut [u8], x1: i32, y1: i32, x2: i32, y2: i32) {
    let line = Line::new(Point::new(x1, y1), Point::new(x2, y2));
    let mut memory = Memory::from_bytes(data);
    let Some(color) = get_draw_color(memory.draw_colors, 1) else {
        return;
    };
    let style = PrimitiveStyle::with_stroke(color, 1);
    let mut frame_buf = FrameBuf::from_memory(&mut memory);
    _ = line.draw_styled(&style, &mut frame_buf);
}

pub(crate) fn wasm4_oval(data: &mut [u8], x: i32, y: i32, width: u32, height: u32) {
    let ellipse = Ellipse::new(Point::new(x, y), Size::new(width, height));
    let mut memory = Memory::from_bytes(data);
    let style = get_shape_style(memory.draw_colors);
    let mut frame_buf = FrameBuf::from_memory(&mut memory);
    _ = ellipse.draw_styled(&style, &mut frame_buf);
}

pub(crate) fn wasm4_rect(data: &mut [u8], x: i32, y: i32, width: u32, height: u32) {
    let rect = Rectangle::new(Point::new(x, y), Size::new(width, height));
    let mut memory = Memory::from_bytes(data);
    // panic!(
    //     "{} {} {} {}",
    //     memory.palette[0], memory.palette[1], memory.palette[2], memory.palette[3]
    // );
    let style = get_shape_style(memory.draw_colors);
    let mut frame_buf = FrameBuf::from_memory(&mut memory);
    _ = rect.draw_styled(&style, &mut frame_buf);
}

pub(crate) fn wasm4_text(data: &mut [u8], text_ptr: u32, x: i32, y: i32) {
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

pub(crate) fn wasm4_text_utf8(data: &mut [u8], text_ptr: u32, byte_len: u32, x: i32, y: i32) {
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

pub(crate) fn wasm4_text_utf16(data: &mut [u8], text_ptr: u32, byte_len: u32, x: i32, y: i32) {
    wasm4_text_utf8(data, text_ptr, byte_len, x, y);
}

/// Make style for drawing primitives, use draw color 1 for stroke and color 2 for fill.
fn get_shape_style(draw_colors: &[u8]) -> PrimitiveStyle<Color4> {
    let mut style = PrimitiveStyle::new();
    style.fill_color = get_draw_color(draw_colors, 1);
    if let Some(color) = get_draw_color(draw_colors, 2) {
        style.stroke_width = 1;
        style.stroke_color = Some(color);
    };
    style
}

/// Given draw colors and draw color index, get the palette color.
fn get_draw_color(draw_colors: &[u8], idx: u8) -> Option<Color4> {
    assert!(idx == 1 || idx == 2);
    let mut color = draw_colors[0];
    if idx == 2 {
        color >>= 4;
    }
    color &= 0xf;
    assert!(color <= 4, "draw color has too high palette index");
    if color == 0 {
        return None;
    }
    Some(Color4(color - 1))
}

/// Safely read bytes from the user space of the wasm memory.
fn get_user_data(user_data: &[u8], ptr: u32, len: u32) -> &[u8] {
    let ptr = ptr as usize;
    if ptr < USER_DATA {
        return &[];
    }
    let start = ptr - USER_DATA;
    let end = usize::min(user_data.len(), start + len as usize);
    &user_data[start..end]
}
