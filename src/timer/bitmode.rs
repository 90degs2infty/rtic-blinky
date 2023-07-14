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
    ($w:literal, $m:ident) => {
        paste::paste! {
            #[doc = "Type encoding a bit with of " [<$w>] " bits."]
            #[doc = "See Nordic's docs on the `BITMODE` register for details."]
            pub struct [< W $w >] {}

            impl Width for [< W $w >] {
                fn set(w: &mut Writer) -> &mut Writer {
                    w.bitmode().$m()
                }
            }
        }
    };
}

define_width!(8, _08bit);
define_width!(16, _16bit);
define_width!(24, _24bit);
define_width!(32, _32bit);
