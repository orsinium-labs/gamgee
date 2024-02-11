#![no_std]
#![no_main]

extern crate alloc;

mod bridge;
mod consts;
mod framebuf;
mod linking;

use bridge::Bridge;
use embedded_alloc::Heap;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use framebuf::FrameBuf;
use linking::link;
use pybadge::prelude::entry;
use pybadge::{Color, PyBadge};
use pybadge_high as pybadge;

#[global_allocator]
static HEAP: Heap = Heap::empty();

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
    let bytes = include_bytes!("../snake/build/cart.wasm");
    let module = wasmi::Module::new(&engine, &bytes[..]).unwrap();
    let bridge = Bridge::new(pybadge);
    let mut store = <wasmi::Store<Bridge>>::new(&engine, bridge);
    let mut linker = <wasmi::Linker<Bridge>>::new(&engine);
    link(&mut linker).unwrap();

    let mem_type = wasmi::MemoryType::new(1, Some(1)).unwrap();
    let mem = wasmi::Memory::new(&mut store, mem_type).unwrap();
    linker.define("env", "memory", mem).unwrap();

    let instance_pre = linker.instantiate(&mut store, &module).unwrap();
    let instance = instance_pre.start(&mut store).unwrap();
    // store.data_mut().set_memory(memory, &mut store);

    let memory = match linker.get(&mut store, "env", "memory") {
        Some(wasmi::Extern::Memory(memory)) => memory,
        _ => panic!("memory not found"),
    };
    let (data, bridge) = memory.data_and_store_mut(&mut store);
    let frame_buf = FrameBuf::from_memory(data);
    bridge.start(frame_buf);
    if let Ok(start) = instance.get_typed_func::<(), ()>(&store, "start") {
        start.call(&mut store, ()).unwrap();
    }

    let update = instance.get_typed_func::<(), ()>(&store, "update").unwrap();
    loop {
        update.call(&mut store, ()).unwrap();
        let memory = match linker.get(&mut store, "env", "memory") {
            Some(wasmi::Extern::Memory(memory)) => memory,
            _ => panic!("memory not found"),
        };
        let (data, bridge) = memory.data_and_store_mut(&mut store);
        let frame_buf = FrameBuf::from_memory(data);
        bridge.update(frame_buf);
    }
}
