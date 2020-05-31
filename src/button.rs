#[derive(Debug)]
pub enum Button {
    A,
    B,
    Both,
}

#[derive(Debug)]
pub struct History {
    history: u8,
    pub state: State,
}

#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum State {
    Pushed,
    Released,
}

impl History {
    pub const fn new_released() -> Self {
        History {
            history: 0,
            state: State::Released,
        }
    }

    pub fn measure(&mut self, low: bool) -> bool {
        // A 'low' pin value corresponds to the button being pushed down.

        if low {
            self.history = (self.history << 1) + 1;
        } else {
            self.history <<= 1;
        }

        match (self.history, self.state) {
            (core::u8::MAX, State::Released) => {
                // Button state is 'released' but it is registered as down 8
                // consecutive times
                // => change state
                self.history = core::u8::MAX;
                self.state = State::Pushed;
                true
            }
            (0, State::Pushed) => {
                // Button state is 'pushed' but it is registered as up 8
                // consecutive times
                // => change state
                self.history = 0;
                self.state = State::Released;
                true
            }
            (_, State::Released) => {
                // Button state is 'released' and it seems to be correct
                // => no change
                false
            }
            (_, State::Pushed) => {
                // Button state is 'pushed' and it seems to be correct
                // => no change
                false
            }
        }
    }
}

// TODO: add unit tests
