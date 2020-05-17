use core::cmp::{self, Ord};
use heapless::{binary_heap::Min, consts, BinaryHeap};

crate struct Queue {
    timer: nrf51::TIMER0,
    queue: BinaryHeap<NotReady, consts::U2, Min>,
}

trait Timer {
    fn setup(&mut self);
    fn set_timeout(&mut self, time: u32);
    fn start(&mut self);
    fn stop(&mut self);
    fn current_value(&mut self) -> u32;
}

impl Timer for nrf51::TIMER0 {
    fn setup(&mut self) {
        self.tasks_stop.write(|w| unsafe { w.bits(1) });
        self.tasks_clear.write(|w| unsafe { w.bits(1) });
        self.mode.write(|w| w.mode().timer());
        self.prescaler.write(|w| unsafe { w.prescaler().bits(0) });
        self.bitmode.write(|w| w.bitmode()._32bit());
    }

    fn set_timeout(&mut self, time: u32) {
        self.tasks_clear.write(|w| unsafe { w.bits(1) });
        self.cc[0].write(|w| unsafe { w.bits(time) });
    }

    fn start(&mut self) {
        self.intenset.write(|w| w.compare0().set_bit());
        self.tasks_start.write(|w| unsafe { w.bits(1) });
    }

    fn stop(&mut self) {
        self.intenclr.write(|w| w.compare0().set_bit());
        self.tasks_stop.write(|w| unsafe { w.bits(1) });
    }

    fn current_value(&mut self) -> u32 {
        self.tasks_capture[1].write(|w| unsafe { w.bits(1) });
        self.cc[1].read().bits()
    }
}

impl Queue {
    fn new(timer: nrf51::TIMER0) -> Self {
        timer.setup();

        Queue {
            timer,
            queue: BinaryHeap::new(),
        }
    }

    fn enqueue(&mut self, task: NotReady) -> Result<(), NotReady> {
        let current = self.timer.current_value();

        if let Some(queued) = self.queue.peek() {
            if task.tick > queued.tick {
                
            }
        } else {
            self.timer.set_timeout(current.wrapping_add(task.tick));
            self.queue.push(task)
        }

        // Add to the queue and maybe reschedule next interrupt?
    }

    fn pop(&mut self) -> NotReady {
        // Remove from the queue and schedule next interrupt
    }
}

#[derive(Eq, Ord)]
crate struct NotReady {
    handler: fn(u32),
    tick: u32,
}

impl PartialEq for NotReady {
    fn eq(&self, other: &Self) -> bool {
        self.tick == other.tick
    }
}

impl PartialOrd for NotReady {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.tick.cmp(&other.tick))
    }
}
