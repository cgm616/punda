#![no_std]
#![no_main]

#[macro_use]
extern crate easy_microbit as m;

use m::display;

start!(main);

fn main() -> ! {
    display::display_image(display::constant::SMILEY);

    loop {}
}
