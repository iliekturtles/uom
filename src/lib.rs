#![no_std]

extern crate typenum;

#[macro_use]
mod system;
pub mod si;

use core::marker::{PhantomData};
use core::ops::{Add, Div, Mul, Sub};

pub trait Dimensions {}
pub trait Units {}

#[derive(Clone, Copy, Debug)]
pub struct Quantity<D: Dimensions, U: Units, V> {
    value: V,
    dimensions: PhantomData<D>,
    units: PhantomData<U>,
}

impl<D: Dimensions, U: Units, V: Default> Default for Quantity<D, U, V> {
    fn default() -> Quantity<D, U, V> {
        Quantity { value: V::default(), dimensions: PhantomData, units: PhantomData, }
    }
}

impl<D, Ul, Ur, Vl, Vr> Add<Quantity<D, Ur, Vr>> for Quantity<D, Ul, Vl>
    where D: Dimensions,
        Ul: Units,
        Ur: Units,
        Vl: Add<Vr> {
    type Output = Quantity<D, Ul, <Vl as Add<Vr>>::Output>;

    #[inline]
    fn add(self, rhs: Quantity<D, Ur, Vr>) -> Self::Output {
        Quantity { value: self.value + rhs.value, dimensions: PhantomData, units: PhantomData, }
    }
}

impl<Dl, Dr, Ul, Ur, Vl, Vr> Div<Quantity<Dr, Ur, Vr>> for Quantity<Dl, Ul, Vl>
    where Dl: Dimensions + Sub<Dr>,
        Dr: Dimensions,
        Ul: Units,
        Ur: Units,
        <Dl as Sub<Dr>>::Output: Dimensions,
        Vl: Div<Vr> {
    type Output = Quantity<<Dl as Sub<Dr>>::Output, Ul, <Vl as Div<Vr>>::Output>;

    #[inline]
    fn div(self, rhs: Quantity<Dr, Ur, Vr>) -> Self::Output {
        Quantity { value: self.value / rhs.value, dimensions: PhantomData, units: PhantomData, }
    }
}

pub trait Conversion<V, S>
    where V: Div<V> + Mul<V> {
    fn to_base(value: V, subunit: S) -> <V as Mul<V>>::Output;
    fn from_base(value: V, subunit: S) -> <V as Div<V>>::Output;
    fn new(value: V, subunit: S) -> Self;
    fn get(self, subunit: S) -> <V as Div<V>>::Output;
}
