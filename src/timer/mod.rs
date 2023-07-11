//! A basic hal-level interface to a timer's timer mode.
//!
//! Note that this module kind of by-passes [`nrf52480_hal`'s `timer` module](https://docs.rs/nrf52840-hal/latest/nrf52840_hal/timer/index.html)

use nrf52840_hal::{pac::timer0::bitmode::W, timer::Instance};

use core::marker::PhantomData;

macro_rules! define_prescaler {
    ($num:expr) => {
        paste::paste! {
            #[doc = "Type encoding a prescale value of " [<$num>] "."]
            #[doc = "See Nordic's docs on the `PRESCALER` register for details."]
            pub struct [<U $num>];

            impl Prescaler for [<U $num>] {
                const VAL: u32 = $num;
            }
        }
    };
}

// ---------
// Prescaler
// ---------

define_prescaler!(0);
define_prescaler!(1);
define_prescaler!(2);
define_prescaler!(3);
define_prescaler!(4);
define_prescaler!(5);
define_prescaler!(6);
define_prescaler!(7);
define_prescaler!(8);
define_prescaler!(9);
define_prescaler!(10);
define_prescaler!(11);
define_prescaler!(12);
define_prescaler!(13);
define_prescaler!(14);
define_prescaler!(15);

/// Common interface to all prescale values
pub trait Prescaler {
    /// The eventual value that gets written to the `PRESCALE` register.
    const VAL: u32;
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
        // Make sure the timer is stopped
        timer
            .as_timer0()
            .tasks_stop
            .write(|w| w.tasks_stop().set_bit());

        // Set bit width
        timer.as_timer0().bitmode.write(|w| ThirtyTwo::set(w));

        // Disable and clear interrupts
        timer.as_timer0().intenclr.write(|w| w.compare0().set_bit());
        timer.as_timer0().events_compare[0].write(|w| w);

        // Set timer mode
        timer.as_timer0().mode.write(|w| w.mode().timer());

        // Set prescale value
        timer
            .as_timer0()
            .prescaler
            .write(|w| unsafe { w.bits(U0::VAL) });

        Timer {
            timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T> Timer<T, Stopped, ThirtyTwo, Disabled, CounterMode>
where
    T: Instance,
{
    pub fn counter(timer: T) -> Timer<T, Stopped, ThirtyTwo, Disabled, CounterMode> {
        // Make sure the timer is stopped
        timer
            .as_timer0()
            .tasks_stop
            .write(|w| w.tasks_stop().set_bit());

        // Set bit width
        timer.as_timer0().bitmode.write(|w| ThirtyTwo::set(w));

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
