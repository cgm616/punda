use super::{super::DisplayImage, Animate, Frame};

pub struct Heart(u32);

impl Heart {
    const HEART0: DisplayImage = DisplayImage([
        [false, false, false, false, false],
        [false, false, false, false, false],
        [false, false, true, false, false],
        [false, false, false, false, false],
        [false, false, false, false, false],
    ]);

    const HEART1: DisplayImage = DisplayImage([
        [false, false, false, false, false],
        [false, true, false, true, false],
        [false, true, true, true, false],
        [false, false, true, false, false],
        [false, false, false, false, false],
    ]);

    const HEART2: DisplayImage = DisplayImage([
        [false, true, false, true, false],
        [true, false, true, false, true],
        [true, false, false, false, true],
        [false, true, false, true, false],
        [false, false, true, false, false],
    ]);

    pub fn new() -> Self {
        Heart(0)
    }
}

impl Animate for Heart {
    fn next_screen(&mut self) -> Option<Frame> {
        let frame = match self.0 {
            0 => Some(Frame {
                image: Self::HEART0,
                length: 400,
            }),
            1 => Some(Frame {
                image: Self::HEART1,
                length: 200,
            }),
            2 => Some(Frame {
                image: Self::HEART2,
                length: 400,
            }),
            3 => Some(Frame {
                image: Self::HEART1,
                length: 200,
            }),
            _ => None,
        };

        self.0 = (self.0 + 1);

        frame
    }

    fn frames(&self) -> u32 {
        4
    }
}
