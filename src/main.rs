#![no_std]
#![no_main]

mod bridge;
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

    let instance_pre = linker.instantiate(&mut store, &module).unwrap();
    let instance = instance_pre.start(&mut store).unwrap();
    let update = instance.get_typed_func::<(), ()>(&store, "update").unwrap();
    loop {
        update.call(&mut store, ()).unwrap();
        store.data_mut().update();
    }
}
