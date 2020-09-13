//! Static assertions.

use crate::lib::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex};
use crate::tests::*;

assert_impl_all!(Arguments<Q<Z0, Z0, Z0>, meter>: Clone, Copy);
assert_not_impl_any!(Arguments<Q<Z0, Z0, Z0>, meter>: Binary, Debug, Display, LowerExp, LowerHex,
    Octal, UpperExp, UpperHex);
assert_impl_all!(DisplayStyle: Clone, Copy);

storage_types! {
    types: Float;

    use super::*;

    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>: Clone, Copy, PartialEq, PartialOrd, Send,
        Sync);
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>: Clone, Copy, Debug, Display,
        LowerExp, UpperExp);
    assert_not_impl_any!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>: Binary, LowerHex, Octal,
        UpperHex);
}

storage_types! {
    types: PrimInt;

    use super::*;

    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>: Clone, Copy, Eq, Ord, PartialEq, PartialOrd,
        Send, Sync, crate::lib::hash::Hash);
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>: Clone, Copy, Binary, Debug,
        Display, LowerHex, Octal, UpperHex);
    assert_not_impl_any!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>: LowerExp, UpperExp);
}

storage_types! {
    types: Rational, Rational32, Rational64;

    use super::*;

    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>: Clone, Copy, Eq, Ord, PartialEq, PartialOrd,
        Send, Sync, crate::lib::hash::Hash);
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>: Clone, Copy, Debug, Display);
    assert_not_impl_any!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>: Binary, LowerExp,
        LowerHex, Octal, UpperExp, UpperHex);
}

storage_types! {
    types: BigInt, BigUint;

    use super::*;

    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>: Clone, Eq, Ord, PartialEq, PartialOrd, Send,
        Sync, crate::lib::hash::Hash);
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>: Clone, Binary, Debug,
        Display, LowerHex, Octal, UpperHex);
    assert_not_impl_any!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>: LowerExp, UpperExp);
}

storage_types! {
    types: BigRational;

    use super::*;

    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>: Clone, Eq, Ord, PartialEq, PartialOrd, Send,
        Sync, crate::lib::hash::Hash);
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>: Clone, Debug, Display);
    assert_not_impl_any!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>: Binary, LowerExp,
        LowerHex, Octal, UpperExp, UpperHex);
}
