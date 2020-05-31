use super::{
    context::*,
    syscall::{syscall, Producer, Syscall},
};
pub use microbit::display::{image, Display as _DisplayBackend};
use microbit::display::{Frame, MicrobitFrame, Render};

pub type DisplayBackend = _DisplayBackend<MicrobitFrame>;

pub fn show<C: SyscallCapable>(cx: &mut C, image: &impl Render) {
    let mut frame = MicrobitFrame::const_default();
    frame.set(image);
    syscall(cx, Syscall::StartDisplay(frame));
}
