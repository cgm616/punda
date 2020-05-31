use super::{
    context::*,
    syscall::{self, Syscall},
};
use heapless::{consts::*, String};
use microbit::{GPIO, UART0};

pub fn print<C: SyscallCapable>(cx: &mut C, string: String<U50>) {
    syscall::syscall(cx, Syscall::SerialPrint(string));
}

pub fn println<C: SyscallCapable>(cx: &mut C, string: String<U48>) {
    let mut new: String<U50> = String::new();
    new.push_str(&string);
    new.push_str("\n\r");
    print(cx, new);
}

// TODO: change to a buffer and flush on specific interrupt
pub struct UART0Buffer<'a>(pub &'a UART0);

impl<'a> core::fmt::Write for UART0Buffer<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let uart0 = self.0;
        uart0.tasks_starttx.write(|w| unsafe { w.bits(1) });
        for c in s.as_bytes() {
            /* Write the current character to the output register */
            uart0.txd.write(|w| unsafe { w.bits(u32::from(*c)) });

            /* Wait until the UART is clear to send */
            while uart0.events_txdrdy.read().bits() == 0 {}

            /* And then set it back to 0 again, just because ?!? */
            uart0.events_txdrdy.write(|w| unsafe { w.bits(0) });
        }
        uart0.tasks_stoptx.write(|w| unsafe { w.bits(1) });
        Ok(())
    }
}
