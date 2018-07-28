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
