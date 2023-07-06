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
        pac::{NVIC, TIMER1},
        prelude::*,
        timer::Instance,
    };

    use rtic_blinky::timer::{Enabled, Periodic, Started, Timer, TwentyFour, U0};

    // Shared resources go here
    #[shared]
    struct Shared {
        // TODO: Add resources
    }

    // Local resources go here
    #[local]
    struct Local {
        led: P0_13<Output<PushPull>>,
        timer: Timer<TIMER1, Started, TwentyFour, Enabled, Periodic<U0>>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        let p = cx.device;

        let p0 = Parts::new(p.P0);
        let led = p0.p0_13.into_push_pull_output(Level::High);

        // Timer
        let mut timer = Timer::periodic(p.TIMER1)
            .set_prescale::<U0>()
            .set_counterwidth::<TwentyFour>();

        // Interrupts
        timer.compare_against(0);
        let timer = timer.enable_interrupt();
        unsafe { NVIC::unmask(TIMER1::INTERRUPT) }

        let timer = timer.start();
        (Shared {}, Local { led, timer })
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }

    #[task(binds = TIMER1, local = [led, timer])]
    fn toggle_led(ctx: toggle_led::Context) {
        defmt::info!("toggle_led");

        let timer = ctx.local.timer;
        timer.unpend_interrupt();

        let led = ctx.local.led;

        if led.is_set_high().unwrap() {
            led.set_low().unwrap();
        } else {
            led.set_high().unwrap();
        }
    }
}
