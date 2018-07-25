use hal::gpio::gpio::{
    PIN10, PIN11, PIN12, PIN13, PIN14, PIN15, PIN4, PIN5, PIN6, PIN7, PIN8, PIN9, Parts, PIN,
};
use hal::gpio::{Output, PushPull};
use hal::prelude::*;

use core::ops::{Deref, DerefMut};

// (row, column)
const DISPLAY_MAP: [[(usize, usize); 5]; 5] = [
    [(0, 0), (1, 3), (0, 1), (1, 4), (0, 2)],
    [(2, 3), (2, 4), (2, 5), (2, 6), (2, 7)],
    [(1, 1), (0, 8), (1, 2), (2, 8), (1, 0)],
    [(0, 7), (0, 6), (0, 5), (0, 4), (0, 3)],
    [(2, 2), (1, 6), (2, 0), (1, 5), (2, 1)],
];

pub type LED = PIN<Output<PushPull>>;

pub struct Display {
    rows: [LED; 3],
    columns: [LED; 9],
}

pub struct DisplayImage([[bool; 5]; 5]);
pub struct MatrixImage([[bool; 9]; 3]);

impl Deref for DisplayImage {
    type Target = [[bool; 5]; 5];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DisplayImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for MatrixImage {
    type Target = [[bool; 9]; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MatrixImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub const CHECKERBOARD: DisplayImage = DisplayImage([
    [true, false, true, false, true],
    [false, true, false, true, false],
    [true, false, true, false, true],
    [false, true, false, true, false],
    [true, false, true, false, true],
]);

pub const SMILEY: DisplayImage = DisplayImage([
    [false, true, false, true, false],
    [false, false, false, false, false],
    [true, false, false, false, true],
    [false, true, true, true, false],
    [false, false, false, false, false],
]);

impl Display {
    pub fn new(
        row1: PIN13<Output<PushPull>>,
        row2: PIN14<Output<PushPull>>,
        row3: PIN15<Output<PushPull>>,
        column1: PIN4<Output<PushPull>>,
        column2: PIN5<Output<PushPull>>,
        column3: PIN6<Output<PushPull>>,
        column4: PIN7<Output<PushPull>>,
        column5: PIN8<Output<PushPull>>,
        column6: PIN9<Output<PushPull>>,
        column7: PIN10<Output<PushPull>>,
        column8: PIN11<Output<PushPull>>,
        column9: PIN12<Output<PushPull>>,
    ) -> Self {
        Display {
            rows: [row1.downgrade(), row2.downgrade(), row3.downgrade()],
            columns: [
                column1.downgrade(),
                column2.downgrade(),
                column3.downgrade(),
                column4.downgrade(),
                column5.downgrade(),
                column6.downgrade(),
                column7.downgrade(),
                column8.downgrade(),
                column9.downgrade(),
            ],
        }
    }

    pub fn clear(&mut self) {
        self.rows.iter_mut().for_each(|led| led.set_low());
        self.columns.iter_mut().for_each(|led| led.set_high());
    }

    pub fn show_image(
        &mut self,
        img: impl Into<MatrixImage>,
        delay: &mut impl nrf51_hal::hal::blocking::delay::DelayUs<u32>,
        brightness: u8,
    ) {
        if brightness == 0 {
            loop {}
        }
        let img: MatrixImage = img.into();
        let interval = brightness as u32 * 8_u32;
        let refresh = 2048 - interval;

        loop {
            for (pin_row, img_row) in self.rows.iter_mut().zip(img.iter()) {
                self.columns
                    .iter_mut()
                    .zip(img_row.iter())
                    .for_each(|(pin, value)| {
                        if *value {
                            pin.set_low();
                        } else {
                            pin.set_high();
                        }
                    });
                pin_row.set_high();
                delay.delay_us(interval);
                pin_row.set_low();
            }
            delay.delay_us(refresh);
        }
    }
}
