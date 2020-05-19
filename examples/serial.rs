#![no_std]
#![no_main]

use punda::serial;

punda::punda!(init: init, idle: idle);

fn init(cx: &mut punda::context::UserContext) {
    serial::println(cx, "Hello, world!".into());
}

fn idle(cx: &mut punda::context::UserContext) -> ! {
    loop {}
}
