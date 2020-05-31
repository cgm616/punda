use super::context::*;
use heapless::{consts::*, spsc, String};
use microbit::{display::MicrobitFrame, Interrupt};


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

pub fn syscall<C: SyscallCapable>(cx: &mut C, call: Syscall) {
    cx.send(call);
    rtfm::pend(Interrupt::SWI5);
}
