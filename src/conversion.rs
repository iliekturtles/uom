use core::ops::{Div, Mul};

pub trait Conversion<V, S>
    where V: Div<V> + Mul<V> {
    fn to_base(value: V, subunit: S) -> <V as Mul<V>>::Output;

    fn from_base(value: V, subunit: S) -> <V as Div<V>>::Output;
}
