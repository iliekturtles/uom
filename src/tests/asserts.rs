//! Static assertions.

use tests::*;

assert_impl!(arguments; Arguments<Q<Z0, Z0, Z0>, meter>,
    Clone, Copy);
assert_impl!(display_style; DisplayStyle,
    Clone, Copy);

storage_types! {
    types: Float;

    use lib::fmt::{Display, LowerExp, UpperExp};
    use tests::*;

    assert_impl!(q; Quantity<Q<Z0, Z0, Z0>, U<V>, V>,
        Clone, Copy, PartialEq, PartialOrd, Send, Sync);
    assert_impl!(quantity_arguments; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Clone, Copy, Debug, Display, LowerExp, UpperExp);
}

storage_types! {
    types: PrimInt;

    use lib::fmt::{Binary, Display, LowerHex, Octal, UpperHex};
    use tests::*;

    assert_impl!(q; Quantity<Q<Z0, Z0, Z0>, U<V>, V>,
        Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Send, Sync, ::lib::hash::Hash);
    assert_impl!(quantity_arguments; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Clone, Copy, Binary, Debug, Display, LowerHex, Octal, UpperHex);
}

storage_types! {
    types: Rational, Rational32, Rational64;

    use lib::fmt::Display;
    use tests::*;

    assert_impl!(q; Quantity<Q<Z0, Z0, Z0>, U<V>, V>,
        Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Send, Sync, ::lib::hash::Hash);
    assert_impl!(quantity_arguments; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Clone, Copy, Debug, Display);
}

storage_types! {
    types: BigInt, BigUint;

    use lib::fmt::{Binary, Display, LowerHex, Octal, UpperHex};
    use tests::*;

    assert_impl!(q; Quantity<Q<Z0, Z0, Z0>, U<V>, V>,
        Clone, Eq, Ord, PartialEq, PartialOrd, Send, Sync, ::lib::hash::Hash);
    assert_impl!(quantity_arguments; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Clone, Binary, Debug, Display, LowerHex, Octal, UpperHex);
}

storage_types! {
    types: BigRational;

    use lib::fmt::Display;
    use tests::*;

    assert_impl!(q; Quantity<Q<Z0, Z0, Z0>, U<V>, V>,
        Clone, Eq, Ord, PartialEq, PartialOrd, Send, Sync, ::lib::hash::Hash);
    assert_impl!(quantity_arguments; QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>,
        Clone, Debug, Display);
}
