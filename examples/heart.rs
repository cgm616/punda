#![no_std]
#![no_main]

#[macro_use]
extern crate easy_microbit as m;

use m::display::{self, animation::heart::Heart};

start!(main);

fn main() -> ! {
    loop {
        display::run_animation(Heart::new(), true);
    }
}
