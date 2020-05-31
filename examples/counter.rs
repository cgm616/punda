#![no_std]
#![no_main]

use punda::{
    button::{Button, State},
    context::*,
    delay, serial,
};

use rtfm::Mutex;

use core::sync::atomic::{AtomicU32, Ordering};

punda::punda!(init = init, idle = idle, button_handler = button_handler);

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn init(cx: &mut InitContext) {
    serial::println(cx, "Press a button".into());
}

fn idle<P: Mutex<T = Producer>>(cx: &mut IdleContext<P>) -> ! {
    loop {
        hprintln!("count: {}", COUNTER.load(Ordering::Relaxed));
        delay::delay_ms(cx, 1000);
    }
}

fn button_handler(cx: &mut HandlerContext, button: Button, direction: State) {
    let c = COUNTER.load(Ordering::Relaxed);
    COUNTER.store(c + 1, Ordering::Relaxed);
}
