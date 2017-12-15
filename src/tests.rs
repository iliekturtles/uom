use stdlib::marker::PhantomData;
#[allow(unused_imports)]
use num::{Float, FromPrimitive, One, Signed, Zero};
use quickcheck::TestResult;
use typenum::{N1, P1, P2, P3, Z0};
#[allow(unused_imports)]
use {Conversion, ConversionFactor};
use self::length::{kilometer, meter};
use self::mass::kilogram;

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

mod quantity_macro {
    use tests::*;

    // Module level constant to verify that creation is possible.
    #[allow(dead_code)]
    #[allow(trivial_numeric_casts)]
    #[cfg(feature ="f32")]
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
        types: Float;

        use tests::*;

        Q!(tests, V);

        #[test]
        fn struct_literal() {
            let l = Length { dimension: PhantomData, units: PhantomData, value: V::one(), };
            let m = Mass { dimension: PhantomData, units: PhantomData, value: V::one(), };

            assert_eq!(V::one(), l.value);
            assert_eq!(V::one(), m.value);
        }

        #[test]
        fn new() {
            let l1 = Length::new::<kilometer>(V::one());
            let l2 = Length::new::<meter>(V::one());
            let m1 = Mass::new::<kilogram>(V::one());

            assert_eq!(1000.0, l1.value);
            assert_eq!(V::one(), l2.value);
            assert_eq!(V::one(), m1.value);
        }

        #[test]
        fn get() {
            let l1 = Length::new::<kilometer>(V::one());
            let l2 = Length::new::<meter>(V::one());
            let m1 = Mass::new::<kilogram>(V::one());

            assert_eq!(1000.0, l1.get(meter));
            assert_eq!(V::one(), l2.get(meter));
            assert_eq!(V::one(), l1.get(kilometer));
            assert_eq!(0.001, l2.get(kilometer));
            assert_eq!(V::one(), m1.get(kilogram));
        }

        #[test]
        fn conversion() {
            assert_eq!(1000.0, <kilometer as Conversion<V>>::conversion());
            assert_eq!(V::one(), <meter as Conversion<V>>::conversion());
            assert_eq!(V::one(), <kilogram as Conversion<V>>::conversion());
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
                format!("{:?} m^1 kg^1", 1.23),
                format!("{:?}", Length::new::<meter>(1.23) * Mass::new::<kilogram>(V::one())));
        }
    }

    mod float {
        storage_types! {
            types: Float;

            use tests::*;

            const EPSILON: V = 0.00001;

            Q!(tests, V);

            #[test]
            fn floor() {
                let l1 = Length::new::<kilometer>(3.9999);
                let l2 = Length::new::<kilometer>(3.0001);
                let l3 = Length::new::<meter>(3.9999);
                let l4 = Length::new::<meter>(3.0001);
                let m1 = Mass::new::<kilogram>(3.9999);
                let m2 = Mass::new::<kilogram>(3.0001);

                assert_eq!(3.0, l1.floor(kilometer).get(kilometer));
                assert_eq!(3999.0, l1.floor(meter).get(meter));
                assert_eq!(3.0, l2.floor(kilometer).get(kilometer));
                assert_eq!(3000.0, l2.floor(meter).get(meter));
                assert_eq!(0.0, l3.floor(kilometer).get(kilometer));
                assert_eq!(3.0, l3.floor(meter).get(meter));
                assert_eq!(0.0, l4.floor(kilometer).get(kilometer));
                assert_eq!(3.0, l4.floor(meter).get(meter));
                assert_eq!(3.0, m1.floor(kilogram).get(kilogram));
                assert_eq!(3.0, m2.floor(kilogram).get(kilogram));
            }

            #[test]
            fn ceil() {
                let l1 = Length::new::<kilometer>(3.9999);
                let l2 = Length::new::<kilometer>(3.0001);
                let l3 = Length::new::<meter>(3.9999);
                let l4 = Length::new::<meter>(3.0001);
                let m1 = Mass::new::<kilogram>(3.9999);
                let m2 = Mass::new::<kilogram>(3.0001);

                assert_eq!(4.0, l1.ceil(kilometer).get(kilometer));
                assert_eq!(4000.0, l1.ceil(meter).get(meter));
                assert_eq!(4.0, l2.ceil(kilometer).get(kilometer));
                assert_eq!(3001.0, l2.ceil(meter).get(meter));
                assert_eq!(V::one(), l3.ceil(kilometer).get(kilometer));
                assert_eq!(4.0, l3.ceil(meter).get(meter));
                assert_eq!(1.0, l4.ceil(kilometer).get(kilometer));
                assert_eq!(4.0, l4.ceil(meter).get(meter));
                assert_eq!(4.0, m1.ceil(kilogram).get(kilogram));
                assert_eq!(4.0, m2.ceil(kilogram).get(kilogram));
            }

            #[test]
            fn round() {
                let l1 = Length::new::<kilometer>(3.3);
                let l2 = Length::new::<kilometer>(3.5);
                let l3 = Length::new::<meter>(3.3);
                let l4 = Length::new::<meter>(3.5);
                let m1 = Mass::new::<kilogram>(3.3);
                let m2 = Mass::new::<kilogram>(3.5);

                assert_eq!(3.0, l1.round(kilometer).get(kilometer));
                assert_eq!(3300.0, l1.round(meter).get(meter));
                assert_eq!(4.0, l2.round(kilometer).get(kilometer));
                assert_eq!(3500.0, l2.round(meter).get(meter));
                assert_eq!(0.0, l3.round(kilometer).get(kilometer));
                assert_eq!(3.0, l3.round(meter).get(meter));
                assert_eq!(0.0, l4.round(kilometer).get(kilometer));
                assert_eq!(4.0, l4.round(meter).get(meter));
                assert_eq!(3.0, m1.round(kilogram).get(kilogram));
                assert_eq!(4.0, m2.round(kilogram).get(kilogram));
            }

            #[test]
            fn trunc() {
                let l1 = Length::new::<kilometer>(3.3);
                let l2 = Length::new::<kilometer>(3.5);
                let l3 = Length::new::<meter>(3.3);
                let l4 = Length::new::<meter>(3.5);
                let m1 = Mass::new::<kilogram>(3.3);
                let m2 = Mass::new::<kilogram>(3.5);

                assert_eq!(3.0, l1.trunc(kilometer).get(kilometer));
                assert_eq!(3300.0, l1.trunc(meter).get(meter));
                assert_eq!(3.0, l2.trunc(kilometer).get(kilometer));
                assert_eq!(3500.0, l2.trunc(meter).get(meter));
                assert_eq!(0.0, l3.trunc(kilometer).get(kilometer));
                assert_eq!(3.0, l3.trunc(meter).get(meter));
                assert_eq!(0.0, l4.trunc(kilometer).get(kilometer));
                assert_eq!(3.0, l4.trunc(meter).get(meter));
                assert_eq!(3.0, m1.trunc(kilogram).get(kilogram));
                assert_eq!(3.0, m2.trunc(kilogram).get(kilogram));
            }

            #[test]
            fn fract() {
                let l1 = Length::new::<kilometer>(3.3);
                let l2 = Length::new::<kilometer>(3.5);
                let l3 = Length::new::<meter>(3.3);
                let l4 = Length::new::<meter>(3.5);
                let m1 = Mass::new::<kilogram>(3.3);
                let m2 = Mass::new::<kilogram>(3.5);

                assert_ulps_eq!(0.3, l1.fract(kilometer).get(kilometer), epsilon = EPSILON);
                assert_ulps_eq!(0.0, l1.fract(meter).get(meter), epsilon = EPSILON);
                assert_ulps_eq!(0.5, l2.fract(kilometer).get(kilometer), epsilon = EPSILON);
                assert_ulps_eq!(0.0, l2.fract(meter).get(meter), epsilon = EPSILON);
                assert_ulps_eq!(0.0033, l3.fract(kilometer).get(kilometer), epsilon = EPSILON);
                assert_ulps_eq!(0.3, l3.fract(meter).get(meter), epsilon = EPSILON);
                assert_ulps_eq!(0.0035, l4.fract(kilometer).get(kilometer), epsilon = EPSILON);
                assert_ulps_eq!(0.5, l4.fract(meter).get(meter), epsilon = EPSILON);
                assert_ulps_eq!(0.3, m1.fract(kilogram).get(kilogram), epsilon = EPSILON);
                assert_ulps_eq!(0.5, m2.fract(kilogram).get(kilogram), epsilon = EPSILON);
            }
        }
    }
}

mod system_macro {
    storage_types! {
        types: Float;

        use tests::*;

        const EPSILON: V = 0.00001;

        Q!(tests, V);

        quickcheck! {
            #[allow(trivial_casts)]
            fn from_base(v: V) -> bool
            {
                type MeterKilogram = Units<V, length = meter, mass = kilogram>;
                type KilometerKilogram = Units<V, length = kilometer, mass = kilogram>;

                // meter -> meter.
                ulps_eq!(v,
                        ::tests::from_base::<length::Dimension, MeterKilogram, V, meter>(&v),
                        epsilon = EPSILON)
                    // kilometer -> kilometer.
                    && ulps_eq!(v,
                        ::tests::from_base::<length::Dimension, KilometerKilogram, V, kilometer>(&v),
                        epsilon = EPSILON)
                    // meter -> kilometer.
                    && ulps_eq!(v / <kilometer as ::Conversion<V>>::conversion().value(),
                        ::tests::from_base::<length::Dimension, MeterKilogram, V, kilometer>(&v),
                        epsilon = EPSILON)
                    // kilometer -> meter.
                    && ulps_eq!(v * <kilometer as ::Conversion<V>>::conversion().value(),
                        ::tests::from_base::<length::Dimension, KilometerKilogram, V, meter>(&v),
                        epsilon = EPSILON)
            }

            #[allow(trivial_casts)]
            fn to_base(v: V) -> bool
            {
                type MeterKilogram = Units<V, length = meter, mass = kilogram>;
                type KilometerKilogram = Units<V, length = kilometer, mass = kilogram>;

                // meter -> meter.
                ulps_eq!(v,
                        ::tests::to_base::<length::Dimension, MeterKilogram, V, meter>(&v),
                        epsilon = EPSILON)
                    // kilometer -> kilometer.
                    && ulps_eq!(v,
                        ::tests::to_base::<length::Dimension, KilometerKilogram, V, kilometer>(&v),
                        epsilon = EPSILON)
                    // kilometer -> meter.
                    && ulps_eq!(v * <kilometer as ::Conversion<V>>::conversion().value(),
                        ::tests::to_base::<length::Dimension, MeterKilogram, V, kilometer>(&v),
                        epsilon = EPSILON)
                    // meter -> kilometer.
                    && ulps_eq!(v / <kilometer as ::Conversion<V>>::conversion().value(),
                        ::tests::to_base::<length::Dimension, KilometerKilogram, V, meter>(&v),
                        epsilon = EPSILON)
            }

            #[allow(trivial_casts)]
            fn change_base(v: V) -> bool
            {
                type MeterKilogram = Units<V, length = meter, mass = kilogram>;
                type KilometerKilogram = Units<V, length = kilometer, mass = kilogram>;

                // meter -> meter.
                ulps_eq!(v,
                        ::tests::change_base::<length::Dimension, MeterKilogram, MeterKilogram, V>(&v),
                        epsilon = EPSILON)
                    // kilometer -> kilometer.
                    && ulps_eq!(v,
                        ::tests::change_base::<length::Dimension, KilometerKilogram, KilometerKilogram, V>(&v),
                        epsilon = EPSILON)
                    // kilometer -> meter.
                    && ulps_eq!(v * <kilometer as ::Conversion<V>>::conversion().value(),
                        ::tests::change_base::<length::Dimension, MeterKilogram, KilometerKilogram, V>(&v),
                        epsilon = EPSILON)
                    // meter -> kilometer.
                    && ulps_eq!(v / <kilometer as ::Conversion<V>>::conversion().value(),
                        ::tests::change_base::<length::Dimension, KilometerKilogram, MeterKilogram, V>(&v),
                        epsilon = EPSILON)
            }

            #[allow(trivial_casts)]
            fn add(l: V, r: V) -> bool {
                (l + r) == (Length::new::<meter>(l) + Length::new::<meter>(r)).get(meter)
            }

            #[allow(trivial_casts)]
            fn sub(l: V, r: V) -> bool {
                (l - r) == (Length::new::<meter>(l) - Length::new::<meter>(r)).get(meter)
            }

            #[allow(trivial_casts)]
            fn mul_quantity(l: V, r: V) -> bool {
                // TODO Use `.get(square_meter)`
                (l * r) == (Length::new::<meter>(l) * Length::new::<meter>(r)).value
            }

            #[allow(trivial_casts)]
            fn mul_v(l: V, r: V) -> bool {
                (l * r) == (Length::new::<meter>(l) * r).get(meter)
                    //&& (l * r) == (l * Length::new::<meter>(r)).get(meter)
            }

            #[allow(trivial_casts)]
            fn div_quantity(l: V, r: V) -> bool {
                // TODO Use `.get(?)`
                (l / r) == (Length::new::<meter>(l) / Length::new::<meter>(r)).value
            }

            #[allow(trivial_casts)]
            fn div_v(l: V, r: V) -> bool {
                // TODO Use `get(meter^-1)`
                (l / r) == (Length::new::<meter>(l) / r).get(meter)
                    //&& (l / r) == (l / Length::new::<meter>(r)).value
            }

            #[allow(trivial_casts)]
            fn rem(l: V, r: V) -> bool {
                (l % r) == (Length::new::<meter>(l) % Length::new::<meter>(r)).get(meter)
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
                fn is_nan(v: V) -> bool {
                    v.is_nan() == Length::new::<meter>(v).is_nan()
                }

                #[allow(trivial_casts)]
                fn is_infinite(v: V) -> bool {
                    v.is_infinite() == Length::new::<meter>(v).is_infinite()
                }

                #[allow(trivial_casts)]
                fn is_finite(v: V) -> bool {
                    v.is_finite() == Length::new::<meter>(v).is_finite()
                }

                #[allow(trivial_casts)]
                fn is_normal(v: V) -> bool {
                    v.is_normal() == Length::new::<meter>(v).is_normal()
                }

                #[allow(trivial_casts)]
                fn classify(v: V) -> bool {
                    v.classify() == Length::new::<meter>(v).classify()
                }

                #[allow(trivial_casts)]
                fn cbrt(v: V) -> bool {
                    let l: Quantity<Q<P1, Z0>, U<V>, V> = Quantity::<Q<P3, Z0>, U<V>, V> {
                        dimension: ::stdlib::marker::PhantomData,
                        units: ::stdlib::marker::PhantomData,
                        value: v,
                    }.cbrt();

                    v.cbrt() == l.value
                }

                #[allow(trivial_casts)]
                fn is_sign_positive(v: V) -> bool {
                    v.is_sign_positive() == Length::new::<meter>(v).is_sign_positive()
                }

                #[allow(trivial_casts)]
                fn is_sign_negative(v: V) -> bool {
                    v.is_sign_negative() == Length::new::<meter>(v).is_sign_negative()
                }

                #[allow(trivial_casts)]
                fn mul_add(s: V, a: V, b: V) -> bool {
                    let r: Quantity<Q<P2, Z0>, U<V>, V> = Length::new::<meter>(s).mul_add(
                        Length::new::<meter>(a),
                        Quantity::<Q<P2, Z0>, U<V>, V> {
                            dimension: ::stdlib::marker::PhantomData,
                            units: ::stdlib::marker::PhantomData,
                            value: b
                        });

                    s.mul_add(a, b) == r.value
                }

                #[allow(trivial_casts)]
                fn recip(v: V) -> bool {
                    let a: Quantity<Q<N1, Z0>, U<V>, V> = Quantity::<Q<P1, Z0>, U<V>, V> {
                        dimension: ::stdlib::marker::PhantomData,
                        units: ::stdlib::marker::PhantomData,
                        value: v,
                    }.recip();

                    v.recip() == a.value
                }

                #[allow(trivial_casts)]
                fn powi(v: V) -> bool {
                    v.powi(3) == Length::new::<meter>(v).powi(P3::new()).value
                }

                #[allow(trivial_casts)]
                fn sqrt(v: V) -> TestResult {
                    if v < 0.0 {
                        return TestResult::discard();
                    }

                    let l: Quantity<Q<P1, Z0>, U<V>, V> = Quantity::<Q<P2, Z0>, U<V>, V> {
                        dimension: ::stdlib::marker::PhantomData,
                        units: ::stdlib::marker::PhantomData,
                        value: v,
                    }.sqrt();

                    TestResult::from_bool(v.sqrt() == l.value)
                }

                #[allow(trivial_casts)]
                fn max(l: V, r: V) -> bool {
                    l.max(r) == Length::new::<meter>(l).max(Length::new::<meter>(r)).get(meter)
                }

                #[allow(trivial_casts)]
                fn min(l: V, r: V) -> bool {
                    l.min(r) == Length::new::<meter>(l).min(Length::new::<meter>(r)).get(meter)
                }
            }
        }
    }

    mod signed {
        storage_types! {
            types: Float;

            use tests::*;

            Q!(tests, V);

            quickcheck! {
                #[allow(trivial_casts)]
                fn abs(v: V) -> bool {
                    v.abs() == Length::new::<meter>(v).abs().get(meter)
                }

                #[allow(trivial_casts)]
                fn signum(v: V) -> bool {
                    v.signum() == Length::new::<meter>(v).signum().get(meter)
                }

                #[allow(trivial_casts)]
                fn neg(l: V) -> bool {
                    -l == -Length::new::<meter>(l).get(meter)
                }
            }
        }
    }

    mod op_assign {
        storage_types! {
            types: Float, PrimInt;

            use tests::*;

            Q!(tests, V);

            quickcheck! {
                #[allow(trivial_casts)]
                fn add_assign(l: V, r: V) -> bool {
                    let mut f = l;
                    let mut v = Length::new::<meter>(l);

                    f += r;
                    v += Length::new::<meter>(r);

                    f == v.get(meter)
                }

                #[allow(trivial_casts)]
                fn sub_assign(l: V, r: V) -> bool {
                    let mut f = l;
                    let mut v = Length::new::<meter>(l);

                    f -= r;
                    v -= Length::new::<meter>(r);

                    f == v.get(meter)
                }

                #[allow(trivial_casts)]
                fn mul_assign(l: V, r: V) -> bool {
                    let mut f = l;
                    let mut v = Length::new::<meter>(l);

                    f *= r;
                    v *= r;

                    f == v.get(meter)
                }

                #[allow(trivial_casts)]
                fn div_assign(l: V, r: V) -> bool {
                    let mut f = l;
                    let mut v = Length::new::<meter>(l);

                    f /= r;
                    v /= r;

                    f == v.get(meter)
                }

                #[allow(trivial_casts)]
                fn rem_assign(l: V, r: V) -> bool {
                    let mut f = l;
                    let mut v = Length::new::<meter>(l);

                    f %= r;
                    v %= Length::new::<meter>(r);

                    f == v.get(meter)
                }
            }
        }
    }
}

mod quantities_macro {
    mod fractional {
        storage_types! {
            types: Float;

            use tests::*;

            const EPSILON: V = 0.00001;

            mod f { Q!(tests, super::V); }
            mod k { Q!(tests, super::V, (kilometer, kilogram)); }

            #[test]
            fn new() {
                let l1 = k::Length::new::<kilometer>(V::one());
                let l2 = k::Length::new::<meter>(V::one());
                let m1 = k::Mass::new::<kilogram>(V::one());

                assert_eq!(V::one(), l1.value);
                assert_eq!(1.0_E-3, l2.value);
                assert_eq!(V::one(), m1.value);
            }

            #[test]
            fn get() {
                let l1 = k::Length::new::<kilometer>(V::one());
                let l2 = k::Length::new::<meter>(V::one());
                let m1 = k::Mass::new::<kilogram>(V::one());

                assert_ulps_eq!(1000.0, l1.get(meter));
                assert_ulps_eq!(V::one(), l2.get(meter));
                assert_ulps_eq!(V::one(), l1.get(kilometer));
                assert_ulps_eq!(0.001, l2.get(kilometer));
                assert_ulps_eq!(V::one(), m1.get(kilogram));
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn add(l: V, r: V) -> bool {
                    ulps_eq!(l + r,
                        (k::Length::new::<meter>(l) + f::Length::new::<meter>(r)).get(meter),
                        epsilon = EPSILON)
                }

                #[allow(trivial_casts)]
                fn sub(l: V, r: V) -> bool {
                    ulps_eq!((l - r),
                        (k::Length::new::<meter>(l) - f::Length::new::<meter>(r)).get(meter),
                        epsilon = EPSILON)
                }

                #[allow(trivial_casts)]
                fn mul_quantity(l: V, r: V) -> bool {
                    // TODO Use `.get(square_meter)`
                    ulps_eq!(l * r,
                            (f::Length::new::<meter>(l) * k::Length::new::<meter>(r)).value,
                            epsilon = EPSILON)
                        && ulps_eq!(l * r,
                            (f::Length::new::<meter>(l) * k::Mass::new::<kilogram>(r)).value,
                            epsilon = EPSILON)
                        && ulps_eq!(l * r,
                            (k::Length::new::<kilometer>(l) * f::Mass::new::<kilogram>(r)).value,
                            epsilon = EPSILON)
                }

                #[allow(trivial_casts)]
                fn div_quantity(l: V, r: V) -> bool {
                    // TODO Use `.get(?)`
                    ulps_eq!(l / r,
                        (k::Length::new::<meter>(l) / f::Length::new::<meter>(r)).value,
                        epsilon = EPSILON)
                }

                #[allow(trivial_casts)]
                fn rem(l: V, r: V) -> bool {
                    ulps_eq!(l % r,
                        (k::Length::new::<meter>(l) % f::Length::new::<meter>(r)).get(meter),
                        epsilon = EPSILON)
                }
            }
        }
    }

    mod float {
        storage_types! {
            types: Float;

            use tests::*;

            const EPSILON: V = 0.00001;

            mod f { Q!(tests, super::V); }
            mod k { Q!(tests, super::V, (kilometer, kilogram)); }

            quickcheck! {
                #[allow(trivial_casts)]
                fn add_assign(l: V, r: V) -> bool {
                    let mut f = l;
                    let mut v = k::Length::new::<meter>(l);

                    f += r;
                    v += f::Length::new::<meter>(r);

                    ulps_eq!(f, v.get(meter), epsilon = EPSILON)
                }

                #[allow(trivial_casts)]
                fn sub_assign(l: V, r: V) -> bool {
                    let mut f = l;
                    let mut v = k::Length::new::<meter>(l);

                    f -= r;
                    v -= f::Length::new::<meter>(r);

                    ulps_eq!(f, v.get(meter), epsilon = EPSILON)
                }

                #[allow(trivial_casts)]
                fn rem_assign(l: V, r: V) -> bool {
                    let mut f = l;
                    let mut v = k::Length::new::<meter>(l);

                    f %= r;
                    v %= f::Length::new::<meter>(r);

                    ulps_eq!(f, v.get(meter), epsilon = EPSILON)
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

                assert_eq!(3.0, l1.floor(kilometer).get(kilometer));
                assert_eq!(3999.0, l1.floor(meter).get(meter));
                assert_eq!(3.0, l2.floor(kilometer).get(kilometer));
                assert_eq!(3000.0, l2.floor(meter).get(meter));
                assert_eq!(0.0, l3.floor(kilometer).get(kilometer));
                assert_eq!(3.0, l3.floor(meter).get(meter));
                assert_eq!(0.0, l4.floor(kilometer).get(kilometer));
                assert_eq!(3.0, l4.floor(meter).get(meter));
                assert_eq!(3.0, m1.floor(kilogram).get(kilogram));
                assert_eq!(3.0, m2.floor(kilogram).get(kilogram));
            }

            #[test]
            fn ceil() {
                let l1 = k::Length::new::<kilometer>(3.9999);
                let l2 = k::Length::new::<kilometer>(3.0001);
                let l3 = k::Length::new::<meter>(3.9999);
                let l4 = k::Length::new::<meter>(3.0001);
                let m1 = k::Mass::new::<kilogram>(3.9999);
                let m2 = k::Mass::new::<kilogram>(3.0001);

                assert_eq!(4.0, l1.ceil(kilometer).get(kilometer));
                assert_eq!(4000.0, l1.ceil(meter).get(meter));
                assert_eq!(4.0, l2.ceil(kilometer).get(kilometer));
                assert_eq!(3001.0, l2.ceil(meter).get(meter));
                assert_eq!(V::one(), l3.ceil(kilometer).get(kilometer));
                assert_eq!(4.0, l3.ceil(meter).get(meter));
                assert_eq!(V::one(), l4.ceil(kilometer).get(kilometer));
                assert_eq!(4.0, l4.ceil(meter).get(meter));
                assert_eq!(4.0, m1.ceil(kilogram).get(kilogram));
                assert_eq!(4.0, m2.ceil(kilogram).get(kilogram));
            }

            #[test]
            fn round() {
                let l1 = k::Length::new::<kilometer>(3.3);
                let l2 = k::Length::new::<kilometer>(3.5);
                let l3 = k::Length::new::<meter>(3.3);
                let l4 = k::Length::new::<meter>(3.5);
                let m1 = k::Mass::new::<kilogram>(3.3);
                let m2 = k::Mass::new::<kilogram>(3.5);

                assert_eq!(3.0, l1.round(kilometer).get(kilometer));
                assert_eq!(3300.0, l1.round(meter).get(meter));
                assert_eq!(4.0, l2.round(kilometer).get(kilometer));
                assert_eq!(3500.0, l2.round(meter).get(meter));
                assert_eq!(0.0, l3.round(kilometer).get(kilometer));
                assert_eq!(3.0, l3.round(meter).get(meter));
                assert_eq!(0.0, l4.round(kilometer).get(kilometer));
                assert_eq!(4.0, l4.round(meter).get(meter));
                assert_eq!(3.0, m1.round(kilogram).get(kilogram));
                assert_eq!(4.0, m2.round(kilogram).get(kilogram));
            }

            #[test]
            fn trunc() {
                let l1 = k::Length::new::<kilometer>(3.3);
                let l2 = k::Length::new::<kilometer>(3.5);
                let l3 = k::Length::new::<meter>(3.3);
                let l4 = k::Length::new::<meter>(3.5);
                let m1 = k::Mass::new::<kilogram>(3.3);
                let m2 = k::Mass::new::<kilogram>(3.5);

                assert_eq!(3.0, l1.trunc(kilometer).get(kilometer));
                assert_eq!(3300.0, l1.trunc(meter).get(meter));
                assert_eq!(3.0, l2.trunc(kilometer).get(kilometer));
                assert_eq!(3500.0, l2.trunc(meter).get(meter));
                assert_eq!(0.0, l3.trunc(kilometer).get(kilometer));
                assert_eq!(3.0, l3.trunc(meter).get(meter));
                assert_eq!(0.0, l4.trunc(kilometer).get(kilometer));
                assert_eq!(3.0, l4.trunc(meter).get(meter));
                assert_eq!(3.0, m1.trunc(kilogram).get(kilogram));
                assert_eq!(3.0, m2.trunc(kilogram).get(kilogram));
            }

            #[test]
            fn fract() {
                let l1 = k::Length::new::<kilometer>(3.3);
                let l2 = k::Length::new::<kilometer>(3.5);
                let l3 = k::Length::new::<meter>(3.3);
                let l4 = k::Length::new::<meter>(3.5);
                let m1 = k::Mass::new::<kilogram>(3.3);
                let m2 = k::Mass::new::<kilogram>(3.5);

                assert_ulps_eq!(0.3, l1.fract(kilometer).get(kilometer), epsilon = EPSILON);
                assert_ulps_eq!(0.0, l1.fract(meter).get(meter), epsilon = EPSILON);
                assert_ulps_eq!(0.5, l2.fract(kilometer).get(kilometer), epsilon = EPSILON);
                assert_ulps_eq!(0.0, l2.fract(meter).get(meter), epsilon = EPSILON);
                assert_ulps_eq!(0.0033, l3.fract(kilometer).get(kilometer), epsilon = EPSILON);
                assert_ulps_eq!(0.3, l3.fract(meter).get(meter), epsilon = EPSILON);
                assert_ulps_eq!(0.0035, l4.fract(kilometer).get(kilometer), epsilon = EPSILON);
                assert_ulps_eq!(0.5, l4.fract(meter).get(meter), epsilon = EPSILON);
                assert_ulps_eq!(0.3, m1.fract(kilogram).get(kilogram), epsilon = EPSILON);
                assert_ulps_eq!(0.5, m2.fract(kilogram).get(kilogram), epsilon = EPSILON);
            }
        }
    }
}
