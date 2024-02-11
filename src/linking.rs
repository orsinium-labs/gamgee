use crate::bridge::Bridge;
extern crate alloc;
use crate::framebuf::FrameBuf;

type C<'a> = wasmi::Caller<'a, Bridge>;

pub fn link(linker: &mut wasmi::Linker<Bridge>) -> Result<(), wasmi::errors::LinkerError> {
    linker.func_wrap("pybadge", "echo_i32", |mut caller: C, param: i32| {
        caller.data_mut().echo_i32(param)
    })?;
    linker.func_wrap(
        "env",
        "blit",
        |mut caller: C, sprite_ptr: i32, x: i32, y: i32, width: u32, height: u32, flags: u32| {
            caller
                .data_mut()
                .wasm4_blit(sprite_ptr, x, y, width, height, flags);
        },
    )?;
    linker.func_wrap(
        "env",
        "blitSub",
        |mut caller: C,
         sprite_ptr: i32,
         x: i32,
         y: i32,
         width: u32,
         height: u32,
         src_x: u32,
         src_y: u32,
         stride: i32,
         flags: u32| {
            caller
                .data_mut()
                .wasm4_blit_sub(sprite_ptr, x, y, width, height, src_x, src_y, stride, flags)
        },
    )?;
    linker.func_wrap(
        "env",
        "line",
        |mut caller: C, x1: i32, y1: i32, x2: i32, y2: i32| {
            caller.data_mut().wasm4_line(x1, y1, x2, y2)
        },
    )?;
    linker.func_wrap("env", "hline", |mut caller: C, x: i32, y: i32, len: u32| {
        let mem = match caller.get_export("memory") {
            Some(wasmi::Extern::Memory(mem)) => mem,
            _ => panic!("memory not found"),
        };
        let (data, bridge) = mem.data_and_store_mut(&mut caller);
        let frame_buf = FrameBuf::from_memory(data);
        bridge.wasm4_hline(frame_buf, x, y, len)
    })?;
    linker.func_wrap("env", "vline", |mut caller: C, x: i32, y: i32, len: u32| {
        caller.data_mut().wasm4_vline(x, y, len)
    })?;
    linker.func_wrap(
        "env",
        "oval",
        |mut caller: C, x: i32, y: i32, width: u32, height: u32| {
            caller.data_mut().wasm4_oval(x, y, width, height)
        },
    )?;
    linker.func_wrap(
        "env",
        "rect",
        |mut caller: C, x: i32, y: i32, width: u32, height: u32| {
            caller.data_mut().wasm4_rect(x, y, width, height)
        },
    )?;
    linker.func_wrap("env", "text", |mut caller: C, text: i32, x: i32, y: i32| {
        caller.data_mut().wasm4_text(text, x, y)
    })?;
    linker.func_wrap(
        "env",
        "textUtf8",
        |mut caller: C, text: i32, byte_len: u32, x: i32, y: i32| {
            caller.data_mut().wasm4_text_utf8(text, byte_len, x, y)
        },
    )?;
    linker.func_wrap(
        "env",
        "textUtf16",
        |mut caller: C, text: i32, byte_len: u32, x: i32, y: i32| {
            caller.data_mut().wasm4_text_utf16(text, byte_len, x, y)
        },
    )?;
    linker.func_wrap(
        "env",
        "tone",
        |mut caller: C, frequency: u32, duration: u32, volume: u32, flags: u32| {
            caller
                .data_mut()
                .wasm4_tone(frequency, duration, volume, flags)
        },
    )?;
    linker.func_wrap("env", "diskr", |mut caller: C, dest: i32, size: u32| {
        caller.data_mut().wasm4_diskr(dest, size)
    })?;
    linker.func_wrap("env", "diskw", |mut caller: C, src: i32, size: u32| {
        caller.data_mut().wasm4_diskw(src, size)
    })?;
    linker.func_wrap("env", "trace", |mut caller: C, str: i32| {
        caller.data_mut().wasm4_trace(str)
    })?;
    linker.func_wrap(
        "env",
        "traceUtf8",
        |mut caller: C, str: i32, byte_len: u32| caller.data_mut().wasm4_trace_utf8(str, byte_len),
    )?;
    linker.func_wrap(
        "env",
        "traceUtf16",
        |mut caller: C, str: i32, byte_len: u32| caller.data_mut().wasm4_trace_utf16(str, byte_len),
    )?;

    Ok(())
}
