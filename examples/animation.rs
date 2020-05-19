#![no_std]
#![no_main]

use punda::{
    delay,
    display::{self, image::GreyscaleImage},
};

punda::punda!(init: init, idle: idle);

fn init(cx: &mut punda::context::UserContext) {}

fn idle(cx: &mut punda::context::UserContext) -> ! {
    loop {
        display::show(cx, &BLANK);

        delay::delay_ms(cx, 300);

        display::show(cx, &PIN);

        delay::delay_ms(cx, 100);

        display::show(cx, &MED);

        delay::delay_ms(cx, 200);

        display::show(cx, &LARGE);

        delay::delay_ms(cx, 600);

        display::show(cx, &MED);

        delay::delay_ms(cx, 200);

        display::show(cx, &PIN);

        delay::delay_ms(cx, 100);
    }
}

const BLANK: GreyscaleImage = GreyscaleImage::blank();

const PIN: GreyscaleImage = GreyscaleImage::new(&[
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 9, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
]);

const MED: GreyscaleImage = GreyscaleImage::new(&[
    [0, 0, 0, 0, 0],
    [0, 9, 0, 9, 0],
    [0, 9, 9, 9, 0],
    [0, 0, 9, 0, 0],
    [0, 0, 0, 0, 0],
]);

const LARGE: GreyscaleImage = GreyscaleImage::new(&[
    [0, 9, 0, 9, 0],
    [9, 9, 9, 9, 9],
    [9, 9, 9, 9, 9],
    [0, 9, 9, 9, 0],
    [0, 0, 9, 0, 0],
]);
