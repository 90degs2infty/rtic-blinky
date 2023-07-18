//! Hal-level interface to a timer's state.
//!
//! See [Nordic's docs](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29) for details.

/// Type indicating a running timer.
pub struct Started;

/// Type indicating a not-running timer.
pub struct Stopped;
