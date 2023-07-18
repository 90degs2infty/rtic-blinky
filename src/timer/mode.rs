//! Hal-level interface to a timer's timer mode.
//!
//! See [Nordic's docs](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29) for details.

use crate::timer::prescaler::Prescaler;

use core::marker::PhantomData;

/// Type indicating a timer running in counter mode.
pub struct Counter;

/// Type indicating a timer running in timer mode.
pub struct Timer<P: Prescaler> {
    prescaler: PhantomData<P>,
}
