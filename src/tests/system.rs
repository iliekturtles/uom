//! Tests for the `system!` macro.

storage_types! {
    use tests::*;

    type MeterKelvinBase = dyn Units<V, length = meter, mass = kilogram,
        thermodynamic_temperature = kelvin>;
    type KilometerFahrenheitBase = dyn Units<V, length = kilometer, mass = kilogram,
        thermodynamic_temperature = degree_fahrenheit>;

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
            let km: V = <kilometer as ::Conversion<V>>::coefficient().value();
            let f_coefficient: V = <degree_fahrenheit as ::Conversion<V>>::coefficient().value();
            let f_constant: V =
                <degree_fahrenheit as ::Conversion<V>>::constant(ConstantOp::Add).value();

            // meter -> meter.
            Test::approx_eq(&*v,
                    &::tests::from_base::<length::Dimension, MeterKelvinBase, V, meter>(&*v))
                // kilometer -> kilometer.
                && Test::approx_eq(&*v,
                    &::tests::from_base::<length::Dimension, KilometerFahrenheitBase, V, kilometer>(
                        &*v))
                // meter -> kilometer.
                && Test::approx_eq(&(&*v / &km),
                    &::tests::from_base::<length::Dimension, MeterKelvinBase, V, kilometer>(&*v))
                // kilometer -> meter.
                && Test::approx_eq(&(&*v * &km),
                    &::tests::from_base::<length::Dimension, KilometerFahrenheitBase, V, meter>(
                        &*v))
                // kelvin -> kelvin.
                && Test::approx_eq(&*v,
                    &::tests::from_base::<thermodynamic_temperature::Dimension, MeterKelvinBase, V, kelvin>(
                        &*v))
                // fahrenheit -> fahrenheit.
                // && Test::approx_eq(&*v,
                //     &::tests::from_base::<thermodynamic_temperature::Dimension, KilometerFahrenheitBase, V, degree_fahrenheit>(
                //         &*v))
                // kelvin -> fahrenheit.
                && Test::approx_eq(&(&*v / &f_coefficient - &f_constant),
                    &::tests::from_base::<thermodynamic_temperature::Dimension, MeterKelvinBase, V, degree_fahrenheit>(
                        &*v))
                // fahrenheit -> kelvin.
                // && Test::approx_eq(&(&(&*v + &f_constant) * &f_coefficient),
                //     &::tests::from_base::<thermodynamic_temperature::Dimension, KilometerFahrenheitBase, V, kelvin>(
                //         &*v))
        }

        #[allow(trivial_casts)]
        fn to_base(v: A<V>) -> bool
        {
            let km: V = <kilometer as ::Conversion<V>>::coefficient().value();
            let f_coefficient: V = <degree_fahrenheit as ::Conversion<V>>::coefficient().value();
            let f_constant: V =
                <degree_fahrenheit as ::Conversion<V>>::constant(ConstantOp::Add).value();

            // meter -> meter.
            Test::approx_eq(&*v,
                    &::tests::to_base::<length::Dimension, MeterKelvinBase, V, meter>(&*v))
                // kilometer -> kilometer.
                && Test::approx_eq(&*v,
                    &::tests::to_base::<length::Dimension, KilometerFahrenheitBase, V, kilometer>(
                        &*v))
                // kilometer -> meter.
                && Test::approx_eq(&(&*v * &km),
                    &::tests::to_base::<length::Dimension, MeterKelvinBase, V, kilometer>(&*v))
                // meter -> kilometer.
                && Test::approx_eq(&(&*v / &km),
                    &::tests::to_base::<length::Dimension, KilometerFahrenheitBase, V, meter>(&*v))
                // kelvin -> kelvin.
                && Test::approx_eq(&*v,
                    &::tests::to_base::<thermodynamic_temperature::Dimension, MeterKelvinBase, V, kelvin>(
                        &*v))
                // fahrenheit -> fahrenheit.
                // && Test::approx_eq(&*v,
                //     &::tests::to_base::<thermodynamic_temperature::Dimension, KilometerFahrenheitBase, V, degree_fahrenheit>(
                //         &*v))
                // fahrenheit -> kelvin.
                && Test::approx_eq(&(&(&*v + &f_constant) * &f_coefficient),
                    &::tests::to_base::<thermodynamic_temperature::Dimension, MeterKelvinBase, V, degree_fahrenheit>(
                        &*v))
                // kelvin -> fahrenheit.
                // && Test::approx_eq(&(&*v / &f_coefficient - &f_constant),
                //     &::tests::to_base::<thermodynamic_temperature::Dimension, KilometerFahrenheitBase, V, kelvin>(
                //         &*v))
        }

        #[allow(trivial_casts)]
        fn change_base(v: A<V>) -> bool
        {
            let km: V = <kilometer as ::Conversion<V>>::coefficient().value();

            // meter -> meter.
            Test::approx_eq(&*v,
                    &::tests::change_base::<length::Dimension, MeterKelvinBase, MeterKelvinBase, V>(
                        &*v))
                // kilometer -> kilometer.
                && Test::approx_eq(&*v,
                    &::tests::change_base::<length::Dimension, KilometerFahrenheitBase, KilometerFahrenheitBase, V>(
                        &*v))
                // kilometer -> meter.
                && Test::approx_eq(&(&*v * &km),
                    &::tests::change_base::<length::Dimension, MeterKelvinBase, KilometerFahrenheitBase, V>(
                        &*v))
                // meter -> kilometer.
                && Test::approx_eq(&(&*v / &km),
                    &::tests::change_base::<length::Dimension, KilometerFahrenheitBase, MeterKelvinBase, V>(
                        &*v))
        }

        #[allow(trivial_casts)]
        fn add(l: A<V>, r: A<V>) -> bool {
            Test::eq(&Length::new::<meter>(&*l + &*r),
                &(Length::new::<meter>((*l).clone())
                    + Length::new::<meter>((*r).clone())))
        }

        #[allow(trivial_casts)]
        fn sub(l: A<V>, r: A<V>) -> bool {
            Test::eq(&Length::new::<meter>(&*l - &*r),
                &(Length::new::<meter>((*l).clone())
                    - Length::new::<meter>((*r).clone())))
        }

        #[allow(trivial_casts)]
        fn mul_quantity(l: A<V>, r: A<V>) -> bool {
            Test::eq(&/*Area::new::<square_meter>*/(&*l * &*r),
                &(Length::new::<meter>((*l).clone())
                    * Length::new::<meter>((*r).clone())).value)
        }

        #[allow(trivial_casts)]
        fn mul_v(l: A<V>, r: A<V>) -> bool {
            Test::eq(&Length::new::<meter>(&*l * &*r),
                    &(Length::new::<meter>((*l).clone()) * (*r).clone()))
                && Test::eq(&Length::new::<meter>(&*l * &*r),
                    &((*l).clone() * Length::new::<meter>((*r).clone())))
        }

        #[allow(trivial_casts)]
        fn div_quantity(l: A<V>, r: A<V>) -> TestResult {
            if *r == V::zero() {
                return TestResult::discard();
            }

            // TODO Use `.get(?)`, add ratio type?
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

            TestResult::from_bool(
                Test::eq(&Length::new::<meter>(&*l / &*r),
                        &(Length::new::<meter>((*l).clone()) / (*r).clone()))
                    && Test::eq(&/*ReciprocalLength::new::<meter>*/(&*l / &*r),
                        &((*l).clone() / Length::new::<meter>((*r).clone())).value))
        }

        #[allow(trivial_casts)]
        fn eq(l: A<V>, r: A<V>) -> bool {
            (*l == *r)
                == (Length::new::<meter>((*l).clone()) == Length::new::<meter>((*r).clone()))
        }

        #[allow(trivial_casts)]
        fn ne(l: A<V>, r: A<V>) -> bool {
            (*l != *r)
                == (Length::new::<meter>((*l).clone()) != Length::new::<meter>((*r).clone()))
        }

        #[allow(trivial_casts)]
        fn partial_cmp(l: A<V>, r: A<V>) -> bool {
            (*l).partial_cmp(&*r)
                == Length::new::<meter>((*l).clone()).partial_cmp(
                    &Length::new::<meter>((*r).clone()))
        }

        #[allow(trivial_casts)]
        fn lt(l: A<V>, r: A<V>) -> bool {
            (*l).lt(&*r)
                == Length::new::<meter>((*l).clone()).lt(&Length::new::<meter>((*r).clone()))
        }

        #[allow(trivial_casts)]
        fn le(l: A<V>, r: A<V>) -> bool {
            (*l).le(&*r)
                == Length::new::<meter>((*l).clone()).le(&Length::new::<meter>((*r).clone()))
        }

        #[allow(trivial_casts)]
        fn gt(l: A<V>, r: A<V>) -> bool {
            (*l).gt(&*r)
                == Length::new::<meter>((*l).clone()).gt(&Length::new::<meter>((*r).clone()))
        }

        #[allow(trivial_casts)]
        fn ge(l: A<V>, r: A<V>) -> bool {
            (*l).ge(&*r)
                == Length::new::<meter>((*l).clone()).ge(&Length::new::<meter>((*r).clone()))
        }

        #[allow(trivial_casts)]
        fn rem(l: A<V>, r: A<V>) -> TestResult {
            if *r == V::zero() {
                return TestResult::discard();
            }

            TestResult::from_bool(
                Test::approx_eq(&Length::new::<meter>(&*l % &*r),
                    &(Length::new::<meter>((*l).clone())
                        % Length::new::<meter>((*r).clone()))))
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
                        .saturating_add(Length::new::<meter>((*r).clone())).get::<meter>()))
            }

            #[allow(trivial_casts)]
            fn saturating_sub(l: A<V>, r: A<V>) -> bool {
                Test::eq(&(l.saturating_sub(*r)),
                    &(Length::new::<meter>((*l).clone())
                        .saturating_sub(Length::new::<meter>((*r).clone())).get::<meter>()))
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

            #[cfg(feature = "std")]
            #[allow(trivial_casts)]
            fn cbrt(v: A<V>) -> bool {
                let l: Quantity<Q<P1, Z0, Z0>, U<V>, V> = Quantity::<Q<P3, Z0, Z0>, U<V>, V> {
                    dimension: ::lib::marker::PhantomData,
                    units: ::lib::marker::PhantomData,
                    value: *v,
                }.cbrt();

                Test::eq(&v.cbrt(), &l.value)
            }

            #[cfg(feature = "std")]
            #[allow(trivial_casts)]
            fn hypot(l: A<V>, r: A<V>) -> bool {
                Test::eq(&Length::new::<meter>(l.hypot(*r)),
                    &Length::new::<meter>(*l).hypot(Length::new::<meter>(*r)))
            }

            #[allow(trivial_casts)]
            fn is_sign_positive(v: A<V>) -> bool {
                v.is_sign_positive() == Length::new::<meter>(*v).is_sign_positive()
            }

            #[allow(trivial_casts)]
            fn is_sign_negative(v: A<V>) -> bool {
                v.is_sign_negative() == Length::new::<meter>(*v).is_sign_negative()
            }

            #[cfg(feature = "std")]
            #[allow(trivial_casts)]
            fn mul_add(s: A<V>, a: A<V>, b: A<V>) -> bool {
                let r: Quantity<Q<P2, Z0, Z0>, U<V>, V> = Length::new::<meter>(*s).mul_add(
                    Length::new::<meter>(*a),
                    Quantity::<Q<P2, Z0, Z0>, U<V>, V> {
                        dimension: ::lib::marker::PhantomData,
                        units: ::lib::marker::PhantomData,
                        value: *b
                    });

                Test::eq(&s.mul_add(*a, *b), &r.value)
            }

            #[allow(trivial_casts)]
            fn recip(v: A<V>) -> bool {
                let a: Quantity<Q<N1, Z0, Z0>, U<V>, V> = Quantity::<Q<P1, Z0, Z0>, U<V>, V> {
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

            #[cfg(feature = "std")]
            #[allow(trivial_casts)]
            fn sqrt(v: A<V>) -> TestResult {
                if *v < V::zero() {
                    return TestResult::discard();
                }

                let l: Quantity<Q<P1, Z0, Z0>, U<V>, V> = Quantity::<Q<P2, Z0, Z0>, U<V>, V> {
                    dimension: ::lib::marker::PhantomData,
                    units: ::lib::marker::PhantomData,
                    value: *v,
                }.sqrt();

                TestResult::from_bool(Test::eq(&v.sqrt(), &l.value))
            }

            #[allow(trivial_casts)]
            fn max(l: A<V>, r: A<V>) -> bool {
                Test::eq(&Length::new::<meter>(l.max(*r)),
                    &Length::new::<meter>(*l).max(Length::new::<meter>(*r)))
            }

            #[allow(trivial_casts)]
            fn min(l: A<V>, r: A<V>) -> bool {
                Test::eq(&Length::new::<meter>(l.min(*r)),
                    &Length::new::<meter>(*l).min(Length::new::<meter>(*r)))
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
                Test::eq(&Length::new::<meter>(v.abs()),
                    &Length::new::<meter>((*v).clone()).abs())
            }

            #[allow(trivial_casts)]
            fn signum(v: A<V>) -> bool {
                Test::eq(&Length::new::<meter>(v.signum()),
                    &Length::new::<meter>((*v).clone()).signum())
            }

            #[allow(trivial_casts)]
            fn neg(l: A<V>) -> bool {
                Test::eq(&Length::new::<meter>(-(*l).clone()),
                    &-Length::new::<meter>((*l).clone()))
            }
        }
    }
}

mod non_ratio {
    storage_types! {
        types: PrimInt, BigInt, BigUint, Float;

        use tests::*;

        Q!(tests, V);

        #[test]
        fn default() {
           Test::assert_eq(&Length::new::<meter>(V::default()), &Length::default());
        }
    }
}

mod non_big {
    storage_types! {
        types: PrimInt, Rational, Rational32, Rational64, Float;

        use tests::*;

        Q!(tests, V);

        quickcheck! {
            #[allow(trivial_casts)]
            fn add_assign(l: A<V>, r: A<V>) -> bool {
                let mut f = *l;
                let mut v = Length::new::<meter>(*l);

                f += *r;
                v += Length::new::<meter>(*r);

                Test::eq(&Length::new::<meter>(f), &v)
            }

            #[allow(trivial_casts)]
            fn sub_assign(l: A<V>, r: A<V>) -> bool {
                let mut f = *l;
                let mut v = Length::new::<meter>(*l);

                f -= *r;
                v -= Length::new::<meter>(*r);

                Test::eq(&Length::new::<meter>(f), &v)
            }

            #[allow(trivial_casts)]
            fn mul_assign(l: A<V>, r: A<V>) -> bool {
                let mut f = *l;
                let mut v = Length::new::<meter>(*l);

                f *= *r;
                v *= *r;

                Test::eq(&Length::new::<meter>(f), &v)
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

                TestResult::from_bool(Test::eq(&Length::new::<meter>(f), &v))
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

                TestResult::from_bool(Test::approx_eq(&Length::new::<meter>(f), &v))
            }
        }
    }
}

mod primitive {
    storage_types! {
        types: Float, PrimInt;

        use tests::*;

        Q!(tests, V);

        #[test]
        fn sum() {
            Test::assert_eq(
                &Length::new::<meter>(
                    (1..10).map(V::from_i32).map(Option::unwrap).sum()),
                &(1..10).map(V::from_i32).map(Option::unwrap)
                    .map(|v| { Length::new::<meter>(v) }).sum());
        }

        #[cfg(feature = "serde")]
        quickcheck! {
            // These serde tests can't be run against num-backed numeric backends because the
            // num crate hasn't been updated to Serde 1.0 yet.
            #[allow(trivial_casts)]
            fn serde_serialize(v: A<V>) -> bool {
                let m = Length::new::<meter>((*v).clone());
                let json_f = serde_json::to_string(&*v).expect("Must be able to serialize num");
                let json_q = serde_json::to_string(&m).expect("Must be able to serialize Quantity");

                json_f == json_q
            }

            #[allow(trivial_casts)]
            fn serde_deserialize(v: A<V>) -> bool {
                let json_f = serde_json::to_string(&*v).expect("Must be able to serialize num");
                let length: Length = serde_json::from_str(&json_f)
                    .expect("Must be able to deserialize Quantity");

                Test::approx_eq(&Length::new::<meter>(*v), &length)
            }
        }
    }
}

mod fixed {
    storage_types! {
        types: PrimInt, BigInt, BigUint, Ratio;

        use tests::*;

        Q!(tests, V);

        quickcheck! {
            #[allow(trivial_casts)]
            fn cmp(l: A<V>, r: A<V>) -> bool {
                (*l).cmp(&*r)
                    == Length::new::<meter>((*l).clone()).cmp(
                        &Length::new::<meter>((*r).clone()))
            }

            #[allow(trivial_casts)]
            fn max(l: A<V>, r: A<V>) -> bool {
                Test::eq(&Length::new::<meter>((*l).clone().max((*r).clone())),
                    &Ord::max(Length::new::<meter>((*l).clone()),
                        Length::new::<meter>((*r).clone())))
            }

            #[allow(trivial_casts)]
            fn min(l: A<V>, r: A<V>) -> bool {
                Test::eq(&Length::new::<meter>((*l).clone().min((*r).clone())),
                    &Ord::min(Length::new::<meter>((*l).clone()),
                        Length::new::<meter>((*r).clone())))
            }
        }
    }
}
