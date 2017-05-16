#[cfg(feature = "f64")]
type F = f64;
#[cfg(not(feature = "f64"))]
type F = f32;

pub const EPSILON: F = 0.00001;

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

mod f {
    Q!(tests, super::F);
}

mod km {
    Q!(tests, super::F, (kilometer, kilogram));
}

mod quantity_macro {
    use super::*;
    use super::f::*;
    use super::length::{kilometer, meter};
    use super::mass::kilogram;

    // Module level constant to verify that creation is possible.
    #[allow(dead_code)]
    const LENGTH: Quantity<Q<::typenum::P1, ::typenum::Z0>, U<F>, F> = Quantity {
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
        assert_eq!(format!("{:?} m^-1", 1.0), format!("{:?}", 1.0 / TLength::new::<meter>(1.0)));
        assert_eq!(format!("{:.2?} m^1", 1.0), format!("{:.2?}", TLength::new::<meter>(1.0)));
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
    use typenum::{N1, P1, P2, Z0};

    quickcheck! {
        #[cfg(feature = "std")]
        #[allow(trivial_casts)]
        fn sqrt(v: F) -> TestResult {
            if v < 0.0 {
                return TestResult::discard();
            }

            let l: Quantity<Q<P1, Z0>, U<F>, F> = Quantity::<Q<P2, Z0>, U<F>, F> {
                dimension: ::stdlib::marker::PhantomData,
                units: ::stdlib::marker::PhantomData,
                value: v,
            }.sqrt();

            TestResult::from_bool(v.sqrt() == l.value)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn add(l: F, r: F) -> bool {
            (l + r) == (TLength::new::<meter>(l) + TLength::new::<meter>(r)).get(meter)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn add_assign(l: F, r: F) -> bool {
            let mut f = l;
            let mut v = TLength::new::<meter>(l);

            f += r;
            v += TLength::new::<meter>(r);

            f == v.get(meter)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn sub(l: F, r: F) -> bool {
            (l - r) == (TLength::new::<meter>(l) - TLength::new::<meter>(r)).get(meter)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn sub_assign(l: F, r: F) -> bool {
            let mut f = l;
            let mut v = TLength::new::<meter>(l);

            f -= r;
            v -= TLength::new::<meter>(r);

            f == v.get(meter)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn mul_quantity(l: F, r: F) -> bool {
            // TODO Use `.get(square_meter)`
            (l * r) == (TLength::new::<meter>(l) * TLength::new::<meter>(r)).value
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn mul_float(l: F, r: F) -> bool {
            (l * r) == (TLength::new::<meter>(l) * r).get(meter)
                && (l * r) == (l * TLength::new::<meter>(r)).get(meter)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn mul_assign(l: F, r: F) -> bool {
            let mut f = l;
            let mut v = TLength::new::<meter>(l);

            f *= r;
            v *= r;

            f == v.get(meter)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn div_quantity(l: F, r: F) -> bool {
            // TODO Use `.get(?)`
            (l / r) == (TLength::new::<meter>(l) / TLength::new::<meter>(r)).value
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn div_float(l: F, r: F) -> bool {
            // TODO Use `get(meter^-1)`
            (l / r) == (TLength::new::<meter>(l) / r).get(meter)
                && (l / r) == (l / TLength::new::<meter>(r)).value
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn div_assign(l: F, r: F) -> bool {
            let mut f = l;
            let mut v = TLength::new::<meter>(l);

            f /= r;
            v /= r;

            f == v.get(meter)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn neg(l: F) -> bool {
            -l == -TLength::new::<meter>(l).get(meter)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn rem(l: F, r: F) -> bool {
            (l % r) == (TLength::new::<meter>(l) % TLength::new::<meter>(r)).get(meter)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn rem_assign(l: F, r: F) -> bool {
            let mut f = l;
            let mut v = TLength::new::<meter>(l);

            f %= r;
            v %= TLength::new::<meter>(r);

            f == v.get(meter)
        }
    }

    #[test]
    fn conversion() {
        type U1 = BaseUnits<meter, kilogram, F>;
        type U2 = BaseUnits<kilometer, kilogram, F>;

        assert_eq!(1.0, <U1 as Units<Q<Z0, Z0>, F>>::conversion());
        assert_eq!(1.0, <U1 as Units<Q<Z0, P1>, F>>::conversion());
        assert_eq!(1.0, <U1 as Units<Q<P1, Z0>, F>>::conversion());
        assert_eq!(1.0, <U1 as Units<Q<P1, P1>, F>>::conversion());
        assert_eq!(1.0, <U1 as Units<Q<Z0, N1>, F>>::conversion());
        assert_eq!(1.0, <U1 as Units<Q<N1, Z0>, F>>::conversion());
        assert_eq!(1.0, <U2 as Units<Q<Z0, Z0>, F>>::conversion());
        assert_eq!(1.0, <U2 as Units<Q<Z0, P1>, F>>::conversion());
        assert_eq!(1.0_E3, <U2 as Units<Q<P1, Z0>, F>>::conversion());
        assert_eq!(1.0_E3, <U2 as Units<Q<P1, P1>, F>>::conversion());
        assert_eq!(1.0, <U2 as Units<Q<Z0, N1>, F>>::conversion());
        assert_eq!(1.0_E-3, <U2 as Units<Q<N1, Z0>, F>>::conversion());
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

    quickcheck! {
        #[allow(trivial_casts)]
        fn add(l: F, r: F) -> bool {
            ulps_eq!(l + r,
                (k::TLength::new::<meter>(l) + f::TLength::new::<meter>(r)).get(meter),
                epsilon = EPSILON)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn add_assign(l: F, r: F) -> bool {
            let mut f = l;
            let mut v = k::TLength::new::<meter>(l);

            f += r;
            v += f::TLength::new::<meter>(r);

            ulps_eq!(f, v.get(meter), epsilon = EPSILON)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn sub(l: F, r: F) -> bool {
            ulps_eq!((l - r),
                (k::TLength::new::<meter>(l) - f::TLength::new::<meter>(r)).get(meter),
                epsilon = EPSILON)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn sub_assign(l: F, r: F) -> bool {
            let mut f = l;
            let mut v = k::TLength::new::<meter>(l);

            f -= r;
            v -= f::TLength::new::<meter>(r);

            ulps_eq!(f, v.get(meter), epsilon = EPSILON)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn mul_quantity(l: F, r: F) -> bool {
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
        fn div_quantity(l: F, r: F) -> bool {
            // TODO Use `.get(?)`
            ulps_eq!(l / r,
                (k::TLength::new::<meter>(l) / f::TLength::new::<meter>(r)).value,
                epsilon = EPSILON)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn rem(l: F, r: F) -> bool {
            ulps_eq!(l % r,
                (k::TLength::new::<meter>(l) % f::TLength::new::<meter>(r)).get(meter),
                epsilon = EPSILON)
        }
    }

    quickcheck! {
        #[allow(trivial_casts)]
        fn rem_assign(l: F, r: F) -> bool {
            let mut f = l;
            let mut v = k::TLength::new::<meter>(l);

            f %= r;
            v %= f::TLength::new::<meter>(r);

            ulps_eq!(f, v.get(meter), epsilon = EPSILON)
        }
    }
}
