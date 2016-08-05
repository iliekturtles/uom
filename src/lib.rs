#![no_std]

extern crate typenum;

#[macro_use]
mod system;
pub mod si;

use core::marker::{PhantomData};
use core::ops::{Add, Div, Mul, Sub};

pub trait Dimensions {}

#[derive(Clone, Copy, Debug)]
pub struct Quantity<D: Dimensions, B, V> {
    value: V,
    dimensions: PhantomData<D>,
    base: PhantomData<B>,
}

impl<D: Dimensions, B, V: Default> Default for Quantity<D, B, V> {
    fn default() -> Quantity<D, B, V> {
        Quantity { value: V::default(), dimensions: PhantomData, base: PhantomData, }
    }
}

impl<D, B, Vl, Vr> Add<Quantity<D, B, Vr>> for Quantity<D, B, Vl>
    where D: Dimensions,
        Vl: Add<Vr> {
    type Output = Quantity<D, B, <Vl as Add<Vr>>::Output>;

    #[inline]
    fn add(self, rhs: Quantity<D, B, Vr>) -> Self::Output {
        Quantity { value: self.value + rhs.value, dimensions: PhantomData, base: PhantomData, }
    }
}

impl<Dl, Dr, B, Vl, Vr> Div<Quantity<Dr, B, Vr>> for Quantity<Dl, B, Vl>
    where Dl: Dimensions + Sub<Dr>,
        Dr: Dimensions,
        <Dl as Sub<Dr>>::Output: Dimensions,
        Vl: Div<Vr> {
    type Output = Quantity<<Dl as Sub<Dr>>::Output, B, <Vl as Div<Vr>>::Output>;

    #[inline]
    fn div(self, rhs: Quantity<Dr, B, Vr>) -> Self::Output {
        Quantity { value: self.value / rhs.value, dimensions: PhantomData, base: PhantomData, }
    }
}

pub trait Conversion<V, S>
    where V: Div<V> + Mul<V> {
    fn to_base(value: V, subunit: S) -> <V as Mul<V>>::Output;
    fn from_base(value: V, subunit: S) -> <V as Div<V>>::Output;
    fn new(value: V, subunit: S) -> Self;
    fn get(self, subunit: S) -> <V as Div<V>>::Output;
}
