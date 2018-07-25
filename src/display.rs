use cortex_m::interrupt::Mutex;

use nrf51::RTC1;

use hal::gpio::gpio::{
    PIN10, PIN11, PIN12, PIN13, PIN14, PIN15, PIN4, PIN5, PIN6, PIN7, PIN8, PIN9, Parts, PIN,
};
use hal::gpio::{Output, PushPull};
use hal::prelude::*;

//use alloc::arc::Arc;
//use alloc::boxed::Box;
use core::cell::RefCell;
use core::ops::DerefMut;

pub mod animation;
pub mod image;

pub use self::image::DisplayImage;
use self::image::MatrixImage;

pub struct Display {
    image: Option<MatrixImage>,
    //    animator: Option<Box<animation::Animate>>,
}

struct Driver {
    rows: [LED; 3],
    columns: [LED; 9],
    timer: RTC1,
}

type LED = PIN<Output<PushPull>>;

pub static DISPLAY: Mutex<RefCell<Option<Display>>> = Mutex::new(RefCell::new(None));
static DRIVER: Mutex<RefCell<Option<Driver>>> = Mutex::new(RefCell::new(None));

pub fn init_display(
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
    timer: RTC1,
) {
    timer.prescaler.write(|w| unsafe { w.bits(163) });
    timer.evtenset.write(|w| w.tick().set_bit());
    timer.intenclr.write(|w| {
        w.tick()
            .set_bit()
            .ovrflw()
            .set_bit()
            .compare0()
            .set_bit()
            .compare1()
            .set_bit()
            .compare2()
            .set_bit()
            .compare3()
            .set_bit()
    });
    timer.intenset.write(|w| w.tick().set_bit());

    cortex_m::interrupt::free(|cs| {
        let mut driver = Driver::new(
            row1, row2, row3, column1, column2, column3, column4, column5, column6, column7,
            column8, column9, timer,
        );
        driver.clear();
        *DRIVER.borrow(cs).borrow_mut() = Some(driver);

        let mut display = Display {
            image: None,
            //            animator: None,
        };
        *DISPLAY.borrow(cs).borrow_mut() = Some(display);
    })
}

#[no_mangle]
pub fn refresh_display(current_row: &mut usize) {
    // Figure out the previous row index so we can turn it off. I'm
    // pretty sure there's a better way to do this, but I don't know
    // what it is.
    let previous_row = match current_row {
        0 => 2,
        1..=2 => *current_row - 1,
        _ => panic!("Current row index not 0 through 2"),
    };

    cortex_m::interrupt::free(|cs| {
        if let (Some(ref mut display), Some(ref mut driver)) = (
            DISPLAY.borrow(cs).borrow_mut().deref_mut(),
            DRIVER.borrow(cs).borrow_mut().deref_mut(),
        ) {
            driver.refresh(display, *current_row, previous_row);
            driver.clear_tick_bit();
        }
    });

    *current_row = (*current_row + 1) % 3;
}

impl Display {
    pub fn display_image(&mut self, img: impl Into<DisplayImage>) {
        self.clear();
        self.image = Some(img.into().into());
        cortex_m::interrupt::free(|cs| match *DRIVER.borrow(cs).borrow_mut() {
            Some(ref mut driver) => driver.start_timer(),
            None => panic!("Driver has not been initialized."),
        });
    }

    /*    pub fn run_animation(&mut self, mut animator: Box<dyn animation::Animate>) {
        self.clear();
        if let Some(frame) = animator.next_screen() {
            self.image = Some(frame.image.into());
            self.animator = Some(animator);

            cortex_m::interrupt::free(|cs| match *DRIVER.borrow(cs).borrow_mut() {
                Some(ref mut driver) => {
                    driver.add_animation_interrupt();
                    driver.set_animation_interrupt(frame.length);
                    driver.start_timer();
                }
                None => panic!("Driver has not been initialized."),
            })
        }
    } */

    pub fn clear(&mut self) {
        self.image = None;
        //        self.animator = None;
    }
}

impl Driver {
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
        timer: RTC1,
    ) -> Self {
        Driver {
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
            timer: timer,
        }
    }

    fn refresh(&mut self, display: &mut Display, current_row: usize, previous_row: usize) {
        // Match over the Option<MatrixImage>. If there is an image, display it;
        // otherwise clear the display.
        match display.image {
            Some(ref image) => {
                // Turn off the previous row.
                self.rows[previous_row].set_low();

                // Obtain the current row in the image in the display.
                let img_row = &image[current_row];

                // Iterate over each of the column pins in the driver and
                // turn them on or off depending on the data in the image row.
                self.columns
                    .iter_mut()
                    .zip(img_row.iter())
                    .for_each(|(column_pin, value)| {
                        if *value {
                            column_pin.set_low();
                        } else {
                            column_pin.set_high();
                        }
                    });

                // Turn the current row on.
                self.rows[current_row].set_high();
            }
            None => {
                self.clear();
                self.stop_timer();
                //                self.remove_animation_interrupt();
            }
        }
    }

    fn clear(&mut self) {
        self.rows.iter_mut().for_each(|led| led.set_low());
        self.columns.iter_mut().for_each(|led| led.set_high());
    }

    fn start_timer(&mut self) {
        self.timer.tasks_start.write(|w| unsafe { w.bits(1) });
    }

    /*    fn add_animation_interrupt(&mut self) {
        self.timer.intenset.write(|w| w.compare0().set_bit());
    }

    fn set_animation_interrupt(&mut self, delay: u32) {
        let current = self.timer.counter.read().counter().bits();
        let next = (current + (delay * 2)) % ((2 ^ 24) - 1);
        self.timer.cc[0].write(|w| unsafe { w.compare().bits(next) });
    }

    fn remove_animation_interrupt(&mut self) {
        self.timer.intenclr.write(|w| w.compare0().set_bit());
    }

    fn clear_animation_bit(&mut self) {
        self.timer.events_compare[0].reset();
    }*/

    //fn add_tick_interrupt(&mut self) {}

    //fn remove_tick_interrupt(&mut self) {}

    fn stop_timer(&mut self) {
        self.timer.tasks_stop.write(|w| unsafe { w.bits(1) });
    }

    fn clear_tick_bit(&mut self) {
        self.timer.events_tick.reset();
    }
}
