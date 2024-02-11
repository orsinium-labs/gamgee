use crate::bridge::Bridge;
extern crate alloc;
use crate::framebuf::FrameBuf;

type C<'a> = wasmi::Caller<'a, Bridge>;

pub fn link(linker: &mut wasmi::Linker<Bridge>, mut store: &mut wasmi::Store<Bridge>) {
    let echo_i32 = wasmi::Func::wrap(&mut store, |mut caller: C, param: i32| {
        caller.data_mut().echo_i32(param);
    });
    linker.define("pybadge", "echo_i32", echo_i32).unwrap();

    let blit = wasmi::Func::wrap(
        &mut store,
        |mut caller: C, sprite_ptr: i32, x: i32, y: i32, width: u32, height: u32, flags: u32| {
            caller
                .data_mut()
                .wasm4_blit(sprite_ptr, x, y, width, height, flags);
        },
    );
    let blit_sub = wasmi::Func::wrap(
        &mut store,
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
    );
    let line = wasmi::Func::wrap(
        &mut store,
        |mut caller: C, x1: i32, y1: i32, x2: i32, y2: i32| {
            caller.data_mut().wasm4_line(x1, y1, x2, y2)
        },
    );
    let hline = wasmi::Func::wrap(&mut store, |mut caller: C, x: i32, y: i32, len: u32| {
        let mem = match caller.get_export("memory") {
            Some(wasmi::Extern::Memory(mem)) => mem,
            _ => panic!("memory not found"),
        };
        let (data, bridge) = mem.data_and_store_mut(&mut caller);
        let frame_buf = FrameBuf::from_memory(data);
        bridge.wasm4_hline(frame_buf, x, y, len)
    });
    let vline = wasmi::Func::wrap(&mut store, |mut caller: C, x: i32, y: i32, len: u32| {
        caller.data_mut().wasm4_vline(x, y, len)
    });
    let oval = wasmi::Func::wrap(
        &mut store,
        |mut caller: C, x: i32, y: i32, width: u32, height: u32| {
            caller.data_mut().wasm4_oval(x, y, width, height)
        },
    );
    let rect = wasmi::Func::wrap(
        &mut store,
        |mut caller: C, x: i32, y: i32, width: u32, height: u32| {
            caller.data_mut().wasm4_rect(x, y, width, height)
        },
    );
    let text = wasmi::Func::wrap(&mut store, |mut caller: C, text: i32, x: i32, y: i32| {
        caller.data_mut().wasm4_text(text, x, y)
    });
    let text_utf8 = wasmi::Func::wrap(
        &mut store,
        |mut caller: C, text: i32, byte_len: u32, x: i32, y: i32| {
            caller.data_mut().wasm4_text_utf8(text, byte_len, x, y)
        },
    );
    let text_utf16 = wasmi::Func::wrap(
        &mut store,
        |mut caller: C, text: i32, byte_len: u32, x: i32, y: i32| {
            caller.data_mut().wasm4_text_utf16(text, byte_len, x, y)
        },
    );
    let tone = wasmi::Func::wrap(
        &mut store,
        |mut caller: C, frequency: u32, duration: u32, volume: u32, flags: u32| {
            caller
                .data_mut()
                .wasm4_tone(frequency, duration, volume, flags)
        },
    );
    let diskr = wasmi::Func::wrap(&mut store, |mut caller: C, dest: i32, size: u32| {
        caller.data_mut().wasm4_diskr(dest, size)
    });
    let diskw = wasmi::Func::wrap(&mut store, |mut caller: C, src: i32, size: u32| {
        caller.data_mut().wasm4_diskw(src, size)
    });
    let trace = wasmi::Func::wrap(&mut store, |mut caller: C, str: i32| {
        caller.data_mut().wasm4_trace(str)
    });
    let trace_utf8 = wasmi::Func::wrap(&mut store, |mut caller: C, str: i32, byte_len: u32| {
        caller.data_mut().wasm4_trace_utf8(str, byte_len)
    });
    let trace_utf16 = wasmi::Func::wrap(&mut store, |mut caller: C, str: i32, byte_len: u32| {
        caller.data_mut().wasm4_trace_utf16(str, byte_len)
    });

    linker.define("env", "blit", blit).unwrap();
    linker.define("env", "blitSub", blit_sub).unwrap();
    linker.define("env", "line", line).unwrap();
    linker.define("env", "hline", hline).unwrap();
    linker.define("env", "vline", vline).unwrap();
    linker.define("env", "oval", oval).unwrap();
    linker.define("env", "rect", rect).unwrap();
    linker.define("env", "text", text).unwrap();
    linker.define("env", "textUtf8", text_utf8).unwrap();
    linker.define("env", "textUtf16", text_utf16).unwrap();
    linker.define("env", "tone", tone).unwrap();
    linker.define("env", "diskr", diskr).unwrap();
    linker.define("env", "diskw", diskw).unwrap();
    linker.define("env", "trace", trace).unwrap();
    linker.define("env", "traceUtf8", trace_utf8).unwrap();
    linker.define("env", "traceUtf16", trace_utf16).unwrap();
}
