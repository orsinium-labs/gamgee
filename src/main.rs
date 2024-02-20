#![no_std]
#![no_main]
#![allow(clippy::too_many_arguments)]
extern crate alloc;

mod bridge;
mod consts;
mod framebuf;
mod linking;
mod memory;

use bridge::Bridge;
use embedded_alloc::Heap;
use embedded_graphics::prelude::*;
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
        const HEAP_SIZE: usize = 185 * 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let mut pybadge = PyBadge::take().unwrap();
    pybadge.display.clear(Color::RED).unwrap();
    let engine = wasmi::Engine::default();
    let bytes = include_bytes!(env!("GAME_PATH"));
    let module = wasmi::Module::new(&engine, &bytes[..]).unwrap();
    let bridge = Bridge::new(pybadge);
    let mut store = <wasmi::Store<Bridge>>::new(&engine, bridge);
    let mut linker = <wasmi::Linker<Bridge>>::new(&engine);

    let mem_type = wasmi::MemoryType::new(1, Some(1)).unwrap();
    let mem = wasmi::Memory::new(&mut store, mem_type).unwrap();
    let memory = mem;
    linker.define("env", "memory", mem).unwrap();
    link(&mut linker, memory).unwrap();

    let instance_pre = linker.instantiate(&mut store, &module).unwrap();
    let instance = instance_pre.start(&mut store).unwrap();
    // store.data_mut().set_memory(memory, &mut store);

    {
        let (data, bridge) = memory.data_and_store_mut(&mut store);
        bridge.init(memory, data);
    }
    // wasi p1
    if let Ok(start) = instance.get_typed_func::<(), ()>(&store, "_initialize") {
        start.call(&mut store, ()).unwrap();
    }
    if let Ok(start) = instance.get_typed_func::<(), ()>(&store, "_start") {
        start.call(&mut store, ()).unwrap();
    }
    if let Ok(start) = instance.get_typed_func::<(), ()>(&store, "start") {
        start.call(&mut store, ()).unwrap();
    }

    let update = instance.get_typed_func::<(), ()>(&store, "update").unwrap();
    loop {
        update.call(&mut store, ()).unwrap();
        let (data, bridge) = memory.data_and_store_mut(&mut store);
        bridge.update(data);
    }
}
