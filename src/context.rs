use super::syscall::Producer;

pub struct UserContext<'r> {
    pub _producer: &'r mut Producer,
}
