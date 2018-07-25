// This file should work

struct Task {
    stack_pointer: usize,
    callback: fn() -> (),
    status: TaskStatus,
    priority: u8,
}

enum TaskStatus {
    Ready,
    Running,
    NotReady,
}
