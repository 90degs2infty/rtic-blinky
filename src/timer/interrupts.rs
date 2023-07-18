//! Hal-level interface to a timer's interrupts.
//!
//! See [Nordic's docs](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29) for details.

/// Type indicating a timer triggering interrupts.
pub struct Enabled;

/// Type indicating a timer not triggering interrupts.
pub struct Disabled;
