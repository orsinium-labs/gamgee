use alloc::string::ToString;
use embedded_graphics::geometry::Point;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use pybadge_high::{Color, PyBadge};

pub struct Bridge {
    command: i32,
    pybadge: PyBadge,
}

impl Bridge {
    pub fn new(pybadge: PyBadge) -> Self {
        Self {
            command: 0,
            pybadge,
        }
    }

    pub fn update(&mut self) {
        let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
        self.pybadge.display.clear(Color::BLUE).unwrap();
        let text = self.command.to_string();
        let pos = Point::new(20, 30);
        Text::new(&text, pos, style)
            .draw(&mut self.pybadge.display)
            .unwrap();
    }

    pub fn echo_i32(&mut self, param: i32) {
        self.command = param
    }
}
