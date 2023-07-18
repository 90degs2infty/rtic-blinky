//! Hal-level interface to a timer's interrupts.
//!
//! See [Nordic's docs](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29) for details.

use core::marker::PhantomData;

/// Type indicating a timer triggering interrupts.
pub struct Enabled;

/// Type indicating a timer not triggering interrupts.
pub struct Disabled;

pub struct Interrupt<I0, I1, I2, I3> {
    i0: PhantomData<I0>,
    i1: PhantomData<I1>,
    i2: PhantomData<I2>,
    i3: PhantomData<I3>,
}
