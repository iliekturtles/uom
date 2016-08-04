#![no_std]

extern crate typenum;

#[macro_use]
mod system;
pub mod si;

use core::marker::{PhantomData};
use core::ops::{Add, Div, Mul, Sub};

pub trait Dimensions {}

#[derive(Clone, Copy, Debug)]
pub struct Quantity<D: Dimensions, V> {
    value: V,
    dimensions: PhantomData<D>,
}

impl<D: Dimensions, V: Default> Default for Quantity<D, V> {
    fn default() -> Quantity<D, V> {
        Quantity { value: V::default(), dimensions: PhantomData, }
    }
}

impl<D, Vl, Vr> Add<Quantity<D, Vr>> for Quantity<D, Vl>
    where D: Dimensions,
        Vl: Add<Vr> {
    type Output = Quantity<D, <Vl as Add<Vr>>::Output>;

    #[inline]
    fn add(self, rhs: Quantity<D, Vr>) -> Self::Output {
        Quantity { value: self.value + rhs.value, dimensions: PhantomData, }
    }
}

impl<Dl, Dr, Vl, Vr> Div<Quantity<Dr, Vr>> for Quantity<Dl, Vl>
    where Dl: Dimensions + Sub<Dr>,
        Dr: Dimensions,
        <Dl as Sub<Dr>>::Output: Dimensions,
        Vl: Div<Vr> {
    type Output = Quantity<<Dl as Sub<Dr>>::Output, <Vl as Div<Vr>>::Output>;

    #[inline]
    fn div(self, rhs: Quantity<Dr, Vr>) -> Self::Output {
        Quantity { value: self.value / rhs.value, dimensions: PhantomData, }
    }
}

pub trait Conversion<V, S>
    where V: Div<V> + Mul<V> {
    fn to_base(value: V, subunit: S) -> <V as Mul<V>>::Output;
    fn from_base(value: V, subunit: S) -> <V as Div<V>>::Output;
    fn get(self, subunit: S) -> <V as Div<V>>::Output;
}
