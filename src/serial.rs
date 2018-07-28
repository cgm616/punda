use core::cell::RefCell;
use core::fmt::{Arguments, Write};
use core::ops::DerefMut;
use cortex_m::interrupt::Mutex;

use hal::gpio::{Floating, Input, Output, PushPull};

use hal::serial::{self, Rx, Serial, Tx};

use hal::gpio::gpio::{PIN24, PIN25};
use nrf51::UART0;

pub static TX: Mutex<RefCell<Option<Tx<UART0>>>> = Mutex::new(RefCell::new(None));
pub static RX: Mutex<RefCell<Option<Rx<UART0>>>> = Mutex::new(RefCell::new(None));

pub fn init_serial(uart: UART0, txpin: PIN24<Output<PushPull>>, rxpin: PIN25<Input<Floating>>) {
    let serial = Serial::uart0(
        uart,
        txpin.downgrade(),
        rxpin.downgrade(),
        serial::BAUDRATEW::BAUD115200,
    );
    let (mut tx, mut rx) = serial.split();

    cortex_m::interrupt::free(|cs| {
        *TX.borrow(cs).borrow_mut() = Some(tx);
        *RX.borrow(cs).borrow_mut() = Some(rx);
    });
}

pub fn write(message: Arguments) -> core::fmt::Result {
    cortex_m::interrupt::free(|cs| match TX.borrow(cs).borrow_mut().deref_mut() {
        Some(ref mut tx) => write!(tx, "{}", message),
        None => Err(core::fmt::Error),
    })
}

pub fn writeln(message: Arguments) -> core::fmt::Result {
    write(format_args!("{}\n\r", message))
}
