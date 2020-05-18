#![no_std]
#![no_main]

punda::punda!(init: init);

use punda::display::{self, image::GreyscaleImage};

fn init(cx: &mut punda::context::UserContext) {
    hprintln!("Showing image");

    let image = GreyscaleImage::new(&[
        [0, 7, 0, 7, 0],
        [7, 3, 7, 3, 7],
        [7, 3, 3, 3, 7],
        [0, 7, 3, 7, 0],
        [0, 0, 7, 0, 0],
    ]);

    display::show(cx, &image);

    hprintln!("Done");
}
