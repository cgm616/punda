#![no_main]
#![no_std]
#![feature(extern_prelude)]
#![feature(alloc)]
#![feature(lang_items)]
#![feature(non_modrs_mods)]
#![feature(const_fn)]
#![feature(crate_visibility_modifier)]

#[macro_use(format)]
extern crate alloc;
extern crate alloc_cortex_m;
extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
use alloc::alloc::Layout;
use rt::ExceptionFrame;
#[macro_use(interrupt)]
extern crate nrf51;
extern crate nb;
extern crate nrf51_hal as hal;
extern crate panic_semihosting;

use hal::delay::Delay;
use hal::prelude::*;

use cortex_m::interrupt::Mutex;

use nrf51::TIMER0;

use alloc::boxed::Box;
use alloc::prelude::*;
use alloc::string::String;
use alloc::string::ToString;
use alloc_cortex_m::CortexMHeap;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;

pub mod display;
pub mod serial;
pub mod temp;

use display::Display;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[macro_export]
macro_rules! start {
    ($path:ident) => {
        #[export_name = "begin"]
        pub extern "C" fn __impl_begin() -> ! {
            let f: fn() -> ! = $path;

            f()
        }
    };
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

entry!(__start);

fn __start() -> ! {
    extern "C" {
        fn begin() -> !;
    }

    let start = rt::heap_start() as usize;
    let size = 1024; // in bytes
    unsafe {
        ALLOCATOR.init(start, size);
    }

    if let Some(mut p) = nrf51::Peripherals::take() {
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

        let gpio = p.GPIO.split();

        let row1 = gpio.pin13.into_push_pull_output();
        let row2 = gpio.pin14.into_push_pull_output();
        let row3 = gpio.pin15.into_push_pull_output();

        let column1 = gpio.pin4.into_push_pull_output();
        let column2 = gpio.pin5.into_push_pull_output();
        let column3 = gpio.pin6.into_push_pull_output();
        let column4 = gpio.pin7.into_push_pull_output();
        let column5 = gpio.pin8.into_push_pull_output();
        let column6 = gpio.pin9.into_push_pull_output();
        let column7 = gpio.pin10.into_push_pull_output();
        let column8 = gpio.pin11.into_push_pull_output();
        let column9 = gpio.pin12.into_push_pull_output();

        display::init_display(
            row1, row2, row3, column1, column2, column3, column4, column5, column6, column7,
            column8, column9, p.RTC1,
        );

        let txpin = gpio.pin24.into_push_pull_output();
        let rxpin = gpio.pin25.into_floating_input();

        serial::init_serial(p.UART0, txpin, rxpin);

        if let Some(mut p) = cortex_m::peripheral::Peripherals::take() {
            p.NVIC.enable(nrf51::Interrupt::RTC1);
            p.NVIC.clear_pending(nrf51::Interrupt::RTC1);
        }

        unsafe {
            begin();
        }
    }

    loop {}
}

#[lang = "oom"]
#[no_mangle]
pub fn rust_oom(_: Layout) -> ! {
    panic!("Out of heap memory!")
}

interrupt! {
    RTC1,
    display::refresh_display,
    state: usize = 0
}
