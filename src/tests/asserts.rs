//! Static assertions.

storage_types! {
    types: Float;

    use tests::*;

    assert_impl!(q; Quantity<Q<Z0, Z0>, U<V>, V>,
        Clone, Copy, PartialEq, PartialOrd, Send, Sync);
}

storage_types! {
    types: PrimInt, Rational, Rational32, Rational64;

    use tests::*;

    assert_impl!(q; Quantity<Q<Z0, Z0>, U<V>, V>,
        Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Send, Sync, ::lib::hash::Hash);
}

storage_types! {
    types: BigInt, BigUint, BigRational;

    use tests::*;

    assert_impl!(q; Quantity<Q<Z0, Z0>, U<V>, V>,
        Clone, Eq, Ord, PartialEq, PartialOrd, Send, Sync, ::lib::hash::Hash);
}
