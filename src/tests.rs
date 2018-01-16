use self::length::{kilometer, meter};
use self::mass::kilogram;
#[allow(unused_imports)]
use {Conversion, ConversionFactor};
#[allow(unused_imports)]
use num::{Float, FromPrimitive, One, Saturating, Signed, Zero};
use quickcheck::TestResult;
use lib::fmt::Debug;
use lib::marker::PhantomData;
#[cfg(feature = "serde")]
use serde_json;
#[allow(unused_imports)]
use typenum::{N1, P1, P2, P3, Z0};

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

mod quantities {
    Q!(tests);
}

/// Test trait to allow tests to perform storage-type sensitive comparisons.
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
        const ULPS: u32 = 16;

        impl super::super::Test for V {
            fn assert_approx_eq(lhs: &Self, rhs: &Self) {
                assert_ulps_eq!(lhs, rhs, epsilon = 64.0 * V::epsilon(), max_ulps = ULPS);
            }

            fn approx_eq(lhs: &Self, rhs: &Self) -> bool {
                ulps_eq!(lhs, rhs, epsilon = 64.0 * V::epsilon(), max_ulps = ULPS)
            }
        }
    }

    storage_types! {
        types: PrimInt, BigInt, BigUint, Ratio;

        impl super::super::Test for V {}
    }
}

#[derive(Clone, Debug)]
struct A<V> {
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

mod quantity_macro {
    use tests::*;

    // Module level constant to verify that creation is possible.
    #[allow(dead_code)]
    #[allow(trivial_numeric_casts)]
    #[cfg(feature = "f32")]
    const LENGTH: quantities::Length<f32> = Quantity {
        dimension: PhantomData,
        units: PhantomData,
        value: 1.0,
    };

    #[test]
    fn description() {
        assert_eq!("length", length::description());
        assert_eq!("mass", mass::description());
    }

    #[test]
    fn abbreviation() {
        assert_eq!("km", kilometer::abbreviation());
        assert_eq!("m", meter::abbreviation());
        assert_eq!("kg", kilogram::abbreviation());
    }

    #[test]
    fn singular() {
        assert_eq!("kilometer", kilometer::singular());
        assert_eq!("meter", meter::singular());
        assert_eq!("kilogram", kilogram::singular());
    }

    #[test]
    fn plural() {
        assert_eq!("kilometers", kilometer::plural());
        assert_eq!("meters", meter::plural());
        assert_eq!("kilograms", kilogram::plural());
    }

    storage_types! {
        use tests::*;

        Q!(tests, V);

        #[test]
        fn struct_literal() {
            let l = Length { dimension: PhantomData, units: PhantomData, value: V::one(), };
            let m = Mass { dimension: PhantomData, units: PhantomData, value: V::one(), };

            Test::assert_eq(&V::one(), &l.value);
            Test::assert_eq(&V::one(), &m.value);
        }

        #[test]
        fn new() {
            let l1 = Length::new::<kilometer>(V::one());
            let l2 = Length::new::<meter>(V::one());
            let m1 = Mass::new::<kilogram>(V::one());

            Test::assert_eq(&V::from_f64(1000.0).unwrap(), &l1.value);
            Test::assert_eq(&V::one(), &l2.value);
            Test::assert_eq(&V::one(), &m1.value);
        }

        #[test]
        fn get() {
            let l1 = Length::new::<kilometer>(V::one());
            let l2 = Length::new::<meter>(V::one());
            let m1 = Mass::new::<kilogram>(V::one());

            Test::assert_eq(&V::from_f64(1000.0).unwrap(), &l1.get(meter));
            Test::assert_eq(&V::one(), &l2.get(meter));
            Test::assert_eq(&V::one(), &l1.get(kilometer));
            Test::assert_eq(&V::from_f64(0.001).unwrap(), &l2.get(kilometer));
            Test::assert_eq(&V::one(), &m1.get(kilogram));
        }

        #[test]
        fn conversion() {
            Test::assert_eq(&V::from_f64(1000.0).unwrap(),
                &<kilometer as Conversion<V>>::conversion().value());
            Test::assert_eq(&V::one(), &<meter as Conversion<V>>::conversion().value());
            Test::assert_eq(&V::one(), &<kilogram as Conversion<V>>::conversion().value());
        }

        #[cfg(feature = "std")]
        #[test]
        fn debug_fmt() {
            assert_eq!(
                format!("{:?} m^1", V::one()),
                format!("{:?}", Length::new::<meter>(V::one())));
            assert_eq!(
                format!("{:?} m^-1", V::one()),
                format!("{:?}", V::one() / Length::new::<meter>(V::one())));
            assert_eq!(
                format!("{:.2?} m^1", V::one()),
                format!("{:.2?}", Length::new::<meter>(V::one())));
            assert_eq!(
                format!("{:?} m^1 kg^1", V::from_f64(1.23).unwrap()),
                format!("{:?}", Length::new::<meter>(V::from_f64(1.23).unwrap()) * Mass::new::<kilogram>(V::one())));
        }
    }

    mod float {
        storage_types! {
            types: Float;

            use tests::*;

            Q!(tests, V);

            #[test]
            fn floor() {
                let l1 = Length::new::<kilometer>(3.9999);
                let l2 = Length::new::<kilometer>(3.0001);
                let l3 = Length::new::<meter>(3.9999);
                let l4 = Length::new::<meter>(3.0001);
                let m1 = Mass::new::<kilogram>(3.9999);
                let m2 = Mass::new::<kilogram>(3.0001);

                Test::assert_eq(&3.0, &l1.floor(kilometer).get(kilometer));
                Test::assert_eq(&3999.0, &l1.floor(meter).get(meter));
                Test::assert_eq(&3.0, &l2.floor(kilometer).get(kilometer));
                Test::assert_eq(&3000.0, &l2.floor(meter).get(meter));
                Test::assert_eq(&0.0, &l3.floor(kilometer).get(kilometer));
                Test::assert_eq(&3.0, &l3.floor(meter).get(meter));
                Test::assert_eq(&0.0, &l4.floor(kilometer).get(kilometer));
                Test::assert_eq(&3.0, &l4.floor(meter).get(meter));
                Test::assert_eq(&3.0, &m1.floor(kilogram).get(kilogram));
                Test::assert_eq(&3.0, &m2.floor(kilogram).get(kilogram));
            }

            #[test]
            fn ceil() {
                let l1 = Length::new::<kilometer>(3.9999);
                let l2 = Length::new::<kilometer>(3.0001);
                let l3 = Length::new::<meter>(3.9999);
                let l4 = Length::new::<meter>(3.0001);
                let m1 = Mass::new::<kilogram>(3.9999);
                let m2 = Mass::new::<kilogram>(3.0001);

                Test::assert_eq(&4.0, &l1.ceil(kilometer).get(kilometer));
                Test::assert_eq(&4000.0, &l1.ceil(meter).get(meter));
                Test::assert_eq(&4.0, &l2.ceil(kilometer).get(kilometer));
                Test::assert_eq(&3001.0, &l2.ceil(meter).get(meter));
                Test::assert_eq(&1.0, &l3.ceil(kilometer).get(kilometer));
                Test::assert_eq(&4.0, &l3.ceil(meter).get(meter));
                Test::assert_eq(&1.0, &l4.ceil(kilometer).get(kilometer));
                Test::assert_eq(&4.0, &l4.ceil(meter).get(meter));
                Test::assert_eq(&4.0, &m1.ceil(kilogram).get(kilogram));
                Test::assert_eq(&4.0, &m2.ceil(kilogram).get(kilogram));
            }

            #[test]
            fn round() {
                let l1 = Length::new::<kilometer>(3.3);
                let l2 = Length::new::<kilometer>(3.5);
                let l3 = Length::new::<meter>(3.3);
                let l4 = Length::new::<meter>(3.5);
                let m1 = Mass::new::<kilogram>(3.3);
                let m2 = Mass::new::<kilogram>(3.5);

                Test::assert_eq(&3.0, &l1.round(kilometer).get(kilometer));
                Test::assert_eq(&3300.0, &l1.round(meter).get(meter));
                Test::assert_eq(&4.0, &l2.round(kilometer).get(kilometer));
                Test::assert_eq(&3500.0, &l2.round(meter).get(meter));
                Test::assert_eq(&0.0, &l3.round(kilometer).get(kilometer));
                Test::assert_eq(&3.0, &l3.round(meter).get(meter));
                Test::assert_eq(&0.0, &l4.round(kilometer).get(kilometer));
                Test::assert_eq(&4.0, &l4.round(meter).get(meter));
                Test::assert_eq(&3.0, &m1.round(kilogram).get(kilogram));
                Test::assert_eq(&4.0, &m2.round(kilogram).get(kilogram));
            }

            #[test]
            fn trunc() {
                let l1 = Length::new::<kilometer>(3.3);
                let l2 = Length::new::<kilometer>(3.5);
                let l3 = Length::new::<meter>(3.3);
                let l4 = Length::new::<meter>(3.5);
                let m1 = Mass::new::<kilogram>(3.3);
                let m2 = Mass::new::<kilogram>(3.5);

                Test::assert_eq(&3.0, &l1.trunc(kilometer).get(kilometer));
                Test::assert_eq(&3300.0, &l1.trunc(meter).get(meter));
                Test::assert_eq(&3.0, &l2.trunc(kilometer).get(kilometer));
                Test::assert_eq(&3500.0, &l2.trunc(meter).get(meter));
                Test::assert_eq(&0.0, &l3.trunc(kilometer).get(kilometer));
                Test::assert_eq(&3.0, &l3.trunc(meter).get(meter));
                Test::assert_eq(&0.0, &l4.trunc(kilometer).get(kilometer));
                Test::assert_eq(&3.0, &l4.trunc(meter).get(meter));
                Test::assert_eq(&3.0, &m1.trunc(kilogram).get(kilogram));
                Test::assert_eq(&3.0, &m2.trunc(kilogram).get(kilogram));
            }

            #[test]
            fn fract() {
                let l1 = Length::new::<kilometer>(3.3);
                let l2 = Length::new::<meter>(3.3);
                let m1 = Mass::new::<kilogram>(3.3);

                Test::assert_eq(&3.3.fract(), &l1.fract(kilometer).get(kilometer));
                Test::assert_eq(&(3.3.fract() * 1000.0), &l1.fract(kilometer).get(meter));
                Test::assert_eq(&((3.3 * 1000.0).fract() / 1000.0),
                    &l1.fract(meter).get(kilometer));
                Test::assert_eq(&(3.3 * 1000.0).fract(), &l1.fract(meter).get(meter));

                Test::assert_eq(&(3.3 / 1000.0).fract(), &l2.fract(kilometer).get(kilometer));
                Test::assert_eq(&((3.3 / 1000.0).fract() * 1000.0),
                    &l2.fract(kilometer).get(meter));
                Test::assert_eq(&(3.3.fract() / 1000.0), &l2.fract(meter).get(kilometer));
                Test::assert_eq(&3.3.fract(), &l2.fract(meter).get(meter));

                Test::assert_eq(&3.3.fract(), &m1.fract(kilogram).get(kilogram));
            }
        }
    }
}

mod system_macro {
    storage_types! {
        use tests::*;

        type MeterKilogram = Units<V, length = meter, mass = kilogram>;
        type KilometerKilogram = Units<V, length = kilometer, mass = kilogram>;

        Q!(tests, V);

        #[test]
        fn zero() {
            let z = Length::zero();

            Test::assert_eq(&V::zero(), &z.value);
            assert!(z.is_zero());
        }

        quickcheck! {
            #[allow(trivial_casts)]
            fn from_base(v: A<V>) -> bool
            {
                let km: V = <kilometer as ::Conversion<V>>::conversion().value();

                // meter -> meter.
                Test::approx_eq(&*v,
                        &::tests::from_base::<length::Dimension, MeterKilogram, V, meter>(&*v))
                    // kilometer -> kilometer.
                    && Test::approx_eq(&*v,
                        &::tests::from_base::<length::Dimension, KilometerKilogram, V, kilometer>(
                            &*v))
                    // meter -> kilometer.
                    && Test::approx_eq(&(&*v / &km),
                        &::tests::from_base::<length::Dimension, MeterKilogram, V, kilometer>(&*v))
                    // kilometer -> meter.
                    && Test::approx_eq(&(&*v * &km),
                        &::tests::from_base::<length::Dimension, KilometerKilogram, V, meter>(&*v))
            }

            #[allow(trivial_casts)]
            fn to_base(v: A<V>) -> bool
            {
                let km: V = <kilometer as ::Conversion<V>>::conversion().value();

                // meter -> meter.
                Test::approx_eq(&*v,
                        &::tests::to_base::<length::Dimension, MeterKilogram, V, meter>(&*v))
                    // kilometer -> kilometer.
                    && Test::approx_eq(&*v,
                        &::tests::to_base::<length::Dimension, KilometerKilogram, V, kilometer>(
                            &*v))
                    // kilometer -> meter.
                    && Test::approx_eq(&(&*v * &km),
                        &::tests::to_base::<length::Dimension, MeterKilogram, V, kilometer>(&*v))
                    // meter -> kilometer.
                    && Test::approx_eq(&(&*v / &km),
                        &::tests::to_base::<length::Dimension, KilometerKilogram, V, meter>(&*v))
            }

            #[allow(trivial_casts)]
            fn change_base(v: A<V>) -> bool
            {
                let km: V = <kilometer as ::Conversion<V>>::conversion().value();

                // meter -> meter.
                Test::approx_eq(&*v,
                        &::tests::change_base::<length::Dimension, MeterKilogram, MeterKilogram, V>(
                            &*v))
                    // kilometer -> kilometer.
                    && Test::approx_eq(&*v,
                        &::tests::change_base::<length::Dimension, KilometerKilogram, KilometerKilogram, V>(
                            &*v))
                    // kilometer -> meter.
                    && Test::approx_eq(&(&*v * &km),
                        &::tests::change_base::<length::Dimension, MeterKilogram, KilometerKilogram, V>(
                            &*v))
                    // meter -> kilometer.
                    && Test::approx_eq(&(&*v / &km),
                        &::tests::change_base::<length::Dimension, KilometerKilogram, MeterKilogram, V>(
                            &*v))
            }

            #[allow(trivial_casts)]
            fn add(l: A<V>, r: A<V>) -> bool {
                Test::eq(&(&*l + &*r),
                    &(Length::new::<meter>((*l).clone())
                        + Length::new::<meter>((*r).clone())).get(meter))
            }

            #[allow(trivial_casts)]
            fn sub(l: A<V>, r: A<V>) -> bool {
                Test::eq(&(&*l - &*r),
                    &(Length::new::<meter>((*l).clone())
                        - Length::new::<meter>((*r).clone())).get(meter))
            }

            #[allow(trivial_casts)]
            fn mul_quantity(l: A<V>, r: A<V>) -> bool {
                // TODO Use `.get(square_meter)`
                Test::eq(&(&*l * &*r),
                    &(Length::new::<meter>((*l).clone())
                        * Length::new::<meter>((*r).clone())).value)
            }

            #[allow(trivial_casts)]
            fn mul_v(l: A<V>, r: A<V>) -> bool {
                Test::eq(&(&*l * &*r),
                        &(Length::new::<meter>((*l).clone()) * (*r).clone()).get(meter))
                    && Test::eq(&(&*l * &*r),
                        &((*l).clone() * Length::new::<meter>((*r).clone())).get(meter))
            }

            #[allow(trivial_casts)]
            fn div_quantity(l: A<V>, r: A<V>) -> TestResult {
                if *r == V::zero() {
                    return TestResult::discard();
                }

                // TODO Use `.get(?)`
                TestResult::from_bool(
                    Test::eq(&(&*l / &*r),
                        &(Length::new::<meter>((*l).clone())
                            / Length::new::<meter>((*r).clone())).value))
            }

            #[allow(trivial_casts)]
            fn div_v(l: A<V>, r: A<V>) -> TestResult {
                if *r == V::zero() {
                    return TestResult::discard();
                }

                // TODO Use `get(meter^-1)`
                TestResult::from_bool(
                    Test::eq(&(&*l / &*r),
                            &(Length::new::<meter>((*l).clone()) / (*r).clone()).get(meter))
                        && Test::eq(&(&*l / &*r),
                            &((*l).clone() / Length::new::<meter>((*r).clone())).value))
            }

            #[allow(trivial_casts)]
            fn rem(l: A<V>, r: A<V>) -> TestResult {
                if *r == V::zero() {
                    return TestResult::discard();
                }

                TestResult::from_bool(
                    Test::approx_eq(&(&*l % &*r),
                        &(Length::new::<meter>((*l).clone())
                            % Length::new::<meter>((*r).clone())).get(meter)))
            }

            #[allow(trivial_casts)]
            fn partial_eq(l: A<V>, r: A<V>) -> bool {
                (*l == *r) == (Length::new::<meter>((*l).clone()) == Length::new::<meter>((*r).clone()))
            }

            #[allow(trivial_casts)]
            fn partial_ord(l: A<V>, r: A<V>) -> bool {
                (l.partial_cmp(&*r)) == (Length::new::<meter>((*l).clone()).partial_cmp(&Length::new::<meter>((*r).clone())))
            }
        }
    }

    mod prim_int {
        storage_types! {
            types: PrimInt;

            use tests::*;

            Q!(tests, V);

            quickcheck! {
                #[allow(trivial_casts)]
                fn saturating_add(l: A<V>, r: A<V>) -> bool {
                    Test::eq(&(l.saturating_add(*r)),
                        &(Length::new::<meter>((*l).clone())
                            .saturating_add(Length::new::<meter>((*r).clone())).get(meter)))
                }

                #[allow(trivial_casts)]
                fn saturating_sub(l: A<V>, r: A<V>) -> bool {
                    Test::eq(&(l.saturating_sub(*r)),
                        &(Length::new::<meter>((*l).clone())
                            .saturating_sub(Length::new::<meter>((*r).clone())).get(meter)))
                }

                #[allow(trivial_casts)]
                fn eq(l: A<V>, r: A<V>) -> bool {
                    (*l == *r) == (Length::new::<meter>((*l).clone()) == Length::new::<meter>((*r).clone()))
                }

                #[allow(trivial_casts)]
                fn ord(l: A<V>, r: A<V>) -> bool {
                    (l.cmp(&*r)) == (Length::new::<meter>((*l).clone()).cmp(&Length::new::<meter>((*r).clone())))
                }
            }
        }
    }

    mod float {
        storage_types! {
            types: Float;

            use tests::*;

            Q!(tests, V);

            #[test]
            fn fp_categories() {
                assert!(!Length::new::<meter>(V::infinity()).is_finite());
                assert!(!Length::new::<meter>(V::neg_infinity()).is_finite());
                assert!(Length::new::<meter>(V::infinity()).is_infinite());
                assert!(Length::new::<meter>(V::neg_infinity()).is_infinite());
                assert!(Length::new::<meter>(V::min_positive_value()).is_normal());
                assert!(Length::new::<meter>(V::max_value()).is_normal());
                assert!(!Length::new::<meter>(V::zero()).is_normal());
                assert!(!Length::new::<meter>(V::nan()).is_normal());
                assert!(!Length::new::<meter>(V::infinity()).is_normal());
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn is_nan(v: A<V>) -> bool {
                    v.is_nan() == Length::new::<meter>(*v).is_nan()
                }

                #[allow(trivial_casts)]
                fn is_infinite(v: A<V>) -> bool {
                    v.is_infinite() == Length::new::<meter>(*v).is_infinite()
                }

                #[allow(trivial_casts)]
                fn is_finite(v: A<V>) -> bool {
                    v.is_finite() == Length::new::<meter>(*v).is_finite()
                }

                #[allow(trivial_casts)]
                fn is_normal(v: A<V>) -> bool {
                    v.is_normal() == Length::new::<meter>(*v).is_normal()
                }

                #[allow(trivial_casts)]
                fn classify(v: A<V>) -> bool {
                    v.classify() == Length::new::<meter>(*v).classify()
                }

                #[allow(trivial_casts)]
                fn cbrt(v: A<V>) -> bool {
                    let l: Quantity<Q<P1, Z0>, U<V>, V> = Quantity::<Q<P3, Z0>, U<V>, V> {
                        dimension: ::lib::marker::PhantomData,
                        units: ::lib::marker::PhantomData,
                        value: *v,
                    }.cbrt();

                    Test::eq(&v.cbrt(), &l.value)
                }

                #[allow(trivial_casts)]
                fn is_sign_positive(v: A<V>) -> bool {
                    v.is_sign_positive() == Length::new::<meter>(*v).is_sign_positive()
                }

                #[allow(trivial_casts)]
                fn is_sign_negative(v: A<V>) -> bool {
                    v.is_sign_negative() == Length::new::<meter>(*v).is_sign_negative()
                }

                #[allow(trivial_casts)]
                fn mul_add(s: A<V>, a: A<V>, b: A<V>) -> bool {
                    let r: Quantity<Q<P2, Z0>, U<V>, V> = Length::new::<meter>(*s).mul_add(
                        Length::new::<meter>(*a),
                        Quantity::<Q<P2, Z0>, U<V>, V> {
                            dimension: ::lib::marker::PhantomData,
                            units: ::lib::marker::PhantomData,
                            value: *b
                        });

                    Test::eq(&s.mul_add(*a, *b), &r.value)
                }

                #[allow(trivial_casts)]
                fn recip(v: A<V>) -> bool {
                    let a: Quantity<Q<N1, Z0>, U<V>, V> = Quantity::<Q<P1, Z0>, U<V>, V> {
                        dimension: ::lib::marker::PhantomData,
                        units: ::lib::marker::PhantomData,
                        value: *v,
                    }.recip();

                    Test::eq(&v.recip(), &a.value)
                }

                #[allow(trivial_casts)]
                fn powi(v: A<V>) -> bool {
                    Test::eq(&v.powi(3), &Length::new::<meter>(*v).powi(P3::new()).value)
                }

                #[allow(trivial_casts)]
                fn sqrt(v: A<V>) -> TestResult {
                    if *v < V::zero() {
                        return TestResult::discard();
                    }

                    let l: Quantity<Q<P1, Z0>, U<V>, V> = Quantity::<Q<P2, Z0>, U<V>, V> {
                        dimension: ::lib::marker::PhantomData,
                        units: ::lib::marker::PhantomData,
                        value: *v,
                    }.sqrt();

                    TestResult::from_bool(Test::eq(&v.sqrt(), &l.value))
                }

                #[allow(trivial_casts)]
                fn max(l: A<V>, r: A<V>) -> bool {
                    Test::eq(&l.max(*r),
                        &Length::new::<meter>(*l).max(Length::new::<meter>(*r)).get(meter))
                }

                #[allow(trivial_casts)]
                fn min(l: A<V>, r: A<V>) -> bool {
                    Test::eq(&l.min(*r),
                        &Length::new::<meter>(*l).min(Length::new::<meter>(*r)).get(meter))
                }
            }
        }
    }

    mod signed {
        storage_types! {
            types: Signed;

            use tests::*;

            Q!(tests, V);

            quickcheck! {
                #[allow(trivial_casts)]
                fn abs(v: A<V>) -> bool {
                    Test::eq(&v.abs(), &Length::new::<meter>((*v).clone()).abs().get(meter))
                }

                #[allow(trivial_casts)]
                fn signum(v: A<V>) -> bool {
                    Test::eq(&v.signum(), &Length::new::<meter>((*v).clone()).signum().get(meter))
                }

                #[allow(trivial_casts)]
                fn neg(l: A<V>) -> bool {
                    Test::eq(&-(*l).clone(), &-Length::new::<meter>((*l).clone()).get(meter))
                }
            }
        }
    }

    mod primitive {
        storage_types! {
            types: Float, PrimInt;

            use tests::*;

            Q!(tests, V);

            quickcheck! {
                #[allow(trivial_casts)]
                fn add_assign(l: A<V>, r: A<V>) -> bool {
                    let mut f = *l;
                    let mut v = Length::new::<meter>(*l);

                    f += *r;
                    v += Length::new::<meter>(*r);

                    Test::eq(&f, &v.get(meter))
                }

                #[allow(trivial_casts)]
                fn sub_assign(l: A<V>, r: A<V>) -> bool {
                    let mut f = *l;
                    let mut v = Length::new::<meter>(*l);

                    f -= *r;
                    v -= Length::new::<meter>(*r);

                    Test::eq(&f, &v.get(meter))
                }

                #[allow(trivial_casts)]
                fn mul_assign(l: A<V>, r: A<V>) -> bool {
                    let mut f = *l;
                    let mut v = Length::new::<meter>(*l);

                    f *= *r;
                    v *= *r;

                    Test::eq(&f, &v.get(meter))
                }

                #[allow(trivial_casts)]
                fn div_assign(l: A<V>, r: A<V>) -> TestResult {
                    if *r == V::zero() {
                        return TestResult::discard();
                    }

                    let mut f = *l;
                    let mut v = Length::new::<meter>(*l);

                    f /= *r;
                    v /= *r;

                    TestResult::from_bool(Test::eq(&f, &v.get(meter)))
                }

                #[allow(trivial_casts)]
                fn rem_assign(l: A<V>, r: A<V>) -> TestResult {
                    if *r == V::zero() {
                        return TestResult::discard();
                    }

                    let mut f = *l;
                    let mut v = Length::new::<meter>(*l);

                    f %= *r;
                    v %= Length::new::<meter>(*r);

                    TestResult::from_bool(Test::approx_eq(&f, &v.get(meter)))
                }

                // These serde tests can't be run against num-backed numeric backends because the num
                // crate hasn't been updated to Serde 1.0 yet.
                #[cfg(feature = "serde")]
                #[allow(trivial_casts)]
                fn serde_serialize(v: A<V>) -> bool {
                    let m = Length::new::<meter>((*v).clone());
                    let json_f = serde_json::to_string(&*v).expect("Must be able to serialize num");
                    let json_q = serde_json::to_string(&m).expect("Must be able to serialize Quantity");

                    json_f == json_q
                }

                #[cfg(feature = "serde")]
                #[allow(trivial_casts)]
                fn serde_deserialize(v: A<V>) -> bool {
                    let json_f = serde_json::to_string(&*v).expect("Must be able to serialize num");
                    let length: Length = serde_json::from_str(&json_f)
                        .expect("Must be able to deserialize Quantity");

                    Test::approx_eq(&*v, &length.get(meter))
                }
            }
        }
    }
}

mod quantities_macro {
    mod fractional {
        storage_types! {
            types: Float, Ratio;

            use tests::*;

            mod f { Q!(tests, super::V); }
            mod k { Q!(tests, super::V, (kilometer, kilogram)); }

            #[test]
            fn new() {
                let l1 = k::Length::new::<kilometer>(V::one());
                let l2 = k::Length::new::<meter>(V::one());
                let m1 = k::Mass::new::<kilogram>(V::one());

                Test::assert_eq(&V::one(), &l1.value);
                Test::assert_eq(&V::from_f64(1.0_E-3).unwrap(), &l2.value);
                Test::assert_eq(&V::one(), &m1.value);
            }

            #[test]
            fn get() {
                let l1 = k::Length::new::<kilometer>(V::one());
                let l2 = k::Length::new::<meter>(V::one());
                let m1 = k::Mass::new::<kilogram>(V::one());

                Test::assert_eq(&V::from_f64(1000.0).unwrap(), &l1.get(meter));
                Test::assert_eq(&V::one(), &l2.get(meter));
                Test::assert_eq(&V::one(), &l1.get(kilometer));
                Test::assert_eq(&V::from_f64(0.001).unwrap(), &l2.get(kilometer));
                Test::assert_eq(&V::one(), &m1.get(kilogram));
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn add(l: A<V>, r: A<V>) -> bool {
                    Test::approx_eq(&(&*l + &*r),
                        &(k::Length::new::<meter>((*l).clone())
                            + f::Length::new::<meter>((*r).clone())).get(meter))
                }

                #[allow(trivial_casts)]
                fn sub(l: A<V>, r: A<V>) -> bool {
                    Test::approx_eq(&(&*l - &*r),
                        &(k::Length::new::<meter>((*l).clone())
                            - f::Length::new::<meter>((*r).clone())).get(meter))
                }

                #[allow(trivial_casts)]
                fn mul_quantity(l: A<V>, r: A<V>) -> bool {
                    // TODO Use `.get(square_meter)`
                    Test::approx_eq(&(&*l * &*r),
                            &(f::Length::new::<meter>((*l).clone())
                                * k::Length::new::<meter>((*r).clone())).value)
                        && Test::approx_eq(&(&*l * &*r),
                            &(f::Length::new::<meter>((*l).clone())
                                * k::Mass::new::<kilogram>((*r).clone())).value)
                        && Test::approx_eq(&(&*l * &*r),
                            &(k::Length::new::<kilometer>((*l).clone())
                                * f::Mass::new::<kilogram>((*r).clone())).value)
                }

                #[allow(trivial_casts)]
                fn div_quantity(l: A<V>, r: A<V>) -> TestResult {
                    if *r == V::zero() {
                        return TestResult::discard();
                    }

                    // TODO Use `.get(?)`
                    TestResult::from_bool(
                        Test::approx_eq(&(&*l / &*r),
                            &(k::Length::new::<meter>((*l).clone())
                                / f::Length::new::<meter>((*r).clone())).value))
                }

                #[allow(trivial_casts)]
                fn rem(l: A<V>, r: A<V>) -> TestResult {
                    if *r == V::zero() {
                        return TestResult::discard();
                    }

                    TestResult::from_bool(
                        Test::approx_eq(&(&*l % &*r),
                            &(k::Length::new::<meter>((*l).clone())
                                % f::Length::new::<meter>((*r).clone())).get(meter)))
                }
            }
        }
    }

    mod float {
        storage_types! {
            types: Float;

            use tests::*;

            mod f { Q!(tests, super::V); }
            mod k { Q!(tests, super::V, (kilometer, kilogram)); }

            quickcheck! {
                #[allow(trivial_casts)]
                fn add_assign(l: A<V>, r: A<V>) -> bool {
                    let mut f = *l;
                    let mut v = k::Length::new::<meter>(*l);

                    f += *r;
                    v += f::Length::new::<meter>(*r);

                    Test::approx_eq(&f, &v.get(meter))
                }

                #[allow(trivial_casts)]
                fn sub_assign(l: A<V>, r: A<V>) -> bool {
                    let mut f = *l;
                    let mut v = k::Length::new::<meter>(*l);

                    f -= *r;
                    v -= f::Length::new::<meter>(*r);

                    Test::approx_eq(&f, &v.get(meter))
                }

                #[allow(trivial_casts)]
                fn rem_assign(l: A<V>, r: A<V>) -> TestResult {
                    if *r == V::zero() {
                        return TestResult::discard();
                    }

                    let mut f = *l;
                    let mut v = k::Length::new::<meter>(*l);

                    f %= *r;
                    v %= f::Length::new::<meter>(*r);

                    TestResult::from_bool(Test::approx_eq(&f, &v.get(meter)))
                }
            }

            #[test]
            fn floor() {
                let l1 = k::Length::new::<kilometer>(3.9999);
                let l2 = k::Length::new::<kilometer>(3.0001);
                let l3 = k::Length::new::<meter>(3.9999);
                let l4 = k::Length::new::<meter>(3.0001);
                let m1 = k::Mass::new::<kilogram>(3.9999);
                let m2 = k::Mass::new::<kilogram>(3.0001);

                Test::assert_eq(&3.0, &l1.floor(kilometer).get(kilometer));
                Test::assert_eq(&3999.0, &l1.floor(meter).get(meter));
                Test::assert_eq(&3.0, &l2.floor(kilometer).get(kilometer));
                Test::assert_eq(&3000.0, &l2.floor(meter).get(meter));
                Test::assert_eq(&0.0, &l3.floor(kilometer).get(kilometer));
                Test::assert_eq(&3.0, &l3.floor(meter).get(meter));
                Test::assert_eq(&0.0, &l4.floor(kilometer).get(kilometer));
                Test::assert_eq(&3.0, &l4.floor(meter).get(meter));
                Test::assert_eq(&3.0, &m1.floor(kilogram).get(kilogram));
                Test::assert_eq(&3.0, &m2.floor(kilogram).get(kilogram));
            }

            #[test]
            fn ceil() {
                let l1 = k::Length::new::<kilometer>(3.9999);
                let l2 = k::Length::new::<kilometer>(3.0001);
                let l3 = k::Length::new::<meter>(3.9999);
                let l4 = k::Length::new::<meter>(3.0001);
                let m1 = k::Mass::new::<kilogram>(3.9999);
                let m2 = k::Mass::new::<kilogram>(3.0001);

                Test::assert_eq(&4.0, &l1.ceil(kilometer).get(kilometer));
                Test::assert_eq(&4000.0, &l1.ceil(meter).get(meter));
                Test::assert_eq(&4.0, &l2.ceil(kilometer).get(kilometer));
                Test::assert_eq(&3001.0, &l2.ceil(meter).get(meter));
                Test::assert_eq(&1.0, &l3.ceil(kilometer).get(kilometer));
                Test::assert_eq(&4.0, &l3.ceil(meter).get(meter));
                Test::assert_eq(&1.0, &l4.ceil(kilometer).get(kilometer));
                Test::assert_eq(&4.0, &l4.ceil(meter).get(meter));
                Test::assert_eq(&4.0, &m1.ceil(kilogram).get(kilogram));
                Test::assert_eq(&4.0, &m2.ceil(kilogram).get(kilogram));
            }

            #[test]
            fn round() {
                let l1 = k::Length::new::<kilometer>(3.3);
                let l2 = k::Length::new::<kilometer>(3.5);
                let l3 = k::Length::new::<meter>(3.3);
                let l4 = k::Length::new::<meter>(3.5);
                let m1 = k::Mass::new::<kilogram>(3.3);
                let m2 = k::Mass::new::<kilogram>(3.5);

                Test::assert_eq(&3.0, &l1.round(kilometer).get(kilometer));
                Test::assert_eq(&3300.0, &l1.round(meter).get(meter));
                Test::assert_eq(&4.0, &l2.round(kilometer).get(kilometer));
                Test::assert_eq(&3500.0, &l2.round(meter).get(meter));
                Test::assert_eq(&0.0, &l3.round(kilometer).get(kilometer));
                Test::assert_eq(&3.0, &l3.round(meter).get(meter));
                Test::assert_eq(&0.0, &l4.round(kilometer).get(kilometer));
                Test::assert_eq(&4.0, &l4.round(meter).get(meter));
                Test::assert_eq(&3.0, &m1.round(kilogram).get(kilogram));
                Test::assert_eq(&4.0, &m2.round(kilogram).get(kilogram));
            }

            #[test]
            fn trunc() {
                let l1 = k::Length::new::<kilometer>(3.3);
                let l2 = k::Length::new::<kilometer>(3.5);
                let l3 = k::Length::new::<meter>(3.3);
                let l4 = k::Length::new::<meter>(3.5);
                let m1 = k::Mass::new::<kilogram>(3.3);
                let m2 = k::Mass::new::<kilogram>(3.5);

                Test::assert_eq(&3.0, &l1.trunc(kilometer).get(kilometer));
                Test::assert_eq(&3300.0, &l1.trunc(meter).get(meter));
                Test::assert_eq(&3.0, &l2.trunc(kilometer).get(kilometer));
                Test::assert_eq(&3500.0, &l2.trunc(meter).get(meter));
                Test::assert_eq(&0.0, &l3.trunc(kilometer).get(kilometer));
                Test::assert_eq(&3.0, &l3.trunc(meter).get(meter));
                Test::assert_eq(&0.0, &l4.trunc(kilometer).get(kilometer));
                Test::assert_eq(&3.0, &l4.trunc(meter).get(meter));
                Test::assert_eq(&3.0, &m1.trunc(kilogram).get(kilogram));
                Test::assert_eq(&3.0, &m2.trunc(kilogram).get(kilogram));
            }

            #[test]
            fn fract() {
                let l1 = k::Length::new::<kilometer>(3.3);
                let l2 = k::Length::new::<meter>(3.3);
                let m1 = k::Mass::new::<kilogram>(3.3);

                Test::assert_eq(&3.3.fract(), &l1.fract(kilometer).get(kilometer));
                Test::assert_eq(&(3.3.fract() * 1000.0), &l1.fract(kilometer).get(meter));
                Test::assert_eq(&((3.3 * 1000.0).fract() / 1000.0),
                    &l1.fract(meter).get(kilometer));
                Test::assert_eq(&(3.3 * 1000.0).fract(), &l1.fract(meter).get(meter));

                Test::assert_eq(&(3.3 / 1000.0).fract(), &l2.fract(kilometer).get(kilometer));
                Test::assert_eq(&((3.3 / 1000.0).fract() * 1000.0),
                    &l2.fract(kilometer).get(meter));
                Test::assert_eq(&(3.3.fract() / 1000.0), &l2.fract(meter).get(kilometer));
                Test::assert_eq(&3.3.fract(), &l2.fract(meter).get(meter));

                Test::assert_eq(&3.3.fract(), &m1.fract(kilogram).get(kilogram));
            }
        }
    }
}

mod static_checks {
    storage_types! {
        types: Float;

        use tests::*;

        assert_impl!(q; Quantity<Q<Z0, Z0>, U<V>, V>, Clone, Copy, Send, Sync);
    }

    storage_types! {
        types: PrimInt, Rational, Rational32, Rational64;

        use tests::*;

        assert_impl!(q; Quantity<Q<Z0, Z0>, U<V>, V>, Clone, Copy, Send, Sync, ::lib::hash::Hash);
    }

    storage_types! {
        types: BigInt, BigUint, BigRational;

        use tests::*;

        assert_impl!(q; Quantity<Q<Z0, Z0>, U<V>, V>, Clone, Send, Sync, ::lib::hash::Hash);
    }
}
