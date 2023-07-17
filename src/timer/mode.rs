use crate::timer::prescaler::Prescaler;

use core::marker::PhantomData;

/// Type indicating a timer running in counter mode.
pub struct Counter;

/// Type indicating a timer running in timer mode.
pub struct Timer<P: Prescaler> {
    prescaler: PhantomData<P>,
}
