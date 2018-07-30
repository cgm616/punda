use core::cell::RefCell;
use core::ops::{Deref, DerefMut};
use cortex_m::interrupt::Mutex;

use alloc::boxed::Box;
use alloc::vec::Vec;

use hal::gpio::gpio::{PIN17, PIN26};
use hal::gpio::{Floating, Input};
use nrf51::GPIOTE;

static GPIOTE: Mutex<RefCell<Option<GPIOTE>>> = Mutex::new(RefCell::new(None));
static A_PIN: Mutex<RefCell<Option<PIN17<Input<Floating>>>>> = Mutex::new(RefCell::new(None));
static B_PIN: Mutex<RefCell<Option<PIN26<Input<Floating>>>>> = Mutex::new(RefCell::new(None));

static A_HANDLERS: Mutex<RefCell<Option<Vec<Handler>>>> = Mutex::new(RefCell::new(None));
static B_HANDLERS: Mutex<RefCell<Option<Vec<Handler>>>> = Mutex::new(RefCell::new(None));
static BOTH_HANDLERS: Mutex<RefCell<Option<Vec<Handler>>>> = Mutex::new(RefCell::new(None));

struct Handler(Box<Fn() -> ()>);

unsafe impl Send for Handler {}

impl Deref for Handler {
    type Target = Box<Fn() -> ()>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Handler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Handler {
    fn new(f: impl Fn() -> () + 'static) -> Self {
        Handler(Box::new(f))
    }
}

crate fn init_buttons(
    a_pin: PIN17<Input<Floating>>,
    b_pin: PIN26<Input<Floating>>,
    gpiote: GPIOTE,
) {
    gpiote.config[0].write(|w| unsafe { w.mode().event().psel().bits(17).polarity().hi_to_lo() });
    gpiote.intenset.write(|w| w.in0().set_bit());
    gpiote.events_in[0].write(|w| unsafe { w.bits(0) });

    gpiote.config[1].write(|w| unsafe { w.mode().event().psel().bits(26).polarity().hi_to_lo() });
    gpiote.intenset.write(|w| w.in1().set_bit());
    gpiote.events_in[1].write(|w| unsafe { w.bits(0) });

    cortex_m::interrupt::free(|cs| {
        *GPIOTE.borrow(cs).borrow_mut() = Some(gpiote);
        *A_PIN.borrow(cs).borrow_mut() = Some(a_pin);
        *B_PIN.borrow(cs).borrow_mut() = Some(b_pin);
        *A_HANDLERS.borrow(cs).borrow_mut() = Some(Vec::with_capacity(8));
        *B_HANDLERS.borrow(cs).borrow_mut() = Some(Vec::with_capacity(8));
        *BOTH_HANDLERS.borrow(cs).borrow_mut() = Some(Vec::with_capacity(8));
    });
}

crate fn gpiote_handler() {
    cortex_m::interrupt::free(|cs| {
        if let (
            Some(ref mut gpiote),
            Some(ref mut a_handlers),
            Some(ref mut b_handlers),
            Some(ref mut both_handlers),
        ) = (
            GPIOTE.borrow(cs).borrow_mut().deref_mut(),
            A_HANDLERS.borrow(cs).borrow_mut().deref_mut(),
            B_HANDLERS.borrow(cs).borrow_mut().deref_mut(),
            BOTH_HANDLERS.borrow(cs).borrow_mut().deref_mut(),
        ) {
            let a = gpiote.events_in[0].read().bits() != 0;
            let b = gpiote.events_in[1].read().bits() != 0;

            match (a, b) {
                (true, true) => both_handlers.iter().for_each(|f| f()),
                (true, false) => a_handlers.iter().for_each(|f| f()),
                (false, true) => b_handlers.iter().for_each(|f| f()),
                _ => unreachable!(),
            }

            gpiote.events_in[0].write(|w| unsafe { w.bits(0) });
            gpiote.events_in[1].write(|w| unsafe { w.bits(0) });
        }
    })
}

macro_rules! handlers {
    ($($name:ident $var:path),+ $(,)*) => {
        $( pub fn $name(f: impl Fn() -> () + 'static) {
            cortex_m::interrupt::free(|cs| {
                if let Some(ref mut handlers) = $var.borrow(cs).borrow_mut().deref_mut() {
                    handlers.push(Handler::new(f))
                }
            })
        })+
    };
}

handlers! {
    register_a_button_handler A_HANDLERS,
    register_b_button_handler B_HANDLERS,
    register_both_button_handler BOTH_HANDLERS,
}

/*
#[macro_export]
macro_rules! register_a_button_handler {
    ($path:ident) => {
        #[export_name = "a_button_handler"]
        pub extern "C" fn __impl_a_button_handler() {
            let f: fn() = $path;

            f()
        }
    };
}

#[macro_export]
macro_rules! register_b_button_handler {
    ($path:ident) => {
        #[export_name = "b_button_handler"]
        pub extern "C" fn __impl_b_button_handler() {
            let f: fn() = $path;

            f()
        }
    };
}

#[macro_export]
macro_rules! register_a_b_button_handler {
    ($path:ident) => {
        #[export_name = "a_b_button_handler"]
        pub extern "C" fn __impl_a_b_button_handler() {
            let f: fn() = $path;

            f()
        }
    };
}
*/
