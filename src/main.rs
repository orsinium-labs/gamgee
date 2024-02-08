#![no_std]
#![no_main]

use pybadge_high::{prelude::*, PyBadge};

#[entry]
fn main() -> ! {
    let mut pybadge = PyBadge::take().unwrap();
    loop {}
}
