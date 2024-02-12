use crate::bridge::Bridge;
extern crate alloc;

type C<'a> = wasmi::Caller<'a, Bridge>;

pub fn link(
    linker: &mut wasmi::Linker<Bridge>,
    memory: wasmi::Memory,
) -> Result<(), wasmi::errors::LinkerError> {
    linker.func_wrap(
        "env",
        "blit",
        move |mut caller: C,
              sprite_ptr: i32,
              x: i32,
              y: i32,
              width: i32,
              height: i32,
              flags: u32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_blit(data, sprite_ptr, x, y, width, height, flags);
        },
    )?;
    linker.func_wrap(
        "env",
        "blitSub",
        move |mut caller: C,
              sprite_ptr: i32,
              x: i32,
              y: i32,
              width: u32,
              height: u32,
              src_x: u32,
              src_y: u32,
              stride: i32,
              flags: u32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_blit_sub(
                data, sprite_ptr, x, y, width, height, src_x, src_y, stride, flags,
            )
        },
    )?;
    linker.func_wrap(
        "env",
        "line",
        move |mut caller: C, x1: i32, y1: i32, x2: i32, y2: i32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_line(data, x1, y1, x2, y2)
        },
    )?;
    linker.func_wrap(
        "env",
        "hline",
        move |mut caller: C, x: i32, y: i32, len: i32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_hline(data, x, y, len)
        },
    )?;
    linker.func_wrap(
        "env",
        "vline",
        move |mut caller: C, x: i32, y: i32, len: i32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_vline(data, x, y, len)
        },
    )?;
    linker.func_wrap(
        "env",
        "oval",
        move |mut caller: C, x: i32, y: i32, width: u32, height: u32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_oval(data, x, y, width, height)
        },
    )?;
    linker.func_wrap(
        "env",
        "rect",
        move |mut caller: C, x: i32, y: i32, width: u32, height: u32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_rect(data, x, y, width, height)
        },
    )?;
    linker.func_wrap(
        "env",
        "text",
        move |mut caller: C, text: i32, x: i32, y: i32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_text(data, text, x, y)
        },
    )?;
    linker.func_wrap(
        "env",
        "textUtf8",
        move |mut caller: C, text: i32, byte_len: u32, x: i32, y: i32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_text_utf8(data, text, byte_len, x, y)
        },
    )?;
    linker.func_wrap(
        "env",
        "textUtf16",
        move |mut caller: C, text: i32, byte_len: u32, x: i32, y: i32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_text_utf16(data, text, byte_len, x, y)
        },
    )?;
    linker.func_wrap(
        "env",
        "tone",
        move |mut caller: C, frequency: u32, duration: u32, volume: u32, flags: u32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_tone(data, frequency, duration, volume, flags)
        },
    )?;
    linker.func_wrap(
        "env",
        "diskr",
        move |mut caller: C, dest: i32, size: u32| -> u32 {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_diskr(data, dest, size)
        },
    )?;
    linker.func_wrap(
        "env",
        "diskw",
        move |mut caller: C, src: i32, size: u32| -> u32 {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_diskw(data, src, size)
        },
    )?;
    linker.func_wrap("env", "trace", move |mut caller: C, str: i32| {
        let (data, bridge) = memory.data_and_store_mut(&mut caller);
        bridge.wasm4_trace(data, str)
    })?;
    linker.func_wrap(
        "env",
        "traceUtf8",
        move |mut caller: C, str: i32, byte_len: u32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_trace_utf8(data, str, byte_len)
        },
    )?;
    linker.func_wrap(
        "env",
        "traceUtf16",
        move |mut caller: C, str: i32, byte_len: u32| {
            let (data, bridge) = memory.data_and_store_mut(&mut caller);
            bridge.wasm4_trace_utf16(data, str, byte_len)
        },
    )?;

    Ok(())
}
