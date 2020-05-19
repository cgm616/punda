use super::syscall::Producer;
use microbit::hal::{delay::DelayTimer, nrf51};

pub struct UserContext<'r> {
    pub _producer: &'r mut Producer,
    pub _timer: &'r mut DelayTimer<nrf51::TIMER0>,
}
