extern crate alloc;
use crate::drawing_funcs::*;

type C<'a, T> = wasmi::Caller<'a, T>;

pub fn link<T>(
    linker: &mut wasmi::Linker<T>,
    memory: wasmi::Memory,
) -> Result<(), wasmi::errors::LinkerError> {
    linker.func_wrap(
        "env",
        "blit",
        move |mut caller: C<T>,
              sprite_ptr: u32,
              x: i32,
              y: i32,
              width: i32,
              height: i32,
              flags: u32| {
            let data = memory.data_mut(&mut caller);
            wasm4_blit(data, sprite_ptr, x, y, width, height, flags);
        },
    )?;
    linker.func_wrap(
        "env",
        "blitSub",
        move |mut caller: C<T>,
              sprite_ptr: u32,
              x: i32,
              y: i32,
              width: i32,
              height: i32,
              src_x: i32,
              src_y: i32,
              stride: i32,
              flags: u32| {
            let data = memory.data_mut(&mut caller);
            wasm4_blit_sub(
                data, sprite_ptr, x, y, width, height, src_x, src_y, stride, flags,
            )
        },
    )?;
    linker.func_wrap(
        "env",
        "line",
        move |mut caller: C<T>, x1: i32, y1: i32, x2: i32, y2: i32| {
            let data = memory.data_mut(&mut caller);
            wasm4_line(data, x1, y1, x2, y2)
        },
    )?;
    linker.func_wrap(
        "env",
        "hline",
        move |mut caller: C<T>, x: i32, y: i32, len: i32| {
            let data = memory.data_mut(&mut caller);
            wasm4_hline(data, x, y, len)
        },
    )?;
    linker.func_wrap(
        "env",
        "vline",
        move |mut caller: C<T>, x: i32, y: i32, len: i32| {
            let data = memory.data_mut(&mut caller);
            wasm4_vline(data, x, y, len)
        },
    )?;
    linker.func_wrap(
        "env",
        "oval",
        move |mut caller: C<T>, x: i32, y: i32, width: u32, height: u32| {
            let data = memory.data_mut(&mut caller);
            wasm4_oval(data, x, y, width, height)
        },
    )?;
    linker.func_wrap(
        "env",
        "rect",
        move |mut caller: C<T>, x: i32, y: i32, width: u32, height: u32| {
            let data = memory.data_mut(&mut caller);
            wasm4_rect(data, x, y, width, height)
        },
    )?;
    linker.func_wrap(
        "env",
        "text",
        move |mut caller: C<T>, text: u32, x: i32, y: i32| {
            let data = memory.data_mut(&mut caller);
            wasm4_text(data, text, x, y)
        },
    )?;
    linker.func_wrap(
        "env",
        "textUtf8",
        move |mut caller: C<T>, text_ptr: u32, byte_len: u32, x: i32, y: i32| {
            let data = memory.data_mut(&mut caller);
            wasm4_text_utf8(data, text_ptr, byte_len, x, y)
        },
    )?;
    linker.func_wrap(
        "env",
        "textUtf16",
        move |mut caller: C<T>, text_ptr: u32, byte_len: u32, x: i32, y: i32| {
            let data = memory.data_mut(&mut caller);
            wasm4_text_utf16(data, text_ptr, byte_len, x, y)
        },
    )?;
    Ok(())
}
