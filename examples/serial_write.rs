#![no_std]
#![no_main]

#[macro_use]
extern crate easy_microbit as m;

use m::serial;

start!(main);

fn main() -> ! {
    serial::writeln(format_args!("Hello from Rust!")).unwrap();

    loop {}
}
