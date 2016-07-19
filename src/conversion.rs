use core::ops::{Div, Mul};

pub trait Conversion<V, S>
    where V: Div<f64> + Mul<f64> {
    fn to_base(value: V, subunit: S) -> <V as Mul<f64>>::Output;

    fn from_base(value: V, subunit: S) -> <V as Div<f64>>::Output;
}
