#![no_std]
#![no_main]

#[macro_use]
extern crate easy_microbit as m;

use m::display::{
    self,
    animation::{Animate, Frame},
    DisplayImage,
};

start!(main);

struct SmileFrownFace(usize);

impl SmileFrownFace {
    fn new() -> Self {
        SmileFrownFace(0)
    }
}

impl Animate for SmileFrownFace {
    fn next_screen(&mut self) -> Option<Frame> {
        let frame = match self.0 {
            0 => Some(Frame {
                image: [
                    [false, true, false, true, false],
                    [false, false, false, false, false],
                    [true, false, false, false, true],
                    [false, true, true, true, false],
                    [false, false, false, false, false],
                ].into(),
                length: 300,
            }),
            1 => Some(Frame {
                image: [
                    [false, true, false, true, false],
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                    [true, true, true, true, true],
                    [false, false, false, false, false],
                ].into(),
                length: 300,
            }),
            2 => Some(Frame {
                image: [
                    [false, true, false, true, false],
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                    [false, true, true, true, false],
                    [true, false, false, false, true],
                ].into(),
                length: 300,
            }),
            3 => Some(Frame {
                image: [
                    [false, true, false, true, false],
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                    [true, true, true, true, true],
                    [false, false, false, false, false],
                ].into(),
                length: 300,
            }),
            _ => None,
        };

        self.0 = self.0 + 1;

        frame
    }

    fn frames(&self) -> u32 {
        4
    }
}

fn main() -> ! {
    loop {
        display::run_animation(SmileFrownFace::new(), true);
    }
}
