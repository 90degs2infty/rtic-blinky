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

use core::{marker::PhantomData, ops::Deref};
use nrf52840_hal::{
    pac::{timer0::RegisterBlock as BasicRegBlock, timer3::RegisterBlock as ExtendedRegBlock},
    timer::Instance,
};
use nrf_proc_macros::enclose;

use crate::timer::{
    bitmode::{Width, W32},
    interrupts::{ClearState, Disabled, Enabled, InterruptSource, IS4, IS6},
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

// Okay, so go ahead and:
// - IDisabled4 and IDisabled6
// - Introduce constructor macro for 4-CC variant and for 6-CC variant
// - Separate the 4-CC and the 6-CC variant for enabling and disabling interrupts, to this end, you may have to restructure the macro code to make it more accessible

#[inline]
fn stop_timer<T>(timer: &T)
where
    T: Instance,
{
    timer
        .as_timer0()
        .tasks_stop
        .write(|w| w.tasks_stop().set_bit());
}

#[inline]
fn ensure_width_32<T>(timer: &T)
where
    T: Instance,
{
    timer.as_timer0().bitmode.write(|w| W32::set(w));
}

#[inline]
fn set_timer_mode<T>(timer: &T)
where
    T: Instance,
{
    timer.as_timer0().mode.write(|w| w.mode().timer());
}

#[inline]
fn set_counter_mode<T>(timer: &T)
where
    T: Instance,
{
    timer.as_timer0().mode.write(|w| w.mode().counter());
}

#[inline]
fn ensure_prescale_0<T>(timer: &T)
where
    T: Instance,
{
    timer
        .as_timer0()
        .prescaler
        .write(|w| unsafe { w.bits(P0::VAL) });
}

impl<T> Timer<T, Stopped, W32, <InterruptSource<T> as ClearState>::Disabled, TimerMode<P0>>
where
    T: Instance,
    InterruptSource<T>: ClearState<Raw = T>,
{
    /// Conversion function to turn a PAC-level timer interface into a
    /// HAL-level timer running in timer mode.
    pub fn timer(timer: T) -> Self {
        // Make sure the timer is stopped
        stop_timer(&timer);

        // Set bit width
        ensure_width_32(&timer);

        // Disable and clear all interrupts
        <InterruptSource<T> as ClearState>::reset_interrupts(&timer);

        // Set timer mode
        set_timer_mode(&timer);

        // Set prescale value
        ensure_prescale_0(&timer);

        Self {
            timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T> Timer<T, Stopped, W32, <InterruptSource<T> as ClearState>::Disabled, CounterMode>
where
    T: Instance,
    InterruptSource<T>: ClearState<Raw = T>,
{
    /// Constructor to turn a PAC-level timer peripheral into a HAL-level timer
    /// running in counter mode.
    pub fn counter(timer: T) -> Self {
        // Make sure the timer is stopped
        stop_timer(&timer);

        // Set bit width
        ensure_width_32(&timer);

        // Disable and clear interrupts
        <InterruptSource<T> as ClearState>::reset_interrupts(&timer);

        // Set counter mode
        set_counter_mode(&timer);

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

macro_rules! timer {
    ( $timer:expr ) => {
        Timer {
            timer: $timer,
            w: PhantomData,
            s: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    };
}

macro_rules! disable_interrupt {
    ( $timer:expr, $num:literal ) => {
        paste::paste! {
            $timer.intenclr.write(|w| w.[< compare $num >]().set_bit());
        }
    };
}

macro_rules! enable_interrupt {
    ( $timer:expr, $num:literal ) => {
        paste::paste! {
            $timer.intenset.write(|w| w.[< compare $num >]().set_bit());
        }
    };
}

macro_rules! unpend_interrupt {
    ( $timer:expr, $num:literal ) => {
        paste::paste! {
            $timer.events_compare[$num].write(|w| w.events_compare().clear_bit());
        }
    };
}

macro_rules! write_compare_value {
    ( $timer:expr, $num:literal, $val:ident) => {
        paste::paste! {
            $timer.cc[$num].write(|w| unsafe { w.cc().bits($val) });
        }
    };
}

macro_rules! define_basic_cc {
    ( $num:literal ) => {
        paste::paste! {

            impl<T, S, W, IA, IB, IC, C> Timer<T, S, W, enclose!(Enabled at $num by IA, IB, IC wrapped_in IS4), C>
            where
                T: Instance,
                W: Width,
                T: Deref<Target = BasicRegBlock>,
            {
                #[doc = "Disable interrupt " $num "."]
                #[doc = ""]
                #[doc = "For details, see Nordic's documentation on the [`INTENCLR`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_9#register.INTENCLR) register."]
                pub fn [< disable_interrupt_ $num >](self) -> Timer<T, S, W, enclose!(Disabled at $num by IA, IB, IC wrapped_in IS4), C> {
                    disable_interrupt!(self.timer, $num);
                    timer!(self.timer)
                }
            }

            impl<T, S, W, IA, IB, IC, C> Timer<T, S, W, enclose!(Disabled at $num by IA, IB, IC wrapped_in IS4), C>
            where
                T: Instance,
                W: Width,
                T: Deref<Target = BasicRegBlock>,
            {
                #[doc = "Disable interrupt " $num "."]
                #[doc = ""]
                #[doc = "For details, see Nordic's documentation on the [`INTENSET`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_8#register.INTENSET) register."]
                pub fn [< enable_interrupt_ $num >](self) -> Timer<T, S, W, enclose!(Enabled at $num by IA, IB, IC wrapped_in IS4), C> {
                    enable_interrupt!(self.timer, $num);
                    timer!(self.timer)
                }
            }

            impl<T, S, W, I, C> Timer<T, S, W, I, C>
            where
                T: Instance,
                W: Width,
                T: Deref<Target = BasicRegBlock>,
            {
                #[doc = "Unpend interrupt " $num "."]
                #[doc = ""]
                #[doc = "For details, see Nordic's documentation on the [`EVENTS_COMPARE`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_6#register.EVENTS_COMPARE-0-5) register."]
                pub fn [< unpend_interrupt_ $num >](&mut self) {
                    unpend_interrupt!(self.timer, $num);
                }

                #[doc = "Set compare value " $num "."]
                #[doc = ""]
                #[doc = "For details, see Nordic's documentation on the [`CC`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_13#register.CC-0-5) register."]
                #[doc = ""]
                #[doc = "A note on safety: it is safe to set any `u32` compare value."]
                #[doc = "Depending on the timer's set bit width, not all bits will be used for comparison by the peripheral, though."]
                pub fn [< compare_against_ $num >](&mut self, val: u32) {
                    write_compare_value!(self.timer, $num, val);
                }
            }

        // todo: task_capture
        }
    }
}

define_basic_cc!(0);
define_basic_cc!(1);
define_basic_cc!(2);
define_basic_cc!(3);

impl<T, S, W, I, C> Timer<T, S, W, I, C>
where
    T: Instance,
    W: Width,
{
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
