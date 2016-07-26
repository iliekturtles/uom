#![no_std]

extern crate typenum;

#[macro_use]
mod system;
pub mod si;

use core::marker::{PhantomData};
use core::ops::{Add, Div, Mul, Sub};

pub trait Dimensions {}

#[derive(Clone, Copy, Debug)]
pub struct Scalar<D: Dimensions, V> {
    value: V,
    dimensions: PhantomData<D>,
}

impl<D: Dimensions, V: Default> Default for Scalar<D, V> {
    fn default() -> Scalar<D, V> {
        Scalar { value: V::default(), dimensions: PhantomData, }
    }
}

impl<D, Vl, Vr> Add<Scalar<D, Vr>> for Scalar<D, Vl>
    where D: Dimensions,
        Vl: Add<Vr> {
    type Output = Scalar<D, <Vl as Add<Vr>>::Output>;

    #[inline]
    fn add(self, rhs: Scalar<D, Vr>) -> Self::Output {
        Scalar { value: self.value + rhs.value, dimensions: PhantomData, }
    }
}

impl<Dl, Dr, Vl, Vr> Div<Scalar<Dr, Vr>> for Scalar<Dl, Vl>
    where Dl: Dimensions + Sub<Dr>,
        Dr: Dimensions,
        <Dl as Sub<Dr>>::Output: Dimensions,
        Vl: Div<Vr> {
    type Output = Scalar<<Dl as Sub<Dr>>::Output, <Vl as Div<Vr>>::Output>;

    #[inline]
    fn div(self, rhs: Scalar<Dr, Vr>) -> Self::Output {
        Scalar { value: self.value / rhs.value, dimensions: PhantomData, }
    }
}

pub trait Conversion<V, S>
    where V: Div<V> + Mul<V> {
    fn to_base(value: V, subunit: S) -> <V as Mul<V>>::Output;
    fn from_base(value: V, subunit: S) -> V;
    fn get(self, subunit: S) -> V;
}
