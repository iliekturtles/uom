//! Static assertions.

use crate::lib::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex};
use crate::lib::hash::Hash;
use crate::lib::marker::Unpin;
#[cfg(feature = "std")]
use crate::lib::panic::{RefUnwindSafe, UnwindSafe};
use crate::tests::*;

assert_impl_all!(Arguments<Q<Z0, Z0, Z0>, meter>:
    Clone, Copy, Send, Sync, Unpin);
#[cfg(feature = "std")]
assert_impl_all!(Arguments<Q<Z0, Z0, Z0>, meter>:
    RefUnwindSafe, UnwindSafe);
assert_not_impl_any!(Arguments<Q<Z0, Z0, Z0>, meter>:
    Binary, Debug, Display, Eq, Hash, LowerExp, LowerHex, Octal, Ord, PartialEq, PartialOrd,
    UpperExp, UpperHex);
#[rustfmt::skip]
assert_impl_all!(DisplayStyle:
    Clone, Copy, Debug, Send, Sync, Unpin);
#[cfg(feature = "std")]
#[rustfmt::skip]
assert_impl_all!(DisplayStyle:
    RefUnwindSafe, UnwindSafe);
#[rustfmt::skip]
assert_not_impl_any!(DisplayStyle:
    Binary, Display, Eq, Hash, LowerExp, LowerHex, Ord, Octal, PartialEq, PartialOrd, UpperExp,
    UpperHex);
#[rustfmt::skip]
assert_impl_all!(ParseQuantityError:
    Clone, Debug, Display, Eq, PartialEq, Send, Sync, Unpin);
#[cfg(feature = "std")]
#[rustfmt::skip]
assert_impl_all!(ParseQuantityError:
    RefUnwindSafe, UnwindSafe);
#[rustfmt::skip]
assert_not_impl_any!(ParseQuantityError:
    Binary, Copy, Hash, LowerExp, LowerHex, Ord, Octal, PartialOrd, UpperExp, UpperHex);

storage_types! {
    types: Float;

    use super::*;

    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        Clone, Copy, Debug, PartialEq, PartialOrd, Send, Sync, Unpin);
    #[cfg(feature = "std")]
    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        RefUnwindSafe, UnwindSafe);
    assert_not_impl_any!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        Binary, Display, Eq, Hash, LowerExp, LowerHex, Octal, Ord, UpperExp, UpperHex);
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        Clone, Copy, Debug, Display, LowerExp, Send, Sync, Unpin, UpperExp);
    #[cfg(feature = "std")]
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        RefUnwindSafe, UnwindSafe);
    assert_not_impl_any!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        Binary, Eq, Hash, LowerHex, Octal, Ord, PartialEq, PartialOrd, UpperHex);
}

storage_types! {
    types: PrimInt;

    use super::*;

    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Send, Sync, Unpin);
    #[cfg(feature = "std")]
    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        RefUnwindSafe, UnwindSafe);
    assert_not_impl_any!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        Binary, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex);
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        Binary, Clone, Copy, Debug, Display, LowerExp, LowerHex, Octal, Send, Sync, Unpin, UpperExp,
        UpperHex);
    #[cfg(feature = "std")]
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        RefUnwindSafe, UnwindSafe);
    assert_not_impl_any!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        Eq, Hash, Ord, PartialEq, PartialOrd);
}

storage_types! {
    types: Rational, Rational32, Rational64;

    use super::*;

    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Send, Sync, Unpin);
    #[cfg(feature = "std")]
    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        RefUnwindSafe, UnwindSafe);
    assert_not_impl_any!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        Binary, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex);
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        Binary, Clone, Copy, Debug, Display, LowerExp, LowerHex, Octal, Send, Sync, Unpin, UpperExp,
        UpperHex);
    #[cfg(feature = "std")]
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        RefUnwindSafe, UnwindSafe);
    assert_not_impl_any!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        Eq, Hash, Ord, PartialEq, PartialOrd);
}

storage_types! {
    types: BigInt, BigUint;

    use super::*;

    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Send, Sync, Unpin);
    #[cfg(feature = "std")]
    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        RefUnwindSafe, UnwindSafe);
    assert_not_impl_any!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        Binary, Copy, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex);
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        Binary, Clone, Debug, Display, LowerHex, Octal, Send, Sync, Unpin, UpperHex);
    #[cfg(feature = "std")]
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        RefUnwindSafe, UnwindSafe);
    assert_not_impl_any!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        Copy, Eq, Hash, LowerExp, Ord, PartialEq, PartialOrd, UpperExp);
}

storage_types! {
    types: BigRational;

    use super::*;

    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Send, Sync, Unpin);
    #[cfg(feature = "std")]
    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        RefUnwindSafe, UnwindSafe);
    assert_not_impl_any!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        Binary, Copy, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex);
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        Binary, Clone, Debug, Display, LowerHex, Octal, Send, Sync, Unpin, UpperHex);
    #[cfg(feature = "std")]
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        RefUnwindSafe, UnwindSafe);
    assert_not_impl_any!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        Copy, Eq, Hash, LowerExp, Ord, PartialEq, PartialOrd, UpperExp);
}

storage_types! {
    types: Complex;

    use super::*;

    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        Clone, Copy, Debug, PartialEq, Send, Sync, Unpin);
    #[cfg(feature = "std")]
    assert_impl_all!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        RefUnwindSafe, UnwindSafe);
    assert_not_impl_any!(Quantity<Q<Z0, Z0, Z0>, U<V>, V>:
        Binary, Display, Eq, Hash, LowerExp, LowerHex, Octal, Ord, PartialOrd, UpperExp, UpperHex);
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        Clone, Copy, Debug, Display, LowerExp, Send, Sync, Unpin, UpperExp);
    #[cfg(feature = "std")]
    assert_impl_all!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        RefUnwindSafe, UnwindSafe);
    assert_not_impl_any!(QuantityArguments<Q<Z0, Z0, Z0>, U<V>, V, meter>:
        Binary, Eq, Hash, LowerHex, Octal, Ord, PartialEq, PartialOrd, UpperHex);
}
