mod private {
    pub trait Sealed {}
}

/// Common interface to all prescale values
///
/// This trait is sealed and not meant to be implemented outside this crate.
pub trait Prescaler: private::Sealed {
    /// The eventual value that gets written to the `PRESCALE` register.
    const VAL: u32;
}

macro_rules! define_prescaler {
    ($num:expr) => {
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
