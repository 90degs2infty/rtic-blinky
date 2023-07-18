//! Hal-level interface to a timer's bitmode.
//!
//! See [Nordic's docs](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29) for details.

use nrf52840_hal::pac::timer0::bitmode::W as Writer;

mod private {
    pub trait Sealed {}
}

/// Common interface to all bitmodes.
///
/// This trait is sealed and not meant to be implemented outside this crate.
pub trait Width {
    fn set(w: &mut Writer) -> &mut Writer;
}

macro_rules! define_width {
    ($w:literal) => {
        paste::paste! {
            #[doc = "Type encoding a bit with of " [<$w>] " bits."]
            #[doc = "See [Nordic's docs on the `BITMODE` register](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_11#register.BITMODE) for details."]
            pub struct [< W $w >] {}

            impl Width for [< W $w >] {
                fn set(w: &mut Writer) -> &mut Writer {
                    w.bitmode().[< _ $w bit >]()
                }
            }
        }
    };
}

define_width!(08);
define_width!(16);
define_width!(24);
define_width!(32);
