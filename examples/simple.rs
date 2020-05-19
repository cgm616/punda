#![no_std]
#![no_main]

punda::punda!(init: init, idle: idle);

fn init(cx: &mut punda::context::UserContext) {}

fn idle(cx: &mut punda::context::UserContext) -> ! {
    loop {}
}
