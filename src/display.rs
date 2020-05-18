use super::{
    context::UserContext,
    syscall::{syscall, Producer, Syscall},
};
pub use microbit::display::{image, Display as _DisplayBackend};
use microbit::display::{Frame, MicrobitFrame, Render};

pub type DisplayBackend = _DisplayBackend<MicrobitFrame>;

pub fn show(cx: &mut UserContext, image: &impl Render) {
    let mut frame = MicrobitFrame::const_default();
    frame.set(image);
    syscall(cx, Syscall::StartDisplay(frame));
}
