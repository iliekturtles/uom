//! Static assertions.

use tests::*;

assert_impl!(arguments; Arguments<Q<P1, Z0>, meter>,
    Clone, Copy);
assert_impl!(display_style; ::fmt::DisplayStyle,
    Clone, Copy);

storage_types! {
    types: Float;

    use tests::*;

    assert_impl!(q; Quantity<Q<Z0, Z0>, U<V>, V>,
        Clone, Copy, PartialEq, PartialOrd, Send, Sync);
    assert_impl!(quantity_arguments; QuantityArguments<Q<P1, Z0>, U<V>, V, meter>,
        Clone, Copy);
}

storage_types! {
    types: PrimInt, Rational, Rational32, Rational64;

    use tests::*;

    assert_impl!(q; Quantity<Q<Z0, Z0>, U<V>, V>,
        Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Send, Sync, ::lib::hash::Hash);
    assert_impl!(quantity_arguments; QuantityArguments<Q<P1, Z0>, U<V>, V, meter>,
        Clone, Copy);
}

storage_types! {
    types: BigInt, BigUint, BigRational;

    use tests::*;

    assert_impl!(q; Quantity<Q<Z0, Z0>, U<V>, V>,
        Clone, Eq, Ord, PartialEq, PartialOrd, Send, Sync, ::lib::hash::Hash);
    assert_impl!(quantity_arguments; QuantityArguments<Q<P1, Z0>, U<V>, V, meter>,
        Clone);
}
