#![no_std]
#![no_main]

#[macro_use]
extern crate easy_microbit as m;

use m::{
    display::{self, DisplayImage},
    gpio,
};

start!(main);

fn main() -> ! {
    gpio::register_a_button_handler(handle_a_button);
    gpio::register_b_button_handler(handle_b_button);
    gpio::register_both_button_handler(handle_both_button);

    loop {}
}

fn handle_a_button() {
    display::display_image(A_BUTTON);
}

fn handle_b_button() {
    display::display_image(B_BUTTON);
}

fn handle_both_button() {
    display::display_image(BOTH_BUTTON);
}

const A_BUTTON: DisplayImage = DisplayImage([[true, false, false, false, false]; 5]);

const B_BUTTON: DisplayImage = DisplayImage([[false, false, false, false, true]; 5]);

const BOTH_BUTTON: DisplayImage = DisplayImage([[false, false, false, false, false]; 5]);
