//! A basic hal-level interface to a timer's timer mode.
//!
//! Note that this module kind of by-passes [`nrf52480_hal`'s `timer` module](https://docs.rs/nrf52840-hal/latest/nrf52840_hal/timer/index.html)

use nrf52840_hal::{timer::Instance, pac::timer0::bitmode::W};

use core::marker::PhantomData;

// Inputs:
// - Prescaler:
//   - four bit to divide 16MHz input (four bit = 0..15)
//   - frequency of timer (in timer mode) is 16MHz / 2 ^ prescale
// - Mode: Periodic, Counter
// - State: Running, Stopped/Cancelled/Inactive
// - Capture: ?
// - Clear: ?
// - Interrupts:
// - PPI stuff
// - Overflow value: bit width (BITMODE register)
//   - 0 - 16bit
//   - 1 - 8bit
//   - 2 - 24bit
//   - 3 - 32bit

// Outputs:
// - Compare

// Invariants
// - Change Prescaler and bit with in stopped state only

// ---------
// Prescaler
// ---------

/// Type encoding a prescale value of 0.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U0;

/// Type encoding a prescale value of 1.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U1;

/// Type encoding a prescale value of 2.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U2;

/// Type encoding a prescale value of 3.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U3;

/// Type encoding a prescale value of 4.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U4;

/// Type encoding a prescale value of 5.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U5;

/// Type encoding a prescale value of 6.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U6;

/// Type encoding a prescale value of 7.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U7;

/// Type encoding a prescale value of 8.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U8;

/// Type encoding a prescale value of 9.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U9;

/// Type encoding a prescale value of 10.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U10;

/// Type encoding a prescale value of 11.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U11;

/// Type encoding a prescale value of 12.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U12;

/// Type encoding a prescale value of 13.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U13;

/// Type encoding a prescale value of 14.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U14;

/// Type encoding a prescale value of 15.
///
/// See Nordic's docs on the `PRESCALER` register for details.
pub struct U15;

/// Common interface to all prescale values
pub trait Prescaler {

    /// The eventual value that gets written to the `PRESCALE` register.
    const VAL: u32;
}

impl Prescaler for U0 {
    const VAL: u32 = 0;
}

impl Prescaler for U1 {
    const VAL: u32 = 1;
}

impl Prescaler for U2 {
    const VAL: u32 = 2;
}

impl Prescaler for U3 {
    const VAL: u32 = 3;
}

impl Prescaler for U4 {
    const VAL: u32 = 4;
}

impl Prescaler for U5 {
    const VAL: u32 = 5;
}

impl Prescaler for U6 {
    const VAL: u32 = 6;
}

impl Prescaler for U7 {
    const VAL: u32 = 7;
}

impl Prescaler for U8 {
    const VAL: u32 = 8;
}

impl Prescaler for U9 {
    const VAL: u32 = 9;
}

impl Prescaler for U10 {
    const VAL: u32 = 10;
}

impl Prescaler for U11 {
    const VAL: u32 = 11;
}

impl Prescaler for U12 {
    const VAL: u32 = 12;
}

impl Prescaler for U13 {
    const VAL: u32 = 13;
}

impl Prescaler for U14 {
    const VAL: u32 = 14;
}

impl Prescaler for U15 {
    const VAL: u32 = 15;
}

// -----------------
// Counter bit width
// -----------------

/// Type indicating an eight bit wide timer.
///
/// See Nordic's docs on the `BITMODE` register for details.
pub struct Eight;

/// Type indicating an sixteen bit wide timer.
///
/// See Nordic's docs on the `BITMODE` register for details.
pub struct Sixteen;

/// Type indicating an twentyfour bit wide timer.
///
/// See Nordic's docs on the `BITMODE` register for details.
pub struct TwentyFour;

/// Type indicating an thirtytwo bit wide timer.
///
/// See Nordic's docs on the `BITMODE` register for details.
pub struct ThirtyTwo;

/// Common interface to all bitmodes.
pub trait Width {
    fn set(w: &mut W) -> &mut W;
}

impl Width for Eight {
    fn set(w: &mut W) -> &mut W {
        w.bitmode()._08bit()
    }
}

impl Width for Sixteen {
    fn set(w: &mut W) -> &mut W {
        w.bitmode()._16bit()
    }
}

impl Width for TwentyFour {
    fn set(w: &mut W) -> &mut W {
        w.bitmode()._24bit()
    }
}

impl Width for ThirtyTwo {
    fn set(w: &mut W) -> &mut W {
        w.bitmode()._32bit()
    }
}

// -----------------------------
// Mode dependent Configurations
// -----------------------------

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
pub struct Timer<T: Instance, S, W: Width, I, C> {
    timer: T,
    w: PhantomData<W>,
    s: PhantomData<S>,
    i: PhantomData<I>,
    c: PhantomData<C>,
}

impl<T> Timer<T, Stopped, ThirtyTwo, Disabled, TimerMode<U0>>
where
    T: Instance,
{
    /// Conversion function to turn a PAC-level timer interface into a HAL-level one.
    pub fn timer(timer: T) -> Timer<T, Stopped, ThirtyTwo, Disabled, TimerMode<U0>> {
        // Set timer mode
        timer.as_timer0().mode.write(|w| w.mode().timer());

        // Set prescale value
        timer
            .as_timer0()
            .prescaler
            .write(|w| unsafe { w.bits(U0::VAL) });

        // Set bit width
        timer.as_timer0().bitmode.write(|w| ThirtyTwo::set(w));

        // Disable and clear interrupts
        timer.as_timer0().intenclr.write(|w| w.compare0().set_bit());
        timer.as_timer0().events_compare[0].write(|w| w);

        Timer::<T, Stopped, ThirtyTwo, Disabled, TimerMode<U0>> {
            timer,
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
        Timer::<T, Stopped, W, I, TimerMode<P2>> {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }

    /// Set a timer's bit with.
    ///
    /// See `Width` for details.
    pub fn set_counterwidth<W2: Width>(self) -> Timer<T, Stopped, W2, I, TimerMode<P>> {
        self.timer.as_timer0().bitmode.write(|w| W2::set(w));
        Timer::<T, Stopped, W2, I, TimerMode<P>> {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }

    /// Start a timer.
    pub fn start(self) -> Timer<T, Started, W, I, TimerMode<P>> {
        self.timer
            .as_timer0()
            .tasks_start
            .write(|w| w.tasks_start().set_bit());
        Timer::<T, Started, W, I, TimerMode<P>> {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T, W, I, P> Timer<T, Started, W, I, TimerMode<P>>
where
    T: Instance,
    W: Width,
    P: Prescaler,
{
    /// Stop a timer.
    pub fn stop(self) -> Timer<T, Stopped, W, I, TimerMode<P>> {
        self.timer
            .as_timer0()
            .tasks_stop
            .write(|w| w.tasks_stop().set_bit());
        Timer::<T, Stopped, W, I, TimerMode<P>> {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
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
        Timer::<T, S, W, Disabled, C> {
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
        Timer::<T, S, W, Enabled, C> {
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
        self.timer.as_timer0().tasks_clear.write(|w| w.tasks_clear().set_bit());
    }
}
