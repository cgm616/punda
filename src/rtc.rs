use core::cell::RefCell;
use core::ops::{Deref, DerefMut};
use cortex_m::interrupt::Mutex;
use nrf51::RTC1;

crate static SCHEDULER: Mutex<RefCell<Option<Scheduler>>> = Mutex::new(RefCell::new(None));

#[derive(Copy, Clone)]
crate enum RTCInterrupt {
    Tick,
    Overflow,
    Compare0,
    Compare1,
    Compare2,
    Compare3,
}

impl RTCInterrupt {
    crate fn compare_from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(RTCInterrupt::Compare0),
            1 => Some(RTCInterrupt::Compare1),
            2 => Some(RTCInterrupt::Compare2),
            3 => Some(RTCInterrupt::Compare3),
            _ => None,
        }
    }
}

crate type Handler = fn(u32) -> ();

crate struct Scheduler {
    timer: RTC1,
    tick: Option<Handler>,
    overflow: Option<Handler>,
    compare: [Option<Handler>; 4],
}

impl Scheduler {
    fn new(timer: RTC1) -> Self {
        Scheduler {
            timer: timer,
            tick: None,
            overflow: None,
            compare: [None; 4],
        }
    }

    crate fn get_interrupt(&self, interrupt: RTCInterrupt) -> Option<Handler> {
        match interrupt {
            RTCInterrupt::Tick => self.tick,
            RTCInterrupt::Overflow => self.overflow,
            RTCInterrupt::Compare0 => self.compare[0],
            RTCInterrupt::Compare1 => self.compare[1],
            RTCInterrupt::Compare2 => self.compare[2],
            RTCInterrupt::Compare3 => self.compare[3],
        }
    }

    crate fn set_agnostic_interrupt(
        &mut self,
        interrupt: RTCInterrupt,
        f: Handler,
    ) -> Result<(), ()> {
        // TODO: turn on event
        match self.get_interrupt(interrupt) {
            Some(_) => Err(()),
            None => {
                match interrupt {
                    RTCInterrupt::Tick => {
                        self.tick = Some(f);
                        self.timer.evtenset.write(|w| w.tick().set_bit());
                    }
                    RTCInterrupt::Overflow => {
                        self.overflow = Some(f);
                        self.timer.evtenset.write(|w| w.ovrflw().set_bit());
                    }
                    _ => return Err(()),
                }

                Ok(())
            }
        }
    }

    crate fn set_cmp_interrupt(
        &mut self,
        interrupt: RTCInterrupt,
        f: Handler,
        compare: u32,
    ) -> Result<(), ()> {
        match self.get_interrupt(interrupt) {
            Some(_) => Err(()),
            None => {
                match interrupt {
                    RTCInterrupt::Compare0 => {
                        // TODO: set compare
                        self.compare[0] = Some(f);
                        self.timer.cc[0].write(|w| unsafe { w.compare().bits(compare) });
                        self.timer.evtenset.write(|w| w.compare0().set_bit());
                    }
                    RTCInterrupt::Compare1 => {
                        self.compare[1] = Some(f);
                        self.timer.cc[1].write(|w| unsafe { w.compare().bits(compare) });
                        self.timer.evtenset.write(|w| w.compare1().set_bit());
                    }
                    RTCInterrupt::Compare2 => {
                        self.compare[2] = Some(f);
                        self.timer.cc[2].write(|w| unsafe { w.compare().bits(compare) });
                        self.timer.evtenset.write(|w| w.compare2().set_bit());
                    }
                    RTCInterrupt::Compare3 => {
                        self.compare[3] = Some(f);
                        self.timer.cc[3].write(|w| unsafe { w.compare().bits(compare) });
                        self.timer.evtenset.write(|w| w.compare3().set_bit());
                    }
                    _ => return Err(()),
                }
                Ok(())
            }
        }
    }

    crate fn unset_interrupt(&mut self, interrupt: RTCInterrupt) {
        match interrupt {
            RTCInterrupt::Tick => {
                self.tick = None;
                self.timer.evtenclr.write(|w| w.tick().set_bit());
            }
            RTCInterrupt::Overflow => {
                self.overflow = None;
                self.timer.evtenclr.write(|w| w.ovrflw().set_bit());
            }
            RTCInterrupt::Compare0 => {
                self.compare[0] = None;
                self.timer.evtenclr.write(|w| w.compare0().set_bit());
            }
            RTCInterrupt::Compare1 => {
                self.compare[1] = None;
                self.timer.evtenclr.write(|w| w.compare1().set_bit());
            }
            RTCInterrupt::Compare2 => {
                self.compare[2] = None;
                self.timer.evtenclr.write(|w| w.compare2().set_bit());
            }
            RTCInterrupt::Compare3 => {
                self.compare[3] = None;
                self.timer.evtenclr.write(|w| w.compare3().set_bit());
            }
        }
    }

    crate fn set_available_compare(&mut self, f: Handler, compare: u32) -> Result<(), ()> {
        for i in 0..4 {
            if self.compare[i].is_none() {
                self.set_cmp_interrupt(RTCInterrupt::compare_from_index(i).unwrap(), f, compare)
                    .unwrap();
                return Ok(());
            }
        }

        Err(())
    }

    crate fn event_fired(&self, interrupt: RTCInterrupt) -> bool {
        match interrupt {
            RTCInterrupt::Tick => self.timer.events_tick.read().bits() == 1,
            RTCInterrupt::Overflow => self.timer.events_ovrflw.read().bits() == 1,
            RTCInterrupt::Compare0 => self.events_compare[0].read().bits() == 1,
            RTCInterrupt::Compare1 => self.timer.events_compare[1].read().bits() == 1,
            RTCInterrupt::Compare2 => self.timer.events_compare[2].read().bits() == 1,
            RTCInterrupt::Compare3 => self.timer.events_compare[3].read().bits() == 1,
        }
    }

    crate fn clear_event(&mut self, interrupt: RTCInterrupt) {
        match interrupt {
            RTCInterrupt::Tick => self.timer.events_tick.write(|w| unsafe { w.bits(0) }),
            RTCInterrupt::Overflow => self.timer.events_ovrflw.write(|w| unsafe { w.bits(0) }),
            RTCInterrupt::Compare0 => self.timer.events_compare[0].write(|w| unsafe { w.bits(0) }),
            RTCInterrupt::Compare1 => self.timer.events_compare[1].write(|w| unsafe { w.bits(0) }),
            RTCInterrupt::Compare2 => self.timer.events_compare[2].write(|w| unsafe { w.bits(0) }),
            RTCInterrupt::Compare3 => self.timer.events_compare[3].write(|w| unsafe { w.bits(0) }),
        }
    }

    crate fn current_counter(&self) -> u32 {
        self.timer.counter.read().counter().bits()
    }
}

impl Deref for Scheduler {
    type Target = RTC1;

    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl DerefMut for Scheduler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

crate fn init_scheduler(timer: RTC1) {
    let scheduler = Scheduler::new(timer);

    scheduler.prescaler.write(|w| unsafe { w.bits(163) });
    scheduler.intenset.write(|w| {
        w.tick()
            .set_bit()
            .ovrflw()
            .set_bit()
            .compare0()
            .set_bit()
            .compare1()
            .set_bit()
            .compare2()
            .set_bit()
            .compare3()
            .set_bit()
    });

    scheduler.tasks_start.write(|w| unsafe { w.bits(1) });

    cortex_m::interrupt::free(|cs| {
        *SCHEDULER.borrow(cs).borrow_mut() = Some(scheduler);
    });
}

const PRIORITY: [RTCInterrupt; 6] = [
    RTCInterrupt::Tick,
    RTCInterrupt::Overflow,
    RTCInterrupt::Compare0,
    RTCInterrupt::Compare1,
    RTCInterrupt::Compare2,
    RTCInterrupt::Compare3,
];

crate fn handler() {
    let mut counter = 0;
    let mut handler = None;
    // TODO: make CS shorter
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut scheduler) = SCHEDULER.borrow(cs).borrow_mut().deref_mut() {
            counter = scheduler.current_counter();

            for interrupt in PRIORITY.iter() {
                if scheduler.event_fired(*interrupt) {
                    scheduler
                        .get_interrupt(*interrupt)
                        .map(|f| handler = Some(f));
                    scheduler.clear_event(*interrupt);
                    return;
                }
            }
        }
    });

    handler.map(|f| f(counter));
}
