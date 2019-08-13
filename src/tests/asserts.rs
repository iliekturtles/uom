//! Static assertions.

use lib::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex};
use tests::*;

assert_impl_all!(arguments_impl; Arguments<Q<Z0, Z0, Z0>, meter>,
    Clone, Copy);
assert_not_impl_any!(arguments_not_impl; Arguments<Q<Z0, Z0, Z0>, meter>,
    Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex);
assert_impl_all!(display_style; DisplayStyle,
    Clone, Copy);

storage_types! {
    types: Float;

    use super::*;

    assert_impl_all!(q; Quantity<Q<Z0, Z0, Z0>, U<V>, V>,
        Clone, Copy, PartialEq, PartialOrd, Send, Sync);
    assert_impl_all!(qa_impl; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Clone, Copy, Debug, Display, LowerExp, UpperExp);
    assert_not_impl_any!(qa_not_impl; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Binary, LowerHex, Octal, UpperHex);
}

storage_types! {
    types: PrimInt;

    use super::*;

    assert_impl_all!(q; Quantity<Q<Z0, Z0, Z0>, U<V>, V>,
        Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Send, Sync, ::lib::hash::Hash);
    assert_impl_all!(qa_impl; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Clone, Copy, Binary, Debug, Display, LowerHex, Octal, UpperHex);
    assert_not_impl_any!(qa_not_impl; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        LowerExp, UpperExp);
}

storage_types! {
    types: Rational, Rational32, Rational64;

    use super::*;

    assert_impl_all!(q; Quantity<Q<Z0, Z0, Z0>, U<V>, V>,
        Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Send, Sync, ::lib::hash::Hash);
    assert_impl_all!(qa_impl; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Clone, Copy, Debug, Display);
    assert_not_impl_any!(qa_not_impl; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Binary, LowerExp, LowerHex, Octal, UpperExp, UpperHex);
}

storage_types! {
    types: BigInt, BigUint;

    use super::*;

    assert_impl_all!(q; Quantity<Q<Z0, Z0, Z0>, U<V>, V>,
        Clone, Eq, Ord, PartialEq, PartialOrd, Send, Sync, ::lib::hash::Hash);
    assert_impl_all!(qa_impl; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Clone, Binary, Debug, Display, LowerHex, Octal, UpperHex);
    assert_not_impl_any!(qa_not_impl; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        LowerExp, UpperExp);
}

storage_types! {
    types: BigRational;

    use super::*;

    assert_impl_all!(q; Quantity<Q<Z0, Z0, Z0>, U<V>, V>,
        Clone, Eq, Ord, PartialEq, PartialOrd, Send, Sync, ::lib::hash::Hash);
    assert_impl_all!(qa_impl; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Clone, Debug, Display);
    assert_not_impl_any!(qa_not_impl; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Binary, LowerExp, LowerHex, Octal, UpperExp, UpperHex);
}
