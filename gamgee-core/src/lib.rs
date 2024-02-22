#![no_std]
#![allow(clippy::too_many_arguments)]

mod consts;
mod drawing_funcs;
mod framebuf;
mod linking;
mod memory;

pub use consts::*;
pub use framebuf::{Color4, FrameBuf};
pub use linking::link;
pub use memory::Memory;
