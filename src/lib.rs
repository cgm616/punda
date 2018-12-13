#![no_std]
#![feature(alloc)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(tool_lints)]
#![feature(crate_visibility_modifier)]
#![allow(dead_code)]

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
pub extern crate nrf51_hal as hal;
extern crate panic_semihosting;

use hal::prelude::*;

use alloc_cortex_m::CortexMHeap;

pub mod display;
pub mod gpio;
mod rtc;
pub mod serial;
pub mod temp;

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

    if let Some(p) = nrf51::Peripherals::take() {
        p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
        while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
        p.CLOCK.events_lfclkstarted.write(|w| unsafe { w.bits(0) });

        rtc::init_scheduler(p.RTC1);

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
            column8, column9,
        );

        let txpin = gpio.pin24.into_push_pull_output();
        let rxpin = gpio.pin25.into_floating_input();

        serial::init_serial(p.UART0, txpin, rxpin);

        let a_pin = gpio.pin17.into_floating_input();
        let b_pin = gpio.pin26.into_floating_input();

        gpio::init_buttons(a_pin, b_pin, p.GPIOTE);

        if let Some(mut p) = cortex_m::peripheral::Peripherals::take() {
            p.NVIC.enable(nrf51::Interrupt::RTC1);
            p.NVIC.enable(nrf51::Interrupt::GPIOTE);
            p.NVIC.clear_pending(nrf51::Interrupt::RTC1);
            p.NVIC.clear_pending(nrf51::Interrupt::GPIOTE);
        }

        unsafe {
            begin();
        }
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

#[lang = "oom"]
#[no_mangle]
pub fn rust_oom(_: Layout) -> ! {
    panic!("Out of heap memory!")
}

interrupt! {
    RTC1,
    rtc::handler
}

interrupt! {
    GPIOTE,
    gpio::gpiote_handler
}
