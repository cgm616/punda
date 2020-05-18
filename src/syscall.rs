use super::context::UserContext;
use heapless::{consts::*, spsc, String};
use microbit::{display::MicrobitFrame, Interrupt};
use microbit::{GPIO, UART0};

#[derive(Debug)]
pub enum Syscall {
    Empty,
    StartDisplay(MicrobitFrame),
    StopDisplay,
    SerialPrint(String<U50>),
}

pub type Queue = spsc::Queue<Syscall, U8, u8>;
pub type Producer = spsc::Producer<'static, Syscall, U8, u8>;
pub type Consumer = spsc::Consumer<'static, Syscall, U8, u8>;

pub fn syscall(cx: &mut UserContext, call: Syscall) {
    cx._producer.enqueue(call);
    rtfm::pend(Interrupt::SWI5);
}
