#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::ToString;
use core::cell::Cell;
use embedded_alloc::Heap;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use pybadge::{prelude::*, *};
use pybadge_high as pybadge;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub struct InnerState {
    pub command: i32,
}

impl InnerState {
    fn get_command(&self) -> i32 {
        self.command
    }
}

struct HostState {
    state: Cell<InnerState>,
}

impl HostState {
    fn set_command(&mut self, param: i32) {
        self.state.get_mut().command = param
    }
}

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 16 * 1024;
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
    let state = Cell::new(InnerState { command: 0 });
    let inner_state = state.as_ptr();
    let mut store = <wasmi::Store<HostState>>::new(&engine, HostState { state });
    let echo_i32 = wasmi::Func::wrap(
        &mut store,
        |mut caller: wasmi::Caller<'_, HostState>, param: i32| {
            caller.data_mut().set_command(param);
        },
    );
    let mut linker = <wasmi::Linker<HostState>>::new(&engine);
    linker.define("pybadge", "echo_i32", echo_i32).unwrap();
    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    let update = instance.get_typed_func::<(), ()>(&store, "update").unwrap();
    loop {
        update.call(&mut store, ()).unwrap();
        let state;
        unsafe {
            state = inner_state.as_ref().unwrap();
        }
        pybadge.display.clear(Color::BLUE).unwrap();
        Text::new(&state.get_command().to_string(), Point::new(20, 30), style)
            .draw(&mut pybadge.display)
            .unwrap();
    }
}
