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

pub trait ClearState: private::Sealed {
    type Raw;
    type Disabled;
    fn unpend_interrupts(timer: &Self::Raw);
    fn disable_interrupts(timer: &Self::Raw);

    fn reset_interrupts(timer: &Self::Raw) {
        Self::disable_interrupts(timer);
        Self::unpend_interrupts(timer);
    }
}

macro_rules! disable_interrupts {
    ( $timer:ident, $( $i:literal ),+ ) => {
        paste::paste! {
            $timer.intenclr.write(|w| {
                w
                $(
                    .[< compare $i >]()
                    .set_bit()
                )+
            });
        }
    }
}

macro_rules! unpend_interrupts {
    ( $timer:ident, $( $i:literal ),+ ) => {
        paste::paste! {
            $(
                $timer.events_compare[$i].write(|w| w);
            )+
        }
    };
}

macro_rules! define_basic_clearstate {
    ( $t:ty ) => {
        impl private::Sealed for InterruptSource<$t> {}

        impl ClearState for InterruptSource<$t> {
            type Raw = $t;
            type Disabled = IS4<Disabled, Disabled, Disabled, Disabled>;

            fn unpend_interrupts(timer: &Self::Raw) {
                unpend_interrupts!(timer, 0, 1, 2, 3);
            }

            fn disable_interrupts(timer: &Self::Raw) {
                disable_interrupts!(timer, 0, 1, 2, 3);
            }
        }
    };
}

define_basic_clearstate!(TIMER0);
define_basic_clearstate!(TIMER1);
define_basic_clearstate!(TIMER2);

macro_rules! define_extended_clearstate {
    ( $t:ty ) => {
        impl private::Sealed for InterruptSource<$t> {}

        impl ClearState for InterruptSource<$t> {
            type Raw = $t;
            type Disabled = IS6<Disabled, Disabled, Disabled, Disabled, Disabled, Disabled>;

            fn unpend_interrupts(timer: &Self::Raw) {
                unpend_interrupts!(timer, 0, 1, 2, 3, 4, 5);
            }

            fn disable_interrupts(timer: &Self::Raw) {
                disable_interrupts!(timer, 0, 1, 2, 3, 4, 5);
            }
        }
    };
}

define_extended_clearstate!(TIMER3);
define_extended_clearstate!(TIMER4);
