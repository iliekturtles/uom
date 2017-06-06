#[macro_use]
mod length {
    quantity! {
        quantity: TLength; "length";
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
        quantity: TMass; "mass";
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
        TLength,
        TMass,
    }
}

macro_rules! test {
    ($V:ident) => {
        use $crate::stdlib::$V::*;
        use super::*;

        pub const EPSILON: $V = 0.00001;

        mod f {
            Q!(tests, $V);
        }

        mod km {
            Q!(tests, $V, (kilometer, kilogram));
        }

        mod quantity_macro {
            use super::*;
            use super::f::*;
            use super::length::{kilometer, meter};
            use super::mass::kilogram;

            // Module level constant to verify that creation is possible.
            #[allow(dead_code)]
            const LENGTH: Quantity<Q<::typenum::P1, ::typenum::Z0>, U<$V>, $V> = Quantity {
                dimension: ::stdlib::marker::PhantomData,
                units: ::stdlib::marker::PhantomData,
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

            #[test]
            fn struct_literal() {
                use ::stdlib::marker::PhantomData;

                let l = TLength { dimension: PhantomData, units: PhantomData, value: 1.0, };
                let m = TMass { dimension: PhantomData, units: PhantomData, value: 1.0, };

                assert_eq!(1.0, l.value);
                assert_eq!(1.0, m.value);
            }

            #[test]
            fn new() {
                let l1 = TLength::new::<kilometer>(1.0);
                let l2 = TLength::new::<meter>(1.0);
                let m1 = TMass::new::<kilogram>(1.0);

                assert_eq!(1000.0, l1.value);
                assert_eq!(1.0, l2.value);
                assert_eq!(1.0, m1.value);
            }

            #[test]
            fn get() {
                let l1 = TLength::new::<kilometer>(1.0);
                let l2 = TLength::new::<meter>(1.0);
                let m1 = TMass::new::<kilogram>(1.0);

                assert_eq!(1000.0, l1.get(meter));
                assert_eq!(1.0, l2.get(meter));
                assert_eq!(1.0, l1.get(kilometer));
                assert_eq!(0.001, l2.get(kilometer));
                assert_eq!(1.0, m1.get(kilogram));
            }

            #[cfg(feature = "std")]
            #[test]
            fn floor() {
                let l1 = TLength::new::<kilometer>(3.9999);
                let l2 = TLength::new::<kilometer>(3.0001);
                let l3 = TLength::new::<meter>(3.9999);
                let l4 = TLength::new::<meter>(3.0001);
                let m1 = TMass::new::<kilogram>(3.9999);
                let m2 = TMass::new::<kilogram>(3.0001);

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

            #[cfg(feature = "std")]
            #[test]
            fn ceil() {
                let l1 = TLength::new::<kilometer>(3.9999);
                let l2 = TLength::new::<kilometer>(3.0001);
                let l3 = TLength::new::<meter>(3.9999);
                let l4 = TLength::new::<meter>(3.0001);
                let m1 = TMass::new::<kilogram>(3.9999);
                let m2 = TMass::new::<kilogram>(3.0001);

                assert_eq!(4.0, l1.ceil(kilometer).get(kilometer));
                assert_eq!(4000.0, l1.ceil(meter).get(meter));
                assert_eq!(4.0, l2.ceil(kilometer).get(kilometer));
                assert_eq!(3001.0, l2.ceil(meter).get(meter));
                assert_eq!(1.0, l3.ceil(kilometer).get(kilometer));
                assert_eq!(4.0, l3.ceil(meter).get(meter));
                assert_eq!(1.0, l4.ceil(kilometer).get(kilometer));
                assert_eq!(4.0, l4.ceil(meter).get(meter));
                assert_eq!(4.0, m1.ceil(kilogram).get(kilogram));
                assert_eq!(4.0, m2.ceil(kilogram).get(kilogram));
            }

            #[cfg(feature = "std")]
            #[test]
            fn round() {
                let l1 = TLength::new::<kilometer>(3.3);
                let l2 = TLength::new::<kilometer>(3.5);
                let l3 = TLength::new::<meter>(3.3);
                let l4 = TLength::new::<meter>(3.5);
                let m1 = TMass::new::<kilogram>(3.3);
                let m2 = TMass::new::<kilogram>(3.5);

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

            #[cfg(feature = "std")]
            #[test]
            fn trunc() {
                let l1 = TLength::new::<kilometer>(3.3);
                let l2 = TLength::new::<kilometer>(3.5);
                let l3 = TLength::new::<meter>(3.3);
                let l4 = TLength::new::<meter>(3.5);
                let m1 = TMass::new::<kilogram>(3.3);
                let m2 = TMass::new::<kilogram>(3.5);

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

            #[cfg(feature = "std")]
            #[test]
            fn fract() {
                let l1 = TLength::new::<kilometer>(3.3);
                let l2 = TLength::new::<kilometer>(3.5);
                let l3 = TLength::new::<meter>(3.3);
                let l4 = TLength::new::<meter>(3.5);
                let m1 = TMass::new::<kilogram>(3.3);
                let m2 = TMass::new::<kilogram>(3.5);

                ulps_eq!(0.3, l1.fract(kilometer).get(kilometer), epsilon = EPSILON);
                ulps_eq!(0.0, l1.fract(meter).get(meter), epsilon = EPSILON);
                ulps_eq!(0.0, l2.fract(kilometer).get(kilometer), epsilon = EPSILON);
                ulps_eq!(0.5, l2.fract(meter).get(meter), epsilon = EPSILON);
                ulps_eq!(0.0033, l3.fract(kilometer).get(kilometer), epsilon = EPSILON);
                ulps_eq!(0.3, l3.fract(meter).get(meter), epsilon = EPSILON);
                ulps_eq!(0.00035, l4.fract(kilometer).get(kilometer), epsilon = EPSILON);
                ulps_eq!(0.5, l4.fract(meter).get(meter), epsilon = EPSILON);
                ulps_eq!(0.3, m1.fract(kilogram).get(kilogram), epsilon = EPSILON);
                ulps_eq!(0.5, m2.fract(kilogram).get(kilogram), epsilon = EPSILON);
            }

            #[test]
            fn conversion() {
                assert_eq!(1000.0, kilometer::conversion());
                assert_eq!(1.0, meter::conversion());
                assert_eq!(1.0, kilogram::conversion());
            }

            #[cfg(feature = "std")]
            #[test]
            fn debug_fmt() {
                assert_eq!(format!("{:?} m^1", 1.0), format!("{:?}", TLength::new::<meter>(1.0)));
                assert_eq!(
                    format!("{:?} m^-1", 1.0),
                    format!("{:?}", 1.0 / TLength::new::<meter>(1.0)));
                assert_eq!(
                    format!("{:.2?} m^1", 1.0),
                    format!("{:.2?}", TLength::new::<meter>(1.0)));
                assert_eq!(
                    format!("{:?} m^1 kg^1", 1.23),
                    format!("{:?}", TLength::new::<meter>(1.23) * TMass::new::<kilogram>(1.0))
                );
            }
        }

        mod system_macro {
            use super::*;
            use super::f::*;
            use super::length::{kilometer, meter};
            use super::mass::kilogram;
            #[cfg(feature = "std")]
            use quickcheck::TestResult;
            #[allow(unused_imports)]
            use typenum::{N1, P1, P2, P3, Z0};

            fn _assert_static() {
                assert::<Quantity<Q<Z0, Z0>, U<$V>, $V>>();

                fn assert<T: Send + Sync>() {}
            }

            #[test]
            fn fp_categories() {
                assert!(!TLength::new::<meter>(INFINITY).is_finite());
                assert!(!TLength::new::<meter>(NEG_INFINITY).is_finite());
                assert!(TLength::new::<meter>(INFINITY).is_infinite());
                assert!(TLength::new::<meter>(NEG_INFINITY).is_infinite());
                assert!(TLength::new::<meter>(MIN_POSITIVE).is_normal());
                assert!(TLength::new::<meter>(MAX).is_normal());
                assert!(!TLength::new::<meter>(0.0).is_normal());
                assert!(!TLength::new::<meter>(NAN).is_normal());
                assert!(!TLength::new::<meter>(INFINITY).is_normal());
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn is_nan(v: $V) -> bool {
                    v.is_nan() == TLength::new::<meter>(v).is_nan()
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn is_infinite(v: $V) -> bool {
                    v.is_infinite() == TLength::new::<meter>(v).is_infinite()
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn is_finite(v: $V) -> bool {
                    v.is_finite() == TLength::new::<meter>(v).is_finite()
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn is_normal(v: $V) -> bool {
                    v.is_normal() == TLength::new::<meter>(v).is_normal()
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn classify(v: $V) -> bool {
                    v.classify() == TLength::new::<meter>(v).classify()
                }
            }

            quickcheck! {
                #[cfg(feature = "std")]
                #[allow(trivial_casts)]
                fn cbrt(v: $V) -> bool {
                    let l: Quantity<Q<P1, Z0>, U<$V>, $V> = Quantity::<Q<P3, Z0>, U<$V>, $V> {
                        dimension: ::stdlib::marker::PhantomData,
                        units: ::stdlib::marker::PhantomData,
                        value: v,
                    }.cbrt();

                    v.cbrt() == l.value
                }
            }

            quickcheck! {
                #[cfg(feature = "std")]
                #[allow(trivial_casts)]
                fn recip(v: $V) -> bool {
                    let a: Quantity<Q<N1, Z0>, U<$V>, $V> = Quantity::<Q<P1, Z0>, U<$V>, $V> {
                        dimension: ::stdlib::marker::PhantomData,
                        units: ::stdlib::marker::PhantomData,
                        value: v,
                    }.recip();

                    v.recip() == a.value
                }
            }

            quickcheck! {
                #[cfg(feature = "std")]
                #[allow(trivial_casts)]
                fn sqrt(v: $V) -> TestResult {
                    if v < 0.0 {
                        return TestResult::discard();
                    }

                    let l: Quantity<Q<P1, Z0>, U<$V>, $V> = Quantity::<Q<P2, Z0>, U<$V>, $V> {
                        dimension: ::stdlib::marker::PhantomData,
                        units: ::stdlib::marker::PhantomData,
                        value: v,
                    }.sqrt();

                    TestResult::from_bool(v.sqrt() == l.value)
                }
            }

            quickcheck! {
                #[cfg(feature = "std")]
                #[allow(trivial_casts)]
                fn max(l: $V, r: $V) -> bool {
                    l.max(r) == TLength::new::<meter>(l).max(TLength::new::<meter>(r)).get(meter)
                }
            }

            quickcheck! {
                #[cfg(feature = "std")]
                #[allow(trivial_casts)]
                fn min(l: $V, r: $V) -> bool {
                    l.min(r) == TLength::new::<meter>(l).min(TLength::new::<meter>(r)).get(meter)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn add(l: $V, r: $V) -> bool {
                    (l + r) == (TLength::new::<meter>(l) + TLength::new::<meter>(r)).get(meter)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn add_assign(l: $V, r: $V) -> bool {
                    let mut f = l;
                    let mut v = TLength::new::<meter>(l);

                    f += r;
                    v += TLength::new::<meter>(r);

                    f == v.get(meter)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn sub(l: $V, r: $V) -> bool {
                    (l - r) == (TLength::new::<meter>(l) - TLength::new::<meter>(r)).get(meter)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn sub_assign(l: $V, r: $V) -> bool {
                    let mut f = l;
                    let mut v = TLength::new::<meter>(l);

                    f -= r;
                    v -= TLength::new::<meter>(r);

                    f == v.get(meter)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn mul_quantity(l: $V, r: $V) -> bool {
                    // TODO Use `.get(square_meter)`
                    (l * r) == (TLength::new::<meter>(l) * TLength::new::<meter>(r)).value
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn mul_float(l: $V, r: $V) -> bool {
                    (l * r) == (TLength::new::<meter>(l) * r).get(meter)
                        && (l * r) == (l * TLength::new::<meter>(r)).get(meter)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn mul_assign(l: $V, r: $V) -> bool {
                    let mut f = l;
                    let mut v = TLength::new::<meter>(l);

                    f *= r;
                    v *= r;

                    f == v.get(meter)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn div_quantity(l: $V, r: $V) -> bool {
                    // TODO Use `.get(?)`
                    (l / r) == (TLength::new::<meter>(l) / TLength::new::<meter>(r)).value
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn div_float(l: $V, r: $V) -> bool {
                    // TODO Use `get(meter^-1)`
                    (l / r) == (TLength::new::<meter>(l) / r).get(meter)
                        && (l / r) == (l / TLength::new::<meter>(r)).value
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn div_assign(l: $V, r: $V) -> bool {
                    let mut f = l;
                    let mut v = TLength::new::<meter>(l);

                    f /= r;
                    v /= r;

                    f == v.get(meter)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn neg(l: $V) -> bool {
                    -l == -TLength::new::<meter>(l).get(meter)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn rem(l: $V, r: $V) -> bool {
                    (l % r) == (TLength::new::<meter>(l) % TLength::new::<meter>(r)).get(meter)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn rem_assign(l: $V, r: $V) -> bool {
                    let mut f = l;
                    let mut v = TLength::new::<meter>(l);

                    f %= r;
                    v %= TLength::new::<meter>(r);

                    f == v.get(meter)
                }
            }

            #[test]
            fn conversion() {
                type U1 = BaseUnits<meter, kilogram, $V>;
                type U2 = BaseUnits<kilometer, kilogram, $V>;

                assert_eq!(1.0, <U1 as Units<Q<Z0, Z0>, $V>>::conversion());
                assert_eq!(1.0, <U1 as Units<Q<Z0, P1>, $V>>::conversion());
                assert_eq!(1.0, <U1 as Units<Q<P1, Z0>, $V>>::conversion());
                assert_eq!(1.0, <U1 as Units<Q<P1, P1>, $V>>::conversion());
                assert_eq!(1.0, <U1 as Units<Q<Z0, N1>, $V>>::conversion());
                assert_eq!(1.0, <U1 as Units<Q<N1, Z0>, $V>>::conversion());
                assert_eq!(1.0, <U2 as Units<Q<Z0, Z0>, $V>>::conversion());
                assert_eq!(1.0, <U2 as Units<Q<Z0, P1>, $V>>::conversion());
                assert_eq!(1.0_E3, <U2 as Units<Q<P1, Z0>, $V>>::conversion());
                assert_eq!(1.0_E3, <U2 as Units<Q<P1, P1>, $V>>::conversion());
                assert_eq!(1.0, <U2 as Units<Q<Z0, N1>, $V>>::conversion());
                assert_eq!(1.0_E-3, <U2 as Units<Q<N1, Z0>, $V>>::conversion());
            }
        }

        mod quantities_macro {
            use super::*;
            use super::f;
            use super::km as k;
            use super::length::{kilometer, meter};
            use super::mass::kilogram;

            #[test]
            fn new() {
                let l1 = k::TLength::new::<kilometer>(1.0);
                let l2 = k::TLength::new::<meter>(1.0);
                let m1 = k::TMass::new::<kilogram>(1.0);

                assert_eq!(1.0, l1.value);
                assert_eq!(1.0_E-3, l2.value);
                assert_eq!(1.0, m1.value);
            }

            #[test]
            fn get() {
                let l1 = k::TLength::new::<kilometer>(1.0);
                let l2 = k::TLength::new::<meter>(1.0);
                let m1 = k::TMass::new::<kilogram>(1.0);

                ulps_eq!(1000.0, l1.get(meter));
                ulps_eq!(1.0, l2.get(meter));
                ulps_eq!(1.0, l1.get(kilometer));
                ulps_eq!(0.001, l2.get(kilometer));
                ulps_eq!(1.0, m1.get(kilogram));
            }

            #[cfg(feature = "std")]
            #[test]
            fn floor() {
                let l1 = k::TLength::new::<kilometer>(3.9999);
                let l2 = k::TLength::new::<kilometer>(3.0001);
                let l3 = k::TLength::new::<meter>(3.9999);
                let l4 = k::TLength::new::<meter>(3.0001);
                let m1 = k::TMass::new::<kilogram>(3.9999);
                let m2 = k::TMass::new::<kilogram>(3.0001);

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

            #[cfg(feature = "std")]
            #[test]
            fn ceil() {
                let l1 = k::TLength::new::<kilometer>(3.9999);
                let l2 = k::TLength::new::<kilometer>(3.0001);
                let l3 = k::TLength::new::<meter>(3.9999);
                let l4 = k::TLength::new::<meter>(3.0001);
                let m1 = k::TMass::new::<kilogram>(3.9999);
                let m2 = k::TMass::new::<kilogram>(3.0001);

                assert_eq!(4.0, l1.ceil(kilometer).get(kilometer));
                assert_eq!(4000.0, l1.ceil(meter).get(meter));
                assert_eq!(4.0, l2.ceil(kilometer).get(kilometer));
                assert_eq!(3001.0, l2.ceil(meter).get(meter));
                assert_eq!(1.0, l3.ceil(kilometer).get(kilometer));
                assert_eq!(4.0, l3.ceil(meter).get(meter));
                assert_eq!(1.0, l4.ceil(kilometer).get(kilometer));
                assert_eq!(4.0, l4.ceil(meter).get(meter));
                assert_eq!(4.0, m1.ceil(kilogram).get(kilogram));
                assert_eq!(4.0, m2.ceil(kilogram).get(kilogram));
            }

            #[cfg(feature = "std")]
            #[test]
            fn round() {
                let l1 = k::TLength::new::<kilometer>(3.3);
                let l2 = k::TLength::new::<kilometer>(3.5);
                let l3 = k::TLength::new::<meter>(3.3);
                let l4 = k::TLength::new::<meter>(3.5);
                let m1 = k::TMass::new::<kilogram>(3.3);
                let m2 = k::TMass::new::<kilogram>(3.5);

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

            #[cfg(feature = "std")]
            #[test]
            fn trunc() {
                let l1 = k::TLength::new::<kilometer>(3.3);
                let l2 = k::TLength::new::<kilometer>(3.5);
                let l3 = k::TLength::new::<meter>(3.3);
                let l4 = k::TLength::new::<meter>(3.5);
                let m1 = k::TMass::new::<kilogram>(3.3);
                let m2 = k::TMass::new::<kilogram>(3.5);

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

            #[cfg(feature = "std")]
            #[test]
            fn fract() {
                let l1 = k::TLength::new::<kilometer>(3.3);
                let l2 = k::TLength::new::<kilometer>(3.5);
                let l3 = k::TLength::new::<meter>(3.3);
                let l4 = k::TLength::new::<meter>(3.5);
                let m1 = k::TMass::new::<kilogram>(3.3);
                let m2 = k::TMass::new::<kilogram>(3.5);

                ulps_eq!(0.3, l1.fract(kilometer).get(kilometer), epsilon = EPSILON);
                ulps_eq!(0.0, l1.fract(meter).get(meter), epsilon = EPSILON);
                ulps_eq!(0.0, l2.fract(kilometer).get(kilometer), epsilon = EPSILON);
                ulps_eq!(0.5, l2.fract(meter).get(meter), epsilon = EPSILON);
                ulps_eq!(0.0033, l3.fract(kilometer).get(kilometer), epsilon = EPSILON);
                ulps_eq!(0.3, l3.fract(meter).get(meter), epsilon = EPSILON);
                ulps_eq!(0.00035, l4.fract(kilometer).get(kilometer), epsilon = EPSILON);
                ulps_eq!(0.5, l4.fract(meter).get(meter), epsilon = EPSILON);
                ulps_eq!(0.3, m1.fract(kilogram).get(kilogram), epsilon = EPSILON);
                ulps_eq!(0.5, m2.fract(kilogram).get(kilogram), epsilon = EPSILON);
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn add(l: $V, r: $V) -> bool {
                    ulps_eq!(l + r,
                        (k::TLength::new::<meter>(l) + f::TLength::new::<meter>(r)).get(meter),
                        epsilon = EPSILON)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn add_assign(l: $V, r: $V) -> bool {
                    let mut f = l;
                    let mut v = k::TLength::new::<meter>(l);

                    f += r;
                    v += f::TLength::new::<meter>(r);

                    ulps_eq!(f, v.get(meter), epsilon = EPSILON)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn sub(l: $V, r: $V) -> bool {
                    ulps_eq!((l - r),
                        (k::TLength::new::<meter>(l) - f::TLength::new::<meter>(r)).get(meter),
                        epsilon = EPSILON)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn sub_assign(l: $V, r: $V) -> bool {
                    let mut f = l;
                    let mut v = k::TLength::new::<meter>(l);

                    f -= r;
                    v -= f::TLength::new::<meter>(r);

                    ulps_eq!(f, v.get(meter), epsilon = EPSILON)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn mul_quantity(l: $V, r: $V) -> bool {
                    // TODO Use `.get(square_meter)`
                    ulps_eq!(l * r,
                            (f::TLength::new::<meter>(l) * k::TLength::new::<meter>(r)).value,
                            epsilon = EPSILON)
                        && ulps_eq!(l * r,
                            (f::TLength::new::<meter>(l) * k::TMass::new::<kilogram>(r)).value,
                            epsilon = EPSILON)
                        && ulps_eq!(l * r,
                            (k::TLength::new::<kilometer>(l) * f::TMass::new::<kilogram>(r)).value,
                            epsilon = EPSILON)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn div_quantity(l: $V, r: $V) -> bool {
                    // TODO Use `.get(?)`
                    ulps_eq!(l / r,
                        (k::TLength::new::<meter>(l) / f::TLength::new::<meter>(r)).value,
                        epsilon = EPSILON)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn rem(l: $V, r: $V) -> bool {
                    ulps_eq!(l % r,
                        (k::TLength::new::<meter>(l) % f::TLength::new::<meter>(r)).get(meter),
                        epsilon = EPSILON)
                }
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn rem_assign(l: $V, r: $V) -> bool {
                    let mut f = l;
                    let mut v = k::TLength::new::<meter>(l);

                    f %= r;
                    v %= f::TLength::new::<meter>(r);

                    ulps_eq!(f, v.get(meter), epsilon = EPSILON)
                }
            }
        }
    };
}

#[cfg(feature = "f32")]
mod f32 {
    test!(f32);
}

#[cfg(feature = "f64")]
mod f64 {
    test!(f64);
}
