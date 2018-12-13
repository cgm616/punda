use cortex_m::interrupt::Mutex;

use hal::gpio::gpio::{
    PIN, PIN10, PIN11, PIN12, PIN13, PIN14, PIN15, PIN4, PIN5, PIN6, PIN7, PIN8, PIN9,
};
use hal::gpio::{Output, PushPull};
use hal::prelude::*;

use alloc::boxed::Box;
use core::cell::RefCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::AtomicBool;

use super::rtc::{self, RTCInterrupt};

pub mod animation;
pub mod constant;
pub mod image;

pub use self::image::DisplayImage;
use self::image::MatrixImage;

pub struct Display {
    image: Option<MatrixImage>,
    animator: Option<Animator>,
}

struct Driver {
    rows: [LED; 3],
    columns: [LED; 9],
    current_row: usize,
}

pub struct Animator(Box<animation::Animate>);

unsafe impl Send for Animator {} // TODO: fix this, definitely unsafe here

impl Deref for Animator {
    type Target = Box<animation::Animate>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Animator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Animator {
    fn new(animator: impl animation::Animate + 'static) -> Self {
        Animator(Box::new(animator))
    }
}

type LED = PIN<Output<PushPull>>;

pub static DISPLAY: Mutex<RefCell<Option<Display>>> = Mutex::new(RefCell::new(None));
static DRIVER: Mutex<RefCell<Option<Driver>>> = Mutex::new(RefCell::new(None));
static ANIMATION_DONE: AtomicBool = AtomicBool::new(false);

#[allow(clippy::too_many_arguments)]
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
) {
    cortex_m::interrupt::free(|cs| {
        let mut driver = Driver::new(
            row1, row2, row3, column1, column2, column3, column4, column5, column6, column7,
            column8, column9,
        );
        driver.clear();
        *DRIVER.borrow(cs).borrow_mut() = Some(driver);

        let display = Display {
            image: None,
            animator: None,
        };
        *DISPLAY.borrow(cs).borrow_mut() = Some(display);
    })
}

crate fn refresh_display(_counter: u32) {
    cortex_m::interrupt::free(|cs| {
        if let (Some(ref mut display), Some(ref mut driver)) = (
            DISPLAY.borrow(cs).borrow_mut().deref_mut(),
            DRIVER.borrow(cs).borrow_mut().deref_mut(),
        ) {
            driver.refresh(display);
            driver.current_row = (driver.current_row + 1) % 3;
        }
    });
}

crate fn refresh_animation(counter: u32) {
    cortex_m::interrupt::free(|cs| {
        if let (Some(ref mut display), Some(ref mut scheduler)) = (
            DISPLAY.borrow(cs).borrow_mut().deref_mut(),
            rtc::SCHEDULER.borrow(cs).borrow_mut().deref_mut(),
        ) {
            let mut unset = false;
            if let Some(ref mut animator) = display.animator {
                if let Some(frame) = animator.next_screen() {
                    scheduler.unset_interrupt(RTCInterrupt::Compare0);
                    scheduler
                        .set_cmp_interrupt(
                            RTCInterrupt::Compare0,
                            refresh_animation,
                            (counter + (frame.length / 5)) % (2_u32.pow(24) - 1),
                        )
                        .unwrap();
                    display.image = Some(frame.image.into());
                } else {
                    scheduler.unset_interrupt(RTCInterrupt::Compare0);
                    ANIMATION_DONE.store(true, core::sync::atomic::Ordering::Relaxed);
                    unset = true;
                }
            } else {
                scheduler.unset_interrupt(RTCInterrupt::Compare0);
                ANIMATION_DONE.store(true, core::sync::atomic::Ordering::Relaxed);
            }

            if unset {
                display.animator = None;
            }
        }
    });
}

pub fn display_image(img: impl Into<DisplayImage>) {
    cortex_m::interrupt::free(|cs| {
        if let (Some(ref mut display), Some(ref mut scheduler)) = (
            DISPLAY.borrow(cs).borrow_mut().deref_mut(),
            rtc::SCHEDULER.borrow(cs).borrow_mut().deref_mut(),
        ) {
            display.animator = None;
            display.image = Some(img.into().into());
            scheduler.unset_interrupt(RTCInterrupt::Tick);
            scheduler
                .set_agnostic_interrupt(RTCInterrupt::Tick, refresh_display)
                .unwrap();
        }
    });
}

pub fn run_animation(mut animator: impl animation::Animate + 'static, block: bool) {
    if let Some(frame) = animator.next_screen() {
        cortex_m::interrupt::free(|cs| {
            if let (Some(ref mut display), Some(ref mut scheduler)) = (
                DISPLAY.borrow(cs).borrow_mut().deref_mut(),
                rtc::SCHEDULER.borrow(cs).borrow_mut().deref_mut(),
            ) {
                ANIMATION_DONE.store(false, core::sync::atomic::Ordering::Relaxed);
                let length = frame.length;
                display.image = Some(frame.image.into());
                display.animator = Some(Animator::new(animator));

                scheduler.unset_interrupt(RTCInterrupt::Tick);
                scheduler
                    .set_agnostic_interrupt(RTCInterrupt::Tick, refresh_display)
                    .unwrap();

                let compare = (scheduler.current_counter() + (length / 5)) % (2_u32.pow(24) - 1);
                scheduler.unset_interrupt(RTCInterrupt::Compare0);
                scheduler
                    .set_cmp_interrupt(RTCInterrupt::Compare0, refresh_animation, compare)
                    .unwrap();
            }
        });

        if block {
            while !ANIMATION_DONE.load(core::sync::atomic::Ordering::Relaxed) {}
        }
    }
}

impl Driver {
    #[allow(clippy::too_many_arguments)]
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
            current_row: 0,
        }
    }

    fn refresh(&mut self, display: &mut Display) {
        // Match over the Option<MatrixImage>. If there is an image, display it;
        // otherwise clear the display.
        match display.image {
            Some(ref image) => {
                // Figure out the previous row index so we can turn it off. I'm
                // pretty sure there's a better way to do this, but I don't know
                // what it is.
                let previous_row = match self.current_row {
                    0 => 2,
                    1..=2 => self.current_row - 1,
                    _ => panic!("Current row not 0 through 2"),
                };

                // Turn off the previous row.
                self.rows[previous_row].set_low();

                // Obtain the current row in the image in the display.
                let img_row = &image[self.current_row];

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
                self.rows[self.current_row].set_high();
            }
            None => {
                self.clear();
                cortex_m::interrupt::free(|cs| {
                    if let Some(ref mut scheduler) =
                        rtc::SCHEDULER.borrow(cs).borrow_mut().deref_mut()
                    {
                        scheduler.unset_interrupt(RTCInterrupt::Tick);
                    }
                })
            }
        }
    }

    fn clear(&mut self) {
        self.rows.iter_mut().for_each(|led| led.set_low());
        self.columns.iter_mut().for_each(|led| led.set_high());
    }
}
