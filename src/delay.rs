use super::context::*;
use microbit::hal::prelude::*;

pub fn delay_ms<C: DelayCapable>(cx: &mut C, ms: u32) {
    cx.get_timer().delay_ms(ms);
}
