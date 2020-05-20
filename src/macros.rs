#[macro_export]
macro_rules! punda {
    (init = $init_path:ident $(, idle = $idle_path:ident)? $(, button_handler = $button_path:ident)?) => {
        use cortex_m_semihosting::{debug, hprintln};
        use heapless::{consts::*, spsc};
        use microbit::{hal::{nrf51,
                delay::DelayTimer,
                hi_res_timer::TimerFrequency::Freq62500Hz,
                lo_res_timer::{FREQ_256HZ, LoResTimer},
            },
            display::{MicrobitDisplayTimer, MicrobitFrame}
        };
        use punda::{
            context::UserContext,
            display::{DisplayBackend},
            syscall::{Consumer, Producer, Queue, Syscall},
            serial::UART0Buffer,
            button::{History, State as _State, Button as _Button}
        };
        use core::fmt::Write;
        use rtfm::app;

        #[app(device = microbit::hal::nrf51, peripherals = true)]
        const APP: () = {
            struct Resources {
                producer: Producer,
                consumer: Consumer,
                display_timer: MicrobitDisplayTimer<nrf51::TIMER1>,
                display: DisplayBackend,
                user_timer: DelayTimer<nrf51::TIMER0>,
                general_timer: LoResTimer<nrf51::RTC0>,
                gpio: nrf51::GPIO,
                uart: nrf51::UART0,
            }

            #[init(spawn = [__user_init, __user_idle])]
            fn __init(cx: __init::Context) -> __init::LateResources {
                static mut queue: Queue = spsc::Queue(heapless::i::Queue::u8());

                let mut p: nrf51::Peripherals = cx.device;

                p.CLOCK.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
                while p.CLOCK.events_lfclkstarted.read().bits() == 0 {}
                p.CLOCK.events_lfclkstarted.reset();

                let (mut producer, mut consumer) = queue.split();

                let mut display_timer = MicrobitDisplayTimer::new(p.TIMER1);
                microbit::display::initialise_display(&mut display_timer, &mut p.GPIO);

                let mut display = DisplayBackend::new();

                let mut user_timer = DelayTimer::new(p.TIMER0, Freq62500Hz);

                let mut general_timer = LoResTimer::new(p.RTC0);
                general_timer.set_frequency(FREQ_256HZ);
                general_timer.enable_tick_event();
                general_timer.clear_tick_event();
                general_timer.enable_tick_interrupt();
                general_timer.start();

                p.GPIO.pin_cnf[24].write(|w| w.pull().pullup().dir().output());
                p.GPIO.pin_cnf[25].write(|w| w.pull().disabled().dir().input());

                p.UART0.pseltxd.write(|w| unsafe { w.bits(24) });
                p.UART0.pselrxd.write(|w| unsafe { w.bits(25) });

                p.UART0.baudrate.write(|w| w.baudrate().baud115200());
                p.UART0.enable.write(|w| w.enable().enabled());

                p.GPIO.pin_cnf[17].write(|w| w.dir().input().drive().s0s1().pull().disabled().sense().disabled().input().connect());
                p.GPIO.pin_cnf[26].write(|w| w.dir().input().drive().s0s1().pull().disabled().sense().disabled().input().connect());

                /*
                p.GPIOTE.config[0].write(|w| unsafe { w.mode().event().psel().bits(17).polarity().hi_to_lo() });
                p.GPIOTE.intenset.write(|w| w.in0().set_bit());
                p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });

                p.GPIOTE.config[1].write(|w| unsafe { w.mode().event().psel().bits(17).polarity().lo_to_hi() });
                p.GPIOTE.intenset.write(|w| w.in0().set_bit());
                p.GPIOTE.events_in[0].write(|w| unsafe { w.bits(0) });

                p.GPIOTE.config[1].write(|w| unsafe { w.mode().event().psel().bits(26).polarity().hi_to_lo() });
                p.GPIOTE.intenset.write(|w| w.in1().set_bit());
                p.GPIOTE.events_in[1].write(|w| unsafe { w.bits(0) });
                */

                cx.spawn.__user_init().expect("can't spawn __user_init");

                cx.spawn.__user_idle().expect("can't spawn __user_idle");

                __init::LateResources {
                    producer,
                    consumer,
                    display_timer,
                    display,
                    user_timer,
                    general_timer,
                    gpio: p.GPIO,
                    uart: p.UART0,
                }
            }

            #[task(priority = 1, resources = [producer, user_timer])]
            fn __user_init(cx: __user_init::Context) {
                let mut user_context = UserContext {
                    _producer: cx.resources.producer,
                    _timer: cx.resources.user_timer,
                };
                let f: for<'r> fn(&'r mut UserContext) -> () = $init_path;
                f(&mut user_context);
            }

            #[task(priority = 1, resources = [producer, user_timer])]
            fn __user_idle(cx: __user_idle::Context) {
                let mut user_context = UserContext {
                    _producer: cx.resources.producer,
                    _timer: cx.resources.user_timer
                };

                $(
                    let f: for<'r> fn(&'r mut UserContext) -> ! = $idle_path;
                    f(&mut user_context);
                )?

                loop {}
            }

            #[task(binds = TIMER1, priority = 4, resources = [gpio, display_timer, display])]
            fn __refresh_display(mut cx: __refresh_display::Context) {
                microbit::display::handle_display_event(
                    &mut cx.resources.display,
                    cx.resources.display_timer,
                    cx.resources.gpio,
                );
            }

            #[task(binds = SWI5, priority = 2, resources = [consumer, display, display_timer, gpio, uart])]
            fn __syscall(mut cx: __syscall::Context) {
                while let Some(call) = cx.resources.consumer.dequeue() {
                    match call {
                        Syscall::Empty => {}
                        Syscall::StartDisplay(frame) => {
                            cx.resources.display.lock(|display| {
                                display.set_frame(&frame);
                            });
                        }
                        Syscall::StopDisplay => {
                            let frame: MicrobitFrame = MicrobitFrame::const_default();
                            cx.resources.display.lock(|display| {
                                display.set_frame(&frame);
                            });
                        }
                        Syscall::SerialPrint(string) => {
                            let _ = write!(
                                UART0Buffer(&cx.resources.uart),
                                "{}", string
                            );
                        }
                    }
                }
            }

            $(
                #[task(binds = RTC0, priority = 3, resources = [gpio, general_timer], spawn = [__button_handler])]
                fn __general_timer(mut cx: __general_timer::Context) {
                    static mut A: History = History::new_released();
                    static mut B: History = History::new_released();

                    cx.resources.general_timer.clear_tick_event();

                    let mut a_low = false;
                    let mut b_low = false;
                    cx.resources.gpio.lock(|gpio| {
                        a_low = gpio.in_.read().pin17().is_low();
                        b_low = gpio.in_.read().pin26().is_low();
                    });

                    let a_change = A.measure(a_low);
                    let b_change = B.measure(b_low);

                    if a_change {
                        cx.spawn.__button_handler(_Button::A, A.state).unwrap();
                    }

                    if b_change {
                        cx.spawn.__button_handler(_Button::B, B.state).unwrap();
                    }

                    if a_change && b_change && (A.state == B.state) {
                        cx.spawn.__button_handler(_Button::Both, A.state).unwrap();
                    }
                }

                #[task(priority = 1, resources = [producer, user_timer], capacity = 6)]
                fn __button_handler(mut cx: __button_handler::Context, button: _Button, direction: _State) {
                    let mut user_context = UserContext {
                        _producer: cx.resources.producer,
                        _timer: cx.resources.user_timer
                    };

                    let f: for<'r> fn(&'r mut UserContext, _Button, _State) -> () = $button_path;
                    f(&mut user_context, button, direction);
                }
            )?

            extern "C" {
                fn SWI0();
                fn SWI1();
                fn SWI2();
                fn SWI3();
                fn SWI4();
            }
        };
    };
}
