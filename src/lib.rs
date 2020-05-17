#![no_std]

extern crate panic_semihosting;

#[macro_export]
macro_rules! punda {
    (init: $path:ident) => {
        use cortex_m_semihosting::{debug, hprintln};
        use rtfm::app;
        use microbit::hal::{lo_res_timer::{LoResTimer, FREQ_16HZ}, nrf51};

        #[app(device = microbit::hal::nrf51, peripherals = true)]
        const APP: () = {
            struct Resources {
                timer: LoResTimer<nrf51::RTC0>
            }

            #[init]
            fn __init(cx: __init::Context) -> __init::LateResources {
                let mut p: nrf51::Peripherals = cx.device;

                // Starting the low-frequency clock (needed for RTC to work)
                p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
                while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
                p.CLOCK.events_lfclkstarted.reset();

                let mut rtc0 = LoResTimer::new(p.RTC0);
                // 16Hz; 62.5ms period
                rtc0.set_frequency(FREQ_16HZ);
                rtc0.enable_tick_event();
                rtc0.enable_tick_interrupt();
                rtc0.start();

                let f: fn() -> () = $path;
                f();

                __init::LateResources { timer: rtc0 }
            }

            #[task(binds = RTC0, priority = 1, resources = [timer])]
            fn __rtc0(cx: __rtc0::Context) {
                static mut CALLED: u32 = 0;

                &cx.resources.timer.clear_tick_event();

                hprintln!("Called {} times so far", CALLED);

                *CALLED = CALLED.wrapping_add(1);
            }

            #[idle]
            fn __idle(cx: __idle::Context) -> ! {
                loop{}
            }
        };
    };
}