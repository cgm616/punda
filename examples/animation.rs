#![no_std]
#![no_main]

use punda::{
    delay,
    display::{self, image::GreyscaleImage},
};
use rtfm::Mutex;

punda::punda!(init = init, idle = idle);

fn init(_cx: &mut punda::context::InitContext) {}

fn idle<P: Mutex<T = Producer>>(cx: &mut punda::context::IdleContext<P>) -> ! {
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
