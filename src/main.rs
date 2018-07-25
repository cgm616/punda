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
extern crate nrf51_hal as hal;
extern crate panic_semihosting;

use hal::delay::Delay;
use hal::prelude::*;
use hal::serial::{self, Serial, BAUDRATEW};

use cortex_m::interrupt::Mutex;

use nrf51::TIMER0;

use alloc::string::String;
use alloc::string::ToString;
use alloc_cortex_m::CortexMHeap;

use core::cell::RefCell;
use core::fmt::Write;
use core::ops::DerefMut;

mod display;
mod temp;

use display::Display;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

entry!(main);

fn main() -> ! {
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

        if let Some(mut p) = cortex_m::peripheral::Peripherals::take() {
            p.NVIC.enable(nrf51::Interrupt::RTC1);
            p.NVIC.clear_pending(nrf51::Interrupt::RTC1);
        }

        let mut delay: Delay = Delay::new(p.TIMER0);

        /*
        display::run_animation(
            display::ScrollingText::new("Hello, world!", 100),
            &mut delay,
        );

        delay.delay_ms(1000u32);
        */

        loop {
            let temp = temp::measure_temp_float(&mut p.TEMP, temp::Degrees::Fahrenheit);
            let temp: String = format!("{:.2}", temp);

            display::animation::run_animation(
                display::animation::text::ScrollingText::new(&temp, 100),
                &mut delay,
            );
        }
    }

    loop {}
}

/*
interrupt! {
    RTC1,
    tick,
    state: u32 = 0
}

#[no_mangle]
pub fn tick(count: &mut u32) {
    cortex_m::interrupt::free(|cs| {
        *count = *count + 1;

        if let (Some(ref mut tx), Some(ref mut rtc)) = (
            TX.borrow(cs).borrow_mut().deref_mut(),
            RTC.borrow(cs).borrow_mut().deref_mut(),
        ) {
            if *count % 8 == 0 {
                print_regs(tx);
            }
            rtc.events_tick.write(|w| unsafe { w.bits(0) });
        }
    });
}

fn print_regs(tx: &mut serial::Tx<nrf51::UART0>) {
    let control_reg = cortex_m::register::control::read();
    let _ = write!(tx, "\n\rUsing {:?} for stack\n\r", control_reg.spsel());
    let stack_pointer = cortex_m::register::msp::read();
    let _ = write!(tx, "Stack pointer is: {:x}\n\r", stack_pointer);
    let pc = cortex_m::register::pc::read();
    let _ = write!(tx, "Program counter is: {:x}\n\r", pc);
}
*/

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
