//! A basic hal-level interface to a timer's timer mode.
//!
//! Note that this module kind of by-passes [`nrf52480_hal`'s `timer` module](https://docs.rs/nrf52840-hal/latest/nrf52840_hal/timer/index.html)

use nrf52840_hal::timer::Instance;

use core::marker::PhantomData;

pub mod bitmode;
pub mod prescaler;

// -----------------------------
// Mode dependent Configurations
// -----------------------------

use crate::timer::prescaler::{Prescaler, P0};

/// Type indicating a timer running in counter mode.
pub struct CounterMode;

/// Type indicating a timer running in timer mode.
pub struct TimerMode<P: Prescaler> {
    prescaler: PhantomData<P>,
}

// -----
// State
// -----

/// Type indicating a running timer.
pub struct Started;

/// Type indicating a not-running timer.
pub struct Stopped;

// ----------
// Interrupts
// ----------

/// Type indicating a timer triggering interrupts.
pub struct Enabled;

/// Type indicating a timer not triggering interrupts.
pub struct Disabled;

/// HAL-level interface to timer peripheral.
use crate::timer::bitmode::{Width, W32};

pub struct Timer<T: Instance, S, W: Width, I, C> {
    timer: T,
    w: PhantomData<W>,
    s: PhantomData<S>,
    i: PhantomData<I>,
    c: PhantomData<C>,
}

impl<T> Timer<T, Stopped, W32, Disabled, TimerMode<P0>>
where
    T: Instance,
{
    /// Conversion function to turn a PAC-level timer interface into a HAL-level one.
    pub fn timer(timer: T) -> Timer<T, Stopped, W32, Disabled, TimerMode<P0>> {
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

        Timer {
            timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T> Timer<T, Stopped, W32, Disabled, CounterMode>
where
    T: Instance,
{
    pub fn counter(timer: T) -> Timer<T, Stopped, W32, Disabled, CounterMode> {
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

        Timer {
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
    pub fn tick(&mut self) {
        self.timer
            .as_timer0()
            .tasks_count
            .write(|w| w.tasks_count().set_bit());
    }
}

impl<T, S, W, C> Timer<T, S, W, Enabled, C>
where
    T: Instance,
    W: Width,
{
    /// Disable interrupt 0 for timer.
    pub fn disable_interrupt(self) -> Timer<T, S, W, Disabled, C> {
        self.timer
            .as_timer0()
            .intenclr
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

impl<T, S, W, C> Timer<T, S, W, Disabled, C>
where
    T: Instance,
    W: Width,
{
    /// Enable interrupt 0 for timer.
    pub fn enable_interrupt(self) -> Timer<T, S, W, Enabled, C> {
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
    /// See Nordic's documentation on `CC[0]` register for details.
    pub fn compare_against(&mut self, val: u32) {
        self.timer.as_timer0().cc[0].write(|w| unsafe { w.cc().bits(val) });
    }

    /// Clear/Reset the timer.
    ///
    /// This works both in `Started` as well as in `Stopped` state.
    /// See Nordic's documentation on `TASKS_CLEAR` for details.
    pub fn reset(&mut self) {
        self.timer
            .as_timer0()
            .tasks_clear
            .write(|w| w.tasks_clear().set_bit());
    }
}
