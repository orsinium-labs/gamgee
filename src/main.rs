#![no_std]
#![no_main]

mod bridge;
mod consts;
use bridge::Bridge;
extern crate alloc;
use embedded_alloc::Heap;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use pybadge::prelude::entry;
use pybadge::{Color, PyBadge};
use pybadge_high as pybadge;

#[global_allocator]
static HEAP: Heap = Heap::empty();

type C<'a> = wasmi::Caller<'a, Bridge>;

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 128 * 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let mut pybadge = PyBadge::take().unwrap();
    pybadge.display.clear(Color::RED).unwrap();
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    Text::new("Hello Rust!", Point::new(20, 30), style)
        .draw(&mut pybadge.display)
        .unwrap();
    let engine = wasmi::Engine::default();
    let bytes = include_bytes!("../demo.wasm");
    let module = wasmi::Module::new(&engine, &bytes[..]).unwrap();
    let bridge = Bridge::new(pybadge);
    let mut store = <wasmi::Store<Bridge>>::new(&engine, bridge);
    let mut linker = <wasmi::Linker<Bridge>>::new(&engine);

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
        caller.data_mut().wasm4_hline(x, y, len)
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
    linker.define("env", "textUtf16", text_utf16).unwrap();
    linker.define("env", "tone", tone).unwrap();
    linker.define("env", "diskr", diskr).unwrap();
    linker.define("env", "diskw", diskw).unwrap();
    linker.define("env", "traceUtf16", trace_utf16).unwrap();

    let mem_type = wasmi::MemoryType::new(1, Some(1)).unwrap();
    let mem = wasmi::Memory::new(&mut store, mem_type).unwrap();
    linker.define("env", "memory", mem).unwrap();

    let instance_pre = linker.instantiate(&mut store, &module).unwrap();
    let instance = instance_pre.start(&mut store).unwrap();
    let memory = instance.get_memory(&store, "memory");
    store.data_mut().set_memory(memory);
    if let Ok(start) = instance.get_typed_func::<(), ()>(&store, "start") {
        start.call(&mut store, ()).unwrap();
    }
    let update = instance.get_typed_func::<(), ()>(&store, "update").unwrap();
    loop {
        update.call(&mut store, ()).unwrap();
        store.data_mut().update();
    }
}
