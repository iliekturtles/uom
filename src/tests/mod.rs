//! Tests for `uom` macros.

use self::length::{kilometer, meter};
use self::mass::kilogram;
use lib::fmt::Debug;
use lib::marker::PhantomData;
#[allow(unused_imports)]
use num::{Float, FromPrimitive, One, Saturating, Signed, Zero};
use quickcheck::TestResult;
#[cfg(feature = "serde")]
use serde_json;
use str::ParseQuantityError;
#[allow(unused_imports)]
use typenum::{N1, P1, P2, P3, Z0};
#[allow(unused_imports)]
use {Conversion, ConversionFactor};

#[macro_use]
mod length {
    quantity! {
        quantity: Length; "length";
        dimension: Q<P1, Z0>;
        units {
            @kilometer: 1000.0; "km", "kilometer", "kilometers";
            @meter: 1.0; "m", "meter", "meters";
        }
    }
}

#[macro_use]
mod mass {
    quantity! {
        quantity: Mass; "mass";
        dimension: Q<Z0, P1>;
        units {
            @kilogram: 1000.0 / 1000.0; "kg", "kilogram", "kilograms";
        }
    }
}

system! {
    quantities: Q {
        length: meter, L;
        mass: kilogram, M;
    }
    units: U {
        mod length::Length,
        mod mass::Mass,
    }
}

/// Test trait to allow tests to perform storage-type sensitive comparisons.
#[cfg_attr(rustfmt, rustfmt_skip)]
pub trait Test
    : Debug
    + Sized
    + PartialEq
{
    /// Assert that `lhs` and `rhs` are exactly equal.
    fn assert_eq(lhs: &Self, rhs: &Self) {
        assert_eq!(lhs, rhs);
    }

    /// Assert that `lhs` and `rhs` are approximately equal for floating point types or exactly
    /// equal for other types.
    fn assert_approx_eq(lhs: &Self, rhs: &Self) {
        Test::assert_eq(lhs, rhs);
    }

    /// Exactly compare `lhs` and `rhs` and return the result.
    fn eq(lhs: &Self, rhs: &Self) -> bool {
        lhs == rhs
    }

    /// Approximately compare `lhs` and `rhs` for floating point types or exactly compare for other
    /// types and return the result.
    fn approx_eq(lhs: &Self, rhs: &Self) -> bool {
        Test::eq(lhs, rhs)
    }
}

mod test_trait {
    storage_types! {
        types: Float;

        use num::Float;

        // const EPSILON: V = 64.0 * V::epsilon(); //error[E0015]; calls in constants are limited...
        const EPS_FACTOR: V = 0.5;
        const ULPS: u32 = 3;

        impl super::super::Test for V {
            fn assert_approx_eq(lhs: &Self, rhs: &Self) {
                assert_ulps_eq!(lhs, rhs, epsilon = EPS_FACTOR * V::epsilon(), max_ulps = ULPS);
            }

            fn approx_eq(lhs: &Self, rhs: &Self) -> bool {
                ulps_eq!(lhs, rhs, epsilon = EPS_FACTOR * V::epsilon(), max_ulps = ULPS)
            }
        }
    }

    storage_types! {
        types: PrimInt, BigInt, BigUint, Ratio;

        impl super::super::Test for V {}
    }
}

#[derive(Clone, Debug)]
pub struct A<V> {
    v: V,
}

impl<V> ::lib::ops::Deref for A<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

mod a_struct {
    storage_types! {
        types: Float, PrimInt;

        use super::super::A;

        impl ::quickcheck::Arbitrary for A<V> {
            fn arbitrary<G>(g: &mut G) -> Self
            where
                G: ::quickcheck::Gen,
            {
                A { v: V::arbitrary(g), }
            }
        }
    }

    storage_types! {
        types: BigInt, BigUint, Ratio;

        use num::FromPrimitive;
        use super::super::A;

        impl ::quickcheck::Arbitrary for A<V> {
            fn arbitrary<G>(g: &mut G) -> Self
            where
                G: ::quickcheck::Gen,
            {
                A {
                    v: loop {
                        let v = V::from_f64(<f64 as ::quickcheck::Arbitrary>::arbitrary(g));

                        if let Some(a) = v {
                            break a;
                        }
                    },
                }
            }
        }
    }
}

mod asserts;
mod quantities;
mod quantity;
mod system;
