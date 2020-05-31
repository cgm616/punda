use super::syscall::{Producer, Syscall};
use microbit::hal::{delay::DelayTimer, nrf51};
use rtfm::Mutex;

// The three contexts are safe because:
// 1. The user button handler and the user init function have the same priority
//    and do not compete.
// 2. The user idle function can only run after the user init function ends.
// 3. The user button handler doesn't have access to the timer, so while the
//    button handler can interrupt the idle function, it can't compete for a
//    resources not protected.
// 4. There is (as of nrf51-hal v0.7.1) no state held in DelayTimer other than
//    physical registers.

pub struct InitContext<'r> {
    pub _producer: &'r mut Producer,
    pub _timer: &'r mut DelayTimer<nrf51::TIMER0>,
}

pub struct HandlerContext<'r> {
    pub _producer: &'r mut Producer,
}

pub struct IdleContext<P: Mutex<T = Producer>> {
    pub _producer: P,
    //pub _timer: &'r mut DelayTimer<nrf51::TIMER0>,
}

pub trait DelayCapable {
    fn get_timer(&mut self) -> &mut DelayTimer<nrf51::TIMER0>;
}

impl<'r> DelayCapable for InitContext<'r> {
    fn get_timer(&mut self) -> &mut DelayTimer<nrf51::TIMER0> {
        self._timer
    }
}

impl<P: Mutex<T = Producer>> DelayCapable for IdleContext<P> {
    fn get_timer(&mut self) -> &mut DelayTimer<nrf51::TIMER0> {
        // See above why this is safe
        // It might not be though...................
        unsafe { core::mem::transmute::<_, &mut DelayTimer<nrf51::TIMER0>>(1usize) }
    }
}

pub trait SyscallCapable {
    fn send(&mut self, call: Syscall);
}

impl<'r> SyscallCapable for InitContext<'r> {
    fn send(&mut self, call: Syscall) {
        self._producer.enqueue(call);
    }
}

impl<'r> SyscallCapable for HandlerContext<'r> {
    fn send(&mut self, call: Syscall) {
        self._producer.enqueue(call);
    }
}

impl<P: Mutex<T = Producer>> SyscallCapable for IdleContext<P> {
    fn send(&mut self, call: Syscall) {
        self._producer.lock(|p| p.enqueue(call));
    }
}
