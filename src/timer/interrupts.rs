//! Hal-level interface to a timer's interrupts.
//!
//! See [Nordic's docs](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29) for details.

use core::marker::PhantomData;
use nrf52840_hal::{
    pac::{TIMER0, TIMER1, TIMER2, TIMER3, TIMER4},
    timer::Instance,
};

mod private {
    pub trait Sealed {}
}

/// Type indicating a timer triggering interrupts.
pub struct Enabled;

/// Type indicating a timer not triggering interrupts.
pub struct Disabled;

/// Type modelling the en-/disabled state of four interrupts.
///
/// IS is short for Interrupt State.
pub struct IS4<I0, I1, I2, I3> {
    i0: PhantomData<I0>,
    i1: PhantomData<I1>,
    i2: PhantomData<I2>,
    i3: PhantomData<I3>,
}

/// Type modelling the en-/disabled state of six interrupts.
///
/// IS is short for Interrupt State.
pub struct IS6<I0, I1, I2, I3, I4, I5> {
    i0: PhantomData<I0>,
    i1: PhantomData<I1>,
    i2: PhantomData<I2>,
    i3: PhantomData<I3>,
    i4: PhantomData<I4>,
    i5: PhantomData<I5>,
}

// To comply with [C-CTOR](https://docs.rust-embedded.org/book/design-patterns/hal/predictability.html#constructors-are-used-instead-of-extension-traits-c-ctor),
// we wrap the TIMERX types in another type, which then implements the
// InterruptState trait to have the reset state easily accessible
pub struct InterruptSource<T>
where
    T: Instance,
{
    source: PhantomData<T>,
}

pub trait DefaultState: private::Sealed {
    type Disabled;
}

macro_rules! define_interrupt_state_4 {
    ( $num:literal ) => {
        paste::paste! {
            impl private::Sealed for InterruptSource< [< TIMER $num >] > {}

            impl DefaultState for InterruptSource< [< TIMER $num >] > {
                type Disabled = IS4<Disabled, Disabled, Disabled, Disabled>;
            }
        }
    };
}

define_interrupt_state_4!(0);
define_interrupt_state_4!(1);
define_interrupt_state_4!(2);

macro_rules! define_interrupt_state_6 {
    ( $num:literal ) => {
        paste::paste! {
            impl private::Sealed for InterruptSource< [< TIMER $num >] > {}

            impl DefaultState for InterruptSource< [< TIMER $num >] > {
                type Disabled = IS6<Disabled, Disabled, Disabled, Disabled, Disabled, Disabled>;
            }
        }
    };
}

define_interrupt_state_6!(3);
define_interrupt_state_6!(4);
