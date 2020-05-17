#![no_std]
#![no_main]


extern crate punda;
use punda::start;

start!(main);

fn main() -> ! {
    loop {}
}