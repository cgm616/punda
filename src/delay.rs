use super::context::UserContext;
use microbit::hal::prelude::*;

pub fn delay_ms(cx: &mut UserContext, ms: u32) {
    cx._timer.delay_ms(ms);
}
