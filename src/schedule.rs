use cortex_m_semihosting::hprintln;
use heapless::{binary_heap::Min, consts::*, BinaryHeap};
use microbit::hal::{
    lo_res_timer::{self, LoResTimer, RtcCc, RtcFrequency},
    nrf51,
};

pub struct Scheduler {
    pub queue: BinaryHeap<Task, U8, Min>,
}

impl Scheduler {
    // The RTC0 will tick at 32768 / (PRESCALER + 1) Hz, which here is 4096.
    const FREQUENCY: RtcFrequency = lo_res_timer::FREQ_4096HZ;

    // The frequency means that there are approx. 4 ticks per millisecond.
    pub const TPMS: u32 = 4;

    // The RTC0 counter only has 24 bits, so this is 2^24.
    const MAX: u32 = 16_777_216;

    // The max amount of time we'll let someone schedule ahead is 30 minutes.
    // If we were more clever, we could keep track of the number of overflows in
    // the future that each task needs, and count those, to go longer.
    const AHEAD: u32 = 4096 * 60 * 30;

    pub fn new(timer: nrf51::RTC0) -> (Self, LoResTimer<nrf51::RTC0>) {
        // First, we set up the timer to count in roughly milliseconds.
        let mut timer = LoResTimer::new(timer);
        timer.set_frequency(Self::FREQUENCY);

        // Now, we enable the comparison event and start the timer.
        timer.enable_compare_event(RtcCc::CC0);
        timer.start();

        // Finally, we set up the binary heap to store the tasks.
        let queue = BinaryHeap::new();

        (Self { queue }, timer)
    }

    /*
    pub fn schedule(&mut self, func: for<'r> fn(&'r mut super::context::Context) -> (), ms: u32) {
        // Make sure that no task can be scheduled too far ahead.
        assert!(ms < Self::AHEAD);

        // Set the correct interrupts
        let trigger = self.set_compare(ms);

        // Now that the interrupt is set up, we can push the task onto the queue.
        self.queue.push(Task {
            trigger,
            func,
            kind: TaskKind::Single,
        });
    }
    */

    /*
    pub fn schedule_recurring(
        &mut self,
        func: for<'r> fn(&'r mut super::context::Context) -> (),
        ms: u32,
    ) {
        // Make sure that no task can be scheduled too far ahead.
        assert!(ms < Self::AHEAD);

        // Set the correct interrupts
        let trigger = self.set_compare(ms);

        // Now that the interrupt is set up, we can push the task onto the queue.
        self.queue.push(Task {
            trigger,
            func,
            kind: TaskKind::Recurring(ms),
        });
    }
    */

    fn set_compare(&mut self, ms: u32) -> u32 {
        let current;
        
        current = self.timer.read_counter();
        let trigger = (current + (Self::TPMS * ms)) % Self::MAX;
        trigger
    }

    /*
    fn set_compare(&mut self, ms: u32) -> u32 {
        //hprintln!("{:?}", self.queue);
        // Get the current counter value and calculate when the task will trigger.
        let current = self.timer.read_counter();
        let trigger = (current + (Self::TPMS * ms)) % Self::MAX;

        // Set up the CC0 compare register to interrupt when the next task is due.
        match self.queue.peek() {
            Some(old_task) => {
                // If a task exists, there are a number of possibilities:
                // 1: counter < new_task < old_task
                //   new_task comes sooner ----------- update required
                // 2: counter < old_task < new_task
                //   old_task comes sooner
                // 3: old_task < counter < new_task
                //   new_task comes sooner ----------- update required
                // 4: new_task < counter < old_task
                //   old_task comes sooner
                // 5: new_task < old_task < counter
                //   new_task comes sooner ----------- update required
                // 6: old_task < new_task < counter
                //   old_task comes sooner

                // The first condition matches 1, the second 3, and the third 5
                if (current < trigger && trigger < old_task.trigger)
                    || (old_task.trigger < current && current < trigger)
                    || (trigger < old_task.trigger && old_task.trigger < current)
                {
                    // Here, we need to update the comparison value to trigger
                    // an interrupt sooner than the tasks already in the queue.
                    self.timer.set_compare_register(RtcCc::CC0, trigger);
                }
            }
            None => {
                // If no other task exists, the queue is empty. We should set
                // the compare and enable the interrupt.
                self.timer.set_compare_register(RtcCc::CC0, trigger);
                self.timer.clear_compare_event(RtcCc::CC0);
                self.timer.enable_compare_interrupt(RtcCc::CC0);
            }
        }

        trigger
    }
    */
}

pub struct Task {
    pub trigger: u32,
    pub func: for<'r> fn(&'r mut super::context::Context) -> (),
    pub kind: TaskKind,
}

impl core::fmt::Debug for Task {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Task")
            .field("trigger", &self.trigger)
            .field("kind", &self.kind)
            .finish()
    }
}

#[derive(Debug)]
pub enum TaskKind {
    Single,
    Recurring(u32),
}

impl core::cmp::PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.trigger == other.trigger
    }
}

impl core::cmp::PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.trigger.partial_cmp(&other.trigger)
    }
}

impl core::cmp::Eq for Task {}

impl core::cmp::Ord for Task {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.trigger.cmp(&other.trigger)
    }
}
