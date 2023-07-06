use nrf52840_hal::timer::Instance;

use core::marker::PhantomData;

// Inputs:
// - Prescaler:
//   - four bit to divide 16MHz input (four bit = 0..15)
//   - frequency of timer (in timer mode) is 16MHz / 2 ^ prescale
// - Mode: Periodic, Counter
// - State: Running, Stopped/Cancelled/Inactive
// - Capture: ?
// - Clear: ?
// - Interrupts:
// - PPI stuff
// - Overflow value: bit width (BITMODE register)
//   - 0 - 16bit
//   - 1 - 8bit
//   - 2 - 24bit
//   - 3 - 32bit

// Outputs:
// - Compare

// Invariants
// - Change Prescaler and bit with in stopped state only

// Prescaler
pub struct U0;
pub struct U1;
pub struct U2;
pub struct U3;
pub struct U4;
pub struct U5;
pub struct U6;
pub struct U7;
pub struct U8;
pub struct U9;
pub struct U10;
pub struct U11;
pub struct U12;
pub struct U13;
pub struct U14;
pub struct U15;

pub trait Prescaler {
    const VAL: u32;
}

impl Prescaler for U0 {
    const VAL: u32 = 0;
}

impl Prescaler for U1 {
    const VAL: u32 = 1;
}

impl Prescaler for U2 {
    const VAL: u32 = 2;
}

impl Prescaler for U3 {
    const VAL: u32 = 3;
}

impl Prescaler for U4 {
    const VAL: u32 = 4;
}

impl Prescaler for U5 {
    const VAL: u32 = 5;
}

impl Prescaler for U6 {
    const VAL: u32 = 6;
}

impl Prescaler for U7 {
    const VAL: u32 = 7;
}

impl Prescaler for U8 {
    const VAL: u32 = 8;
}

impl Prescaler for U9 {
    const VAL: u32 = 9;
}

impl Prescaler for U10 {
    const VAL: u32 = 10;
}

impl Prescaler for U11 {
    const VAL: u32 = 11;
}

impl Prescaler for U12 {
    const VAL: u32 = 12;
}

impl Prescaler for U13 {
    const VAL: u32 = 13;
}

impl Prescaler for U14 {
    const VAL: u32 = 14;
}

impl Prescaler for U15 {
    const VAL: u32 = 15;
}

// Counter bit width
pub struct Eight;
pub struct Sixteen;
pub struct TwentyFour;
pub struct ThirtyTwo;

use nrf52840_hal::pac::timer0::bitmode::W;
pub trait Width {
    fn set(w: &mut W) -> &mut W;
}

impl Width for Eight {
    fn set(w: &mut W) -> &mut W {
        w.bitmode()._08bit()
    }
}

impl Width for Sixteen {
    fn set(w: &mut W) -> &mut W {
        w.bitmode()._16bit()
    }
}

impl Width for TwentyFour {
    fn set(w: &mut W) -> &mut W {
        w.bitmode()._24bit()
    }
}

impl Width for ThirtyTwo {
    fn set(w: &mut W) -> &mut W {
        w.bitmode()._32bit()
    }
}

// Config
pub struct OneShot;
pub struct Periodic<P: Prescaler> {
    prescaler: PhantomData<P>,
}

// State
pub struct Started;
pub struct Stopped;

// Interrupts
pub struct Enabled;
pub struct Disabled;

pub struct Timer<T: Instance, S, W: Width, I, C> {
    timer: T,
    w: PhantomData<W>,
    s: PhantomData<S>,
    i: PhantomData<I>,
    c: PhantomData<C>,
}

impl<T> Timer<T, Stopped, ThirtyTwo, Disabled, Periodic<U0>>
where
    T: Instance,
{
    pub fn periodic(timer: T) -> Timer<T, Stopped, ThirtyTwo, Disabled, Periodic<U0>> {
        // Set timer mode
        timer.as_timer0().mode.write(|w| w.mode().timer());

        // Set prescale value
        timer
            .as_timer0()
            .prescaler
            .write(|w| unsafe { w.bits(U0::VAL) });

        // Set bit width
        timer.as_timer0().bitmode.write(|w| ThirtyTwo::set(w));

        // Disable and clear interrupts
        timer.as_timer0().intenclr.write(|w| w.compare0().set_bit());
        timer.as_timer0().events_compare[0].write(|w| w);

        Timer::<T, Stopped, ThirtyTwo, Disabled, Periodic<U0>> {
            timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T, W, I, P> Timer<T, Stopped, W, I, Periodic<P>>
where
    T: Instance,
    W: Width,
    P: Prescaler,
{
    pub fn set_prescale<P2: Prescaler>(self) -> Timer<T, Stopped, W, I, Periodic<P2>> {
        self.timer
            .as_timer0()
            .prescaler
            .write(|w| unsafe { w.bits(P2::VAL) });
        Timer::<T, Stopped, W, I, Periodic<P2>> {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }

    pub fn set_counterwidth<W2: Width>(self) -> Timer<T, Stopped, W2, I, Periodic<P>> {
        self.timer.as_timer0().bitmode.write(|w| W2::set(w));
        Timer::<T, Stopped, W2, I, Periodic<P>> {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }

    pub fn start(self) -> Timer<T, Started, W, I, Periodic<P>> {
        self.timer
            .as_timer0()
            .tasks_start
            .write(|w| w.tasks_start().set_bit());
        Timer::<T, Started, W, I, Periodic<P>> {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T, W, I, P> Timer<T, Started, W, I, Periodic<P>>
where
    T: Instance,
    W: Width,
    P: Prescaler,
{
    pub fn stop(self) -> Timer<T, Stopped, W, I, Periodic<P>> {
        self.timer
            .as_timer0()
            .tasks_stop
            .write(|w| w.tasks_stop().set_bit());
        Timer::<T, Stopped, W, I, Periodic<P>> {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T, S, W, C> Timer<T, S, W, Enabled, C>
where
    T: Instance,
    W: Width,
{
    pub fn disable_interrupt(self) -> Timer<T, S, W, Disabled, C> {
        self.timer
            .as_timer0()
            .intenclr
            .write(|w| w.compare0().set_bit());
        Timer::<T, S, W, Disabled, C> {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T, S, W, C> Timer<T, S, W, Disabled, C>
where
    T: Instance,
    W: Width,
{
    pub fn enable_interrupt(self) -> Timer<T, S, W, Enabled, C> {
        self.timer
            .as_timer0()
            .intenset
            .write(|w| w.compare0().set_bit());
        Timer::<T, S, W, Enabled, C> {
            timer: self.timer,
            s: PhantomData,
            w: PhantomData,
            i: PhantomData,
            c: PhantomData,
        }
    }
}

impl<T, S, W, I, C> Timer<T, S, W, I, C>
where
    T: Instance,
    W: Width,
{
    pub fn unpend_interrupt(&mut self) {
        self.timer.as_timer0().events_compare[0].write(|w| w.events_compare().clear_bit());
    }

    pub fn compare_against(&mut self, val: u32) {
        self.timer.as_timer0().cc[0].write(|w| unsafe { w.cc().bits(val) });
    }
}
