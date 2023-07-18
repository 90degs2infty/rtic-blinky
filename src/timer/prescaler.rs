//! Hal-level interface to a timer's prescaler values.
//!
//! See [Nordic's docs](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29) for details.

mod private {
    pub trait Sealed {}
}

/// Common interface to all prescale values
///
/// This trait is sealed and not meant to be implemented outside this crate.
pub trait Prescaler: private::Sealed {
    /// The eventual value that gets written to the [`PRESCALER`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_12#register.PRESCALER) register.
    const VAL: u32;
}

macro_rules! define_prescaler {
    ($num:literal) => {
        paste::paste! {
            #[doc = "Type encoding a prescale value of " [<$num>] "."]
            #[doc = "See Nordic's docs on the `PRESCALER` register for details."]
            pub struct [<P $num>];

            impl private::Sealed for [<P $num >] {}

            impl Prescaler for [<P $num>] {
                const VAL: u32 = $num;
            }
        }
    };
}

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
