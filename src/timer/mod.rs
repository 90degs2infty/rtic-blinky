//! A basic hal-level interface to the timer peripheral.
//!
//! Note that this module kind of by-passes [`nrf52480_hal`'s `timer` module](https://docs.rs/nrf52840-hal/latest/nrf52840_hal/timer/index.html)
//!
//! See [Nordic's docs](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29) for a general overview of the underlying hardware.

pub mod bitmode;
pub mod interrupts;
pub mod mode;
pub mod prescaler;
pub mod state;

use core::marker::PhantomData;
use nrf52840_hal::timer::Instance;

use crate::timer::{
    bitmode::{Width, W32},
    interrupts::{Disabled, Enabled, Interrupt},
    mode::{Counter as CounterMode, Timer as TimerMode},
    prescaler::{Prescaler, P0},
    state::{Started, Stopped},
};

/// HAL-level interface to timer peripheral.
pub struct Timer<T: Instance, S, W: Width, I, C> {
    timer: T,
    w: PhantomData<W>,
    s: PhantomData<S>,
    i: PhantomData<I>,
    c: PhantomData<C>,
}

type IDisabled = Interrupt<Disabled, Disabled, Disabled, Disabled>;

impl<T> Timer<T, Stopped, W32, IDisabled, TimerMode<P0>>
where
    T: Instance,
{
    /// Conversion function to turn a PAC-level timer interface into a
    /// HAL-level timer running in timer mode.
    pub fn timer(timer: T) -> Self {
        // Make sure the timer is stopped
        timer
            .as_timer0()
            .tasks_stop
            .write(|w| w.tasks_stop().set_bit());

        // Set bit width
        timer.as_timer0().bitmode.write(|w| W32::set(w));

        // Disable and clear interrupts
        timer.as_timer0().intenclr.write(|w| w.compare0().set_bit());
        timer.as_timer0().events_compare[0].write(|w| w);

        // Set timer mode
        timer.as_timer0().mode.write(|w| w.mode().timer());

        // Set prescale value
        timer
            .as_timer0()
            .prescaler
            .write(|w| unsafe { w.bits(P0::VAL) });

        Self {
            timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T> Timer<T, Stopped, W32, IDisabled, CounterMode>
where
    T: Instance,
{
    /// Constructor to turn a PAC-level timer peripheral into a HAL-level timer
    /// running in counter mode.
    pub fn counter(timer: T) -> Self {
        // Make sure the timer is stopped
        timer
            .as_timer0()
            .tasks_stop
            .write(|w| w.tasks_stop().set_bit());

        // Set bit width
        timer.as_timer0().bitmode.write(|w| W32::set(w));

        // Disable and clear interrupts
        timer.as_timer0().intenclr.write(|w| w.compare0().set_bit());
        timer.as_timer0().events_compare[0].write(|w| w);

        // Set counter mode
        timer.as_timer0().mode.write(|w| w.mode().counter());

        Self {
            timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T, W, I, C> Timer<T, Stopped, W, I, C>
where
    T: Instance,
    W: Width,
{
    /// Set a timer's bit with.
    ///
    /// See `Width` for details.
    pub fn set_counterwidth<W2: Width>(self) -> Timer<T, Stopped, W2, I, C> {
        self.timer.as_timer0().bitmode.write(|w| W2::set(w));
        Timer {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }

    /// Start a timer.
    pub fn start(self) -> Timer<T, Started, W, I, C> {
        self.timer
            .as_timer0()
            .tasks_start
            .write(|w| w.tasks_start().set_bit());
        Timer {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T, W, I, P> Timer<T, Stopped, W, I, TimerMode<P>>
where
    T: Instance,
    W: Width,
    P: Prescaler,
{
    /// Set a timer's prescale value.
    ///
    /// See `Prescaler` for details.
    pub fn set_prescale<P2: Prescaler>(self) -> Timer<T, Stopped, W, I, TimerMode<P2>> {
        self.timer
            .as_timer0()
            .prescaler
            .write(|w| unsafe { w.bits(P2::VAL) });
        Timer {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T, W, I, C> Timer<T, Started, W, I, C>
where
    T: Instance,
    W: Width,
{
    /// Stop a timer.
    pub fn stop(self) -> Timer<T, Stopped, W, I, C> {
        self.timer
            .as_timer0()
            .tasks_stop
            .write(|w| w.tasks_stop().set_bit());
        Timer {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T, W, I> Timer<T, Started, W, I, CounterMode>
where
    T: Instance,
    W: Width,
{
    /// Increase this counter's value by one.
    ///
    /// Note that increasing the counter's value may cause it to overflow, in
    /// which case the counter starts counting from zero again.
    pub fn tick(&mut self) {
        self.timer
            .as_timer0()
            .tasks_count
            .write(|w| w.tasks_count().set_bit());
    }
}

macro_rules! define_disabled_type_0 {
    () => {
        Interrupt<Disabled, IA, IB, IC>
    };
}

macro_rules! define_disabled_type_1 {
    () => {
        Interrupt<IA, Disabled, IB, IC>
    };
}

macro_rules! define_disabled_type_2 {
    () => {
        Interrupt<IA, IB, Disabled, IC>
    };
}

macro_rules! define_disabled_type_3 {
    () => {
        Interrupt<IA, IB, IC, Disabled>
    };
}

macro_rules! define_enabled_type_0 {
    () => {
        Interrupt<Enabled, IA, IB, IC>
    };
}

macro_rules! define_enabled_type_1 {
    () => {
        Interrupt<IA, Enabled, IB, IC>
    };
}

macro_rules! define_enabled_type_2 {
    () => {
        Interrupt<IA, IB, Enabled, IC>
    };
}

macro_rules! define_enabled_type_3 {
    () => {
        Interrupt<IA, IB, IC, Enabled>
    };
}

macro_rules! define_disable_interrupt {
    ( $num:literal ) => {
        paste::paste! {
            impl<T, S, W, IA, IB, IC, C> Timer<T, S, W, [< define_disabled_type_ $num >]!(), C>
            where
                T: Instance,
                W: Width,
            {
                #[doc = "Disable interrupt " [< $num >] " for this timer."]
                pub fn [< disable_interrupt_ $num >](self) -> Timer<T, S, W, [< define_enabled_type_ $num >]!(), C> {
                    self.timer
                        .as_timer0()
                        .intenclr
                        .write(|w| w.[< compare $num >]().set_bit());
                    Timer {
                        timer: self.timer,
                        s: PhantomData,
                        w: PhantomData,
                        i: PhantomData,
                        c: PhantomData,
                    }
                }
            }
        }
    };
}

define_disable_interrupt!(0);
define_disable_interrupt!(1);
define_disable_interrupt!(2);
define_disable_interrupt!(3);

impl<T, S, W, IA, IB, IC, C> Timer<T, S, W, define_disabled_type_0!(), C>
where
    T: Instance,
    W: Width,
{
    /// Enable interrupt 0 for timer.
    pub fn enable_interrupt(self) -> Timer<T, S, W, define_enabled_type_0!(), C> {
        self.timer
            .as_timer0()
            .intenset
            .write(|w| w.compare0().set_bit());
        Timer {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T, S, W, I, C> Timer<T, S, W, I, C>
where
    T: Instance,
    W: Width,
{
    /// Unpend interrupt 0 for timer.
    pub fn unpend_interrupt(&mut self) {
        self.timer.as_timer0().events_compare[0].write(|w| w.events_compare().clear_bit());
    }

    /// Set compare value 0 for timer.
    ///
    /// See [Nordic's documentation on `CC[0]`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_13#register.CC-0-5) register for details.
    pub fn compare_against(&mut self, val: u32) {
        self.timer.as_timer0().cc[0].write(|w| unsafe { w.cc().bits(val) });
    }

    /// Clear/Reset the timer.
    ///
    /// This works both in `Started` as well as in `Stopped` state.
    /// See [Nordic's documentation on `TASKS_CLEAR`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_3#register.TASKS_CLEAR) for details.
    pub fn reset(&mut self) {
        self.timer
            .as_timer0()
            .tasks_clear
            .write(|w| w.tasks_clear().set_bit());
    }
}
