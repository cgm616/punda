#[macro_export]
macro_rules! punda {
    (init: $path:ident) => {
        use cortex_m_semihosting::{debug, hprintln};
        use heapless::{consts::*, spsc};
        use microbit::{hal::nrf51, display::{MicrobitDisplayTimer, MicrobitFrame}};
        use punda::{
            context::UserContext,
            display::{DisplayBackend},
            syscall::{Consumer, Producer, Queue, Syscall},
            serial::UART0Buffer,
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
                gpio: nrf51::GPIO,
                uart: nrf51::UART0,
            }

            #[init(spawn = [__user_init])]
            fn __init(cx: __init::Context) -> __init::LateResources {
                static mut queue: Queue = spsc::Queue(heapless::i::Queue::u8());

                let mut p: nrf51::Peripherals = cx.device;

                let (mut producer, mut consumer) = queue.split();

                let mut display_timer = MicrobitDisplayTimer::new(p.TIMER1);
                microbit::display::initialise_display(&mut display_timer, &mut p.GPIO);

                let mut display = DisplayBackend::new();

                p.GPIO.pin_cnf[24].write(|w| w.pull().pullup().dir().output());
                p.GPIO.pin_cnf[25].write(|w| w.pull().disabled().dir().input());

                p.UART0.pseltxd.write(|w| unsafe { w.bits(24) });
                p.UART0.pselrxd.write(|w| unsafe { w.bits(25) });

                p.UART0.baudrate.write(|w| w.baudrate().baud115200());
                p.UART0.enable.write(|w| w.enable().enabled());

                cx.spawn.__user_init().unwrap();

                __init::LateResources {
                    producer,
                    consumer,
                    display_timer,
                    display,
                    gpio: p.GPIO,
                    uart: p.UART0,
                }
            }

            #[task(resources = [producer])]
            fn __user_init(cx: __user_init::Context) {
                let mut user_context = UserContext {
                    _producer: cx.resources.producer,
                };
                let f: for<'r> fn(&'r mut UserContext) -> () = $path;
                f(&mut user_context);
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
