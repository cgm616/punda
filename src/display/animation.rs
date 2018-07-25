use hal;

use super::DisplayImage;

pub mod heart;
pub mod text;

pub struct Frame {
    pub image: DisplayImage,
    pub length: u32,
}

pub trait Animate {
    fn next_screen(&mut self) -> Option<Frame>;
    fn frames(&self) -> u32;
}

pub fn run_animation(
    mut animator: impl Animate,
    delay: &mut impl hal::hal::blocking::delay::DelayMs<u32>,
) {
    for _ in 0..animator.frames() {
        let frame = match animator.next_screen() {
            Some(frame) => frame,
            None => return,
        };

        let image = frame.image;
        let length = frame.length;

        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut display) = *super::DISPLAY.borrow(cs).borrow_mut() {
                display.display_image(image);
            }
        });

        delay.delay_ms(length);
    }
}
