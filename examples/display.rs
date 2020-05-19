#![no_std]
#![no_main]

use punda::display::{self, image::GreyscaleImage};

punda::punda!(init: init, idle: idle);

fn init(cx: &mut punda::context::UserContext) {
    let large = GreyscaleImage::new(&[
        [0, 5, 6, 6, 0],
        [6, 5, 0, 0, 0],
        [6, 4, 0, 0, 0],
        [6, 5, 0, 0, 0],
        [0, 5, 6, 6, 0],
    ]);

    display::show(cx, &large);
}

fn idle(cx: &mut punda::context::UserContext) -> ! {
    loop {}
}
