#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use rtic_blinky as _; // global logger + panicking-behavior + memory layout

#[rtic::app(
    device = nrf52840_hal::pac,
    dispatchers = [SWI0_EGU0]
)]
mod app {
    use nrf52840_hal::{
        gpio::{p0::*, Level, Output, PushPull},
        pac::{NVIC, TIMER1, TIMER2},
        prelude::{StatefulOutputPin, *},
        timer::Instance,
    };

    use rtic_blinky::timer::{
        bitmode::{W08, W24},
        prescaler::P0,
        CounterMode, Enabled, Started, Timer, TimerMode,
    };

    use core::fmt::Debug;

    // Shared resources go here
    #[shared]
    struct Shared {
        counter: Timer<TIMER2, Started, W08, Enabled, CounterMode>,
        leds: (P0_13<Output<PushPull>>, P0_14<Output<PushPull>>),
        led_switch: bool,
    }

    // Local resources go here
    #[local]
    struct Local {
        timer: Timer<TIMER1, Started, W24, Enabled, TimerMode<P0>>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let p = cx.device;

        // Leds
        let p0 = Parts::new(p.P0);
        let led_1 = p0.p0_13.into_push_pull_output(Level::High);
        let led_2 = p0.p0_14.into_push_pull_output(Level::High);

        // Timer
        let mut timer = Timer::timer(p.TIMER1)
            .set_prescale::<P0>()
            .set_counterwidth::<W24>();

        // Counter
        let mut counter = Timer::counter(p.TIMER2).set_counterwidth::<W08>();

        // Interrupts
        timer.compare_against(0);
        let timer = timer.enable_interrupt();

        counter.compare_against(4);
        let counter = counter.enable_interrupt();

        unsafe {
            NVIC::unmask(TIMER1::INTERRUPT);
            NVIC::unmask(TIMER2::INTERRUPT);
        }

        let timer = timer.start();
        let counter = counter.start();
        (
            Shared {
                counter,
                leds: (led_1, led_2),
                led_switch: false,
            },
            Local { timer },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        #[allow(clippy::empty_loop)]
        loop {}
    }

    fn toggle_led<T: StatefulOutputPin>(led: &mut T)
    where
        <T as OutputPin>::Error: Debug,
    {
        if led.is_set_high().unwrap() {
            led.set_low().unwrap();
        } else {
            led.set_high().unwrap();
        }
    }

    #[task(binds = TIMER1, local = [timer], shared = [counter, leds, led_switch])]
    fn toggle(ctx: toggle::Context) {
        defmt::info!("toggle");

        let timer = ctx.local.timer;
        timer.unpend_interrupt();

        let ls = ctx.shared.led_switch;
        let leds = ctx.shared.leds;

        (ls, leds).lock(|ls, leds| {
            if *ls {
                toggle_led(&mut leds.1);
            } else {
                toggle_led(&mut leds.0);
            };
        });

        let mut counter = ctx.shared.counter; // why do I need mut here?
        counter.lock(|counter| {
            counter.tick();
        });
    }

    #[task(binds = TIMER2, shared = [counter, leds, led_switch])]
    fn count(ctx: count::Context) {
        defmt::info!("count");

        let leds = ctx.shared.leds;
        let ls = ctx.shared.led_switch;
        let counter = ctx.shared.counter;

        (counter, ls, leds).lock(|counter, ls, leds| {
            // Reset counter
            counter.unpend_interrupt();
            counter.reset();

            // Turn off all leds to start from a known state again
            leds.0.set_high().unwrap();
            leds.1.set_high().unwrap();

            // Switch led
            *ls = !*ls;
        });
    }
}
