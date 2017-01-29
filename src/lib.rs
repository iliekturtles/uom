#![no_std]

extern crate typenum;

#[macro_use]
mod system;
pub mod si;

use core::marker::{PhantomData};
use core::ops::{Add, Div, Sub};

pub trait Dimensions {}
pub trait Units<D: Dimensions, V> {
    fn conversion() -> V;
}
pub trait Unit<V> {
    fn conversion() -> V;
}

#[derive(Clone, Copy, Debug)]
pub struct Quantity<D: Dimensions, U: Units<D, V>, V> {
    value: V,
    dimensions: PhantomData<D>,
    units: PhantomData<U>,
}

impl<D: Dimensions, U: Units<D, V>, V: Default> Default for Quantity<D, U, V> {
    #[inline(always)]
    fn default() -> Quantity<D, U, V> {
        Quantity { value: V::default(), dimensions: PhantomData, units: PhantomData, }
    }
}

#[macro_export]
macro_rules! ops {
    ($($T:ty),+) => {
        $(impl<D, Ul, Ur> Add<$crate::Quantity<D, Ur, $T>> for $crate::Quantity<D, Ul, $T>
            where D: $crate::Dimensions,
                Ul: $crate::Units<D, $T>,
                Ur: $crate::Units<D, $T> {
            type Output = $crate::Quantity<D, Ul, $T>;

            #[inline(always)]
            fn add(self, rhs: $crate::Quantity<D, Ur, $T>) -> Self::Output {
                $crate::Quantity {
                    value: self.value
                        + (<Ur as $crate::Units<D, $T>>::conversion()
                            / <Ul as $crate::Units<D, $T>>::conversion()
                            * rhs.value),
                    dimensions: PhantomData,
                    units: PhantomData,
                }
            }
        }

        impl<Dl, Dr, Ul, Ur> Div<$crate::Quantity<Dr, Ur, $T>> for $crate::Quantity<Dl, Ul, $T>
            where Dl: $crate::Dimensions + Sub<Dr>,
                Dr: $crate::Dimensions,
                Ul: $crate::Units<Dl, $T> + Units<<Dl as Sub<Dr>>::Output, $T>,
                Ur: $crate::Units<Dr, $T>,
                <Dl as Sub<Dr>>::Output: $crate::Dimensions {
            type Output = $crate::Quantity<<Dl as Sub<Dr>>::Output, Ul, $T>;

            #[inline(always)]
            fn div(self, rhs: $crate::Quantity<Dr, Ur, $T>) -> Self::Output {
                $crate::Quantity {
                    value: self.value
                        / (<Ur as $crate::Units<Dr, $T>>::conversion()
                            / <Ul as $crate::Units<Dl, $T>>::conversion()
                            * rhs.value),
                    dimensions: PhantomData,
                    units: PhantomData,
                }
            }
        })+
    }
}

ops!(f32, f64);
