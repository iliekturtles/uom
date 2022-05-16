//! Tests for the `quantity!` macro.

storage_types! {
    use crate::tests::*;

    mod f { Q!(crate::tests, super::V); }
    mod k { Q!(crate::tests, super::V, (kilometer, kilogram, kelvin)); }

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

        Test::assert_eq(&V::from_f64(1000.0).unwrap(), &l1.get::<meter>());
        Test::assert_eq(&V::one(), &l2.get::<meter>());
        Test::assert_eq(&V::one(), &l1.get::<kilometer>());
        Test::assert_eq(&V::from_f64(0.001).unwrap(), &l2.get::<kilometer>());
        Test::assert_eq(&V::one(), &m1.get::<kilogram>());
    }

    #[test]
    fn from_str() {
        let l1 = k::Length::new::<meter>(V::one());
        let l2 = k::Length::new::<kilometer>(V::one());
        let m1 = k::Mass::new::<kilogram>(V::from_f64(1.0E3).unwrap());
        let m2 = k::Mass::new::<kilogram>(V::from_f64(1.0E-3).unwrap());

        Test::assert_eq(&"1 m".parse::<k::Length>().unwrap(), &l1);
        Test::assert_eq(&"1 meter".parse::<k::Length>().unwrap(), &l1);
        Test::assert_eq(&"1 meters".parse::<k::Length>().unwrap(), &l1);
        Test::assert_eq(&"1.0 km".parse::<k::Length>().unwrap(), &l2);
        Test::assert_eq(&"1000 kg".parse::<k::Mass>().unwrap(), &m1);
        Test::assert_eq(&"1.0E-3 kg".parse::<k::Mass>().unwrap(), &m2);

        assert_eq!(&"1m".parse::<k::Length>(), &Err(ParseQuantityError::NoSeparator));
        assert_eq!(&"10k m".parse::<k::Length>(), &Err(ParseQuantityError::ValueParseError));
        assert_eq!(&"1,000 km".parse::<k::Length>(), &Err(ParseQuantityError::ValueParseError));
        assert_eq!(&"10 s".parse::<k::Length>(), &Err(ParseQuantityError::UnknownUnit));
        assert_eq!(&"10 kg //10,000 g".parse::<k::Length>(), &Err(ParseQuantityError::UnknownUnit));
    }

    #[cfg(feature = "autoconvert")]
    quickcheck! {
        #[allow(trivial_casts)]
        fn add(l: A<V>, r: A<V>) -> TestResult {
            let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
            let f = (&*l + &*r) / &km;
            let i = (&*l / &km) + (&*r / &km);

            if !Test::approx_eq(&i, &f) {
                return TestResult::discard();
            }

            TestResult::from_bool(
                Test::approx_eq(&k::Length::new::<meter>(&*l + &*r),
                    &(k::Length::new::<meter>((*l).clone())
                        + f::Length::new::<meter>((*r).clone()))))
        }

        #[allow(trivial_casts)]
        fn sub(l: A<V>, r: A<V>) -> TestResult {
            let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
            let f = (&*l - &*r) / &km;
            let i = (&*l / &km) - (&*r / &km);

            if !Test::approx_eq(&i, &f) {
                return TestResult::discard();
            }

            TestResult::from_bool(
                Test::approx_eq(&k::Length::new::<meter>(&*l - &*r),
                    &(k::Length::new::<meter>((*l).clone())
                        - f::Length::new::<meter>((*r).clone()))))
        }

        #[allow(trivial_casts)]
        fn mul_quantity(l: A<V>, r: A<V>) -> TestResult {
            let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();

            TestResult::from_bool(
                Test::approx_eq(&/*Area::new::<square_meter>*/(&*l * &*r),
                        &(f::Length::new::<meter>((*l).clone())
                            * k::Length::new::<meter>((*r).clone())).value)
                    && Test::approx_eq(
                        &/*Area::new::<square_kilometer>*/(&(&*l / &km) * &(&*r / &km)),
                        &(k::Length::new::<meter>((*l).clone())
                            * f::Length::new::<meter>((*r).clone())).value)
                    && Test::approx_eq(&/*Length-mass*/(&*l * &*r),
                        &(f::Length::new::<meter>((*l).clone())
                            * k::Mass::new::<kilogram>((*r).clone())).value)
                    && Test::approx_eq(&/*Length-mass*/(&*l * &*r),
                        &(k::Length::new::<kilometer>((*l).clone())
                            * f::Mass::new::<kilogram>((*r).clone())).value))
        }

        #[allow(trivial_casts)]
        fn div_quantity(l: A<V>, r: A<V>) -> TestResult {
            if *r == V::zero() {
                return TestResult::discard();
            }

            let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();

            TestResult::from_bool(
                Test::approx_eq(&/*Ratio::new::<ratio>*/(&*l / &*r),
                        &(f::Length::new::<meter>((*l).clone())
                            / k::Length::new::<meter>((*r).clone())).value)
                    && Test::approx_eq(&/*Ratio::new::<ratio>*/(&*l / &*r),
                        &(k::Length::new::<meter>((*l).clone())
                            / f::Length::new::<meter>((*r).clone())).value)
                    && Test::approx_eq(&/*Length/mass*/(&*l / &*r),
                        &(f::Length::new::<meter>((*l).clone())
                            / k::Mass::new::<kilogram>((*r).clone())).value)
                    && Test::approx_eq(&/*Length/mass*/(&*l / &km / &*r),
                        &(k::Length::new::<meter>((*l).clone())
                            / f::Mass::new::<kilogram>((*r).clone())).value))
        }

        #[allow(trivial_casts)]
        fn rem(l: A<V>, r: A<V>) -> TestResult {
            if *r == V::zero() {
                return TestResult::discard();
            }

            let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
            let f = (&*l % &*r) / &km;
            let i = (&*l / &km) % (&*r / &km);

            if !Test::approx_eq(&i, &f) {
                return TestResult::discard();
            }

            TestResult::from_bool(
                Test::approx_eq(&k::Length::new::<meter>(&*l % &*r),
                    &(k::Length::new::<meter>((*l).clone())
                        % f::Length::new::<meter>((*r).clone()))))
        }

        #[allow(trivial_casts)]
        fn eq(l: A<V>, r: A<V>) -> bool {
            let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
            let a = *l == ((*r).clone() / &km) * &km;
            let b = (*l).clone() / &km == (*r).clone() / &km;
            let x = f::Length::new::<meter>((*l).clone())
                == k::Length::new::<meter>((*r).clone());
            let y = k::Length::new::<meter>((*l).clone())
                == f::Length::new::<meter>((*r).clone());

            a == x && b == y
        }

        #[allow(trivial_casts)]
        fn ne(l: A<V>, r: A<V>) -> bool {
            let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
            let a = *l != ((*r).clone() / &km) * &km;
            let b = (*l).clone() / &km != (*r).clone() / &km;
            let x = f::Length::new::<meter>((*l).clone())
                != k::Length::new::<meter>((*r).clone());
            let y = k::Length::new::<meter>((*l).clone())
                != f::Length::new::<meter>((*r).clone());

            a == x && b == y
        }
    }
}

#[cfg(feature = "std")]
mod fmt {
    macro_rules! test_format {
        ($v:expr, $specifier:expr) => {
            test_format!($v, $specifier, ["", "+", "05"]);
        };
        ($v:expr, $specifier:expr, [$($option:expr),+]) => {
            let m = f::Mass::new::<kilogram>((*$v).clone());
            let result = true;

            $(let result = result
                && format!(
                        concat!("{:", $option, $specifier, "} kilogram{}"),
                        *$v, if $v.is_one() { "" } else { "s" })
                    == format!(
                        concat!("{:", $option, $specifier, "}"),
                        m.clone().into_format_args(kilogram, DisplayStyle::Description));)+

            return result;
        };
    }

    storage_types! {
        use crate::tests::*;

        mod f { Q!(crate::tests, super::V); }

        quickcheck! {
            #[allow(trivial_casts)]
            fn display(v: A<V>) -> bool {
                test_format!(v, "");
            }

            #[allow(trivial_casts)]
            fn debug(v: A<V>) -> bool {
                test_format!(v, "?");
            }
        }

        #[test]
        fn round_trip() {
            let l = f::Length::new::<meter>(V::one());
            let s1 = &format!("{}",
                l.clone().into_format_args(kilometer, DisplayStyle::Abbreviation));
            assert_eq!(s1.parse::<f::Length>(), Ok(l.clone()));
            let s2 = &format!("{}", l.clone().into_format_args(meter, DisplayStyle::Abbreviation));
            assert_eq!(s2.parse::<f::Length>(), Ok(l.clone()));
        }
    }

    mod float {
        storage_types! {
            types: Float;

            use crate::tests::*;

            mod f { Q!(crate::tests, super::V); }

            quickcheck! {
                #[allow(trivial_casts)]
                fn lower_exp(v: A<V>) -> bool {
                    test_format!(v, "e");
                }

                #[allow(trivial_casts)]
                fn upper_exp(v: A<V>) -> bool {
                    test_format!(v, "E");
                }
            }

        }
    }

    mod fixed {
        storage_types! {
            types: PrimInt, BigInt, BigUint;

            use crate::tests::*;

            mod f { Q!(crate::tests, super::V); }

            quickcheck! {
                #[allow(trivial_casts)]
                fn binary(v: A<V>) -> bool {
                    test_format!(v, "b");
                }

                #[test]
                fn lower_hex(v: A<V>) -> bool {
                    test_format!(v, "x");
                }

                #[test]
                fn octal(v: A<V>) -> bool {
                    test_format!(v, "o");
                }

                #[test]
                fn upper_hex(v: A<V>) -> bool {
                    test_format!(v, "X");
                }
            }
        }
    }
}

#[cfg(feature = "autoconvert")]
mod non_big {
    storage_types! {
        types: Float, PrimInt, Rational, Rational32, Rational64;

        use crate::tests::*;

        mod f { Q!(crate::tests, super::V); }
        mod k { Q!(crate::tests, super::V, (kilometer, kilogram, kelvin)); }

        quickcheck! {
            #[allow(trivial_casts)]
            fn add_assign(l: A<V>, r: A<V>) -> TestResult {
                let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
                let mut f = *l;
                let mut i = *l / &km;
                let mut v = k::Length::new::<meter>(*l);

                f += *r;
                i += *r / &km;
                v += f::Length::new::<meter>(*r);

                if !Test::approx_eq(&i, &(f / &km)) {
                    return TestResult::discard();
                }

                TestResult::from_bool(Test::approx_eq(&k::Length::new::<meter>(f), &v))
            }

            #[allow(trivial_casts)]
            fn sub_assign(l: A<V>, r: A<V>) -> TestResult {
                let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
                let mut f = *l;
                let mut i = *l / &km;
                let mut v = k::Length::new::<meter>(*l);

                f -= *r;
                i -= *r / &km;
                v -= f::Length::new::<meter>(*r);

                if !Test::approx_eq(&i, &(f / &km)) {
                    return TestResult::discard();
                }

                TestResult::from_bool(Test::approx_eq(&k::Length::new::<meter>(f), &v))
            }

            #[allow(trivial_casts)]
            fn rem_assign(l: A<V>, r: A<V>) -> TestResult {
                if *r == V::zero() {
                    return TestResult::discard();
                }

                let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
                let mut f = *l;
                let mut i = *l / &km;
                let mut v = k::Length::new::<meter>(*l);

                f %= *r;
                i %= *r / &km;
                v %= f::Length::new::<meter>(*r);

                if !Test::approx_eq(&i, &(f / km)) {
                    return TestResult::discard();
                }

                TestResult::from_bool(Test::approx_eq(&k::Length::new::<meter>(f), &v))
            }
        }
    }
}

mod float {
    storage_types! {
        types: Float;

        use crate::tests::*;

        mod f { Q!(crate::tests, super::V); }
        mod k { Q!(crate::tests, super::V, (kilometer, kilogram, kelvin)); }

        #[test]
        fn floor() {
            let l1 = k::Length::new::<kilometer>(3.9999);
            let l2 = k::Length::new::<kilometer>(3.0001);
            let l3 = k::Length::new::<meter>(3.9999);
            let l4 = k::Length::new::<meter>(3.0001);
            let m1 = k::Mass::new::<kilogram>(3.9999);
            let m2 = k::Mass::new::<kilogram>(3.0001);

            Test::assert_eq(&3.0, &l1.floor::<kilometer>().get::<kilometer>());
            Test::assert_eq(&3999.0, &l1.floor::<meter>().get::<meter>());
            Test::assert_eq(&3.0, &l2.floor::<kilometer>().get::<kilometer>());
            Test::assert_eq(&3000.0, &l2.floor::<meter>().get::<meter>());
            Test::assert_eq(&0.0, &l3.floor::<kilometer>().get::<kilometer>());
            Test::assert_eq(&3.0, &l3.floor::<meter>().get::<meter>());
            Test::assert_eq(&0.0, &l4.floor::<kilometer>().get::<kilometer>());
            Test::assert_eq(&3.0, &l4.floor::<meter>().get::<meter>());
            Test::assert_eq(&3.0, &m1.floor::<kilogram>().get::<kilogram>());
            Test::assert_eq(&3.0, &m2.floor::<kilogram>().get::<kilogram>());
        }

        #[test]
        fn ceil() {
            let l1 = k::Length::new::<kilometer>(3.9999);
            let l2 = k::Length::new::<kilometer>(3.0001);
            let l3 = k::Length::new::<meter>(3.9999);
            let l4 = k::Length::new::<meter>(3.0001);
            let m1 = k::Mass::new::<kilogram>(3.9999);
            let m2 = k::Mass::new::<kilogram>(3.0001);

            Test::assert_eq(&4.0, &l1.ceil::<kilometer>().get::<kilometer>());
            Test::assert_eq(&4000.0, &l1.ceil::<meter>().get::<meter>());
            Test::assert_eq(&4.0, &l2.ceil::<kilometer>().get::<kilometer>());
            Test::assert_eq(&3001.0, &l2.ceil::<meter>().get::<meter>());
            Test::assert_eq(&1.0, &l3.ceil::<kilometer>().get::<kilometer>());
            Test::assert_eq(&4.0, &l3.ceil::<meter>().get::<meter>());
            Test::assert_eq(&1.0, &l4.ceil::<kilometer>().get::<kilometer>());
            Test::assert_eq(&4.0, &l4.ceil::<meter>().get::<meter>());
            Test::assert_eq(&4.0, &m1.ceil::<kilogram>().get::<kilogram>());
            Test::assert_eq(&4.0, &m2.ceil::<kilogram>().get::<kilogram>());
        }

        #[test]
        fn round() {
            let l1 = k::Length::new::<kilometer>(3.3);
            let l2 = k::Length::new::<kilometer>(3.5);
            let l3 = k::Length::new::<meter>(3.3);
            let l4 = k::Length::new::<meter>(3.5);
            let m1 = k::Mass::new::<kilogram>(3.3);
            let m2 = k::Mass::new::<kilogram>(3.5);

            Test::assert_eq(&3.0, &l1.round::<kilometer>().get::<kilometer>());
            Test::assert_eq(&3300.0, &l1.round::<meter>().get::<meter>());
            Test::assert_eq(&4.0, &l2.round::<kilometer>().get::<kilometer>());
            Test::assert_eq(&3500.0, &l2.round::<meter>().get::<meter>());
            Test::assert_eq(&0.0, &l3.round::<kilometer>().get::<kilometer>());
            Test::assert_eq(&3.0, &l3.round::<meter>().get::<meter>());
            Test::assert_eq(&0.0, &l4.round::<kilometer>().get::<kilometer>());
            Test::assert_eq(&4.0, &l4.round::<meter>().get::<meter>());
            Test::assert_eq(&3.0, &m1.round::<kilogram>().get::<kilogram>());
            Test::assert_eq(&4.0, &m2.round::<kilogram>().get::<kilogram>());
        }

        #[test]
        fn trunc() {
            let l1 = k::Length::new::<kilometer>(3.3);
            let l2 = k::Length::new::<kilometer>(3.5);
            let l3 = k::Length::new::<meter>(3.3);
            let l4 = k::Length::new::<meter>(3.5);
            let m1 = k::Mass::new::<kilogram>(3.3);
            let m2 = k::Mass::new::<kilogram>(3.5);

            Test::assert_eq(&3.0, &l1.trunc::<kilometer>().get::<kilometer>());
            Test::assert_eq(&3300.0, &l1.trunc::<meter>().get::<meter>());
            Test::assert_eq(&3.0, &l2.trunc::<kilometer>().get::<kilometer>());
            Test::assert_eq(&3500.0, &l2.trunc::<meter>().get::<meter>());
            Test::assert_eq(&0.0, &l3.trunc::<kilometer>().get::<kilometer>());
            Test::assert_eq(&3.0, &l3.trunc::<meter>().get::<meter>());
            Test::assert_eq(&0.0, &l4.trunc::<kilometer>().get::<kilometer>());
            Test::assert_eq(&3.0, &l4.trunc::<meter>().get::<meter>());
            Test::assert_eq(&3.0, &m1.trunc::<kilogram>().get::<kilogram>());
            Test::assert_eq(&3.0, &m2.trunc::<kilogram>().get::<kilogram>());
        }

        #[test]
        fn fract() {
            let l1 = k::Length::new::<kilometer>(3.3);
            let l2 = k::Length::new::<meter>(3.3);
            let m1 = k::Mass::new::<kilogram>(3.3);

            Test::assert_eq(&3.3.fract(), &l1.fract::<kilometer>().get::<kilometer>());
            Test::assert_eq(&(3.3.fract() * 1000.0), &l1.fract::<kilometer>().get::<meter>());
            Test::assert_eq(&((3.3 * 1000.0).fract() / 1000.0),
                &l1.fract::<meter>().get::<kilometer>());
            Test::assert_eq(&(3.3 * 1000.0).fract(), &l1.fract::<meter>().get::<meter>());

            Test::assert_eq(&(3.3 / 1000.0).fract(), &l2.fract::<kilometer>().get::<kilometer>());
            Test::assert_eq(&((3.3 / 1000.0).fract() * 1000.0),
                &l2.fract::<kilometer>().get::<meter>());
            Test::assert_eq(&(3.3.fract() / 1000.0), &l2.fract::<meter>().get::<kilometer>());
            Test::assert_eq(&3.3.fract(), &l2.fract::<meter>().get::<meter>());

            Test::assert_eq(&3.3.fract(), &m1.fract::<kilogram>().get::<kilogram>());
        }

        #[cfg(feature = "std")]
        quickcheck! {
            #[allow(trivial_casts)]
            fn hypot_same(l: V, r: V) -> bool {
                Test::eq(&l.hypot(r),
                    &f::Length::new::<meter>(l).hypot(f::Length::new::<meter>(r)).get::<meter>())
            }
        }

        #[cfg(all(feature = "std", feature = "autoconvert"))]
        quickcheck! {
            #[allow(trivial_casts)]
            fn hypot_mixed(l: V, r: V) -> bool {
                let fk = Test::approx_eq(&l.hypot(r),
                    &f::Length::new::<meter>(l).hypot(k::Length::new::<meter>(r)).get::<meter>());
                let kf = Test::approx_eq(&l.hypot(r),
                    &k::Length::new::<meter>(l).hypot(f::Length::new::<meter>(r)).get::<meter>());

                fk && kf
            }
        }
    }
}

#[cfg(feature = "autoconvert")]
mod non_complex {
    storage_types! {
        // Everything BUT complex
        types: PrimInt, Ratio, Float, BigInt, BigUint;

        use crate::tests::*;

        mod f { Q!(crate::tests, super::V); }
        mod k { Q!(crate::tests, super::V, (kilometer, kilogram, kelvin)); }

        quickcheck! {
            #[allow(trivial_casts)]
            fn partial_cmp(l: A<V>, r: A<V>) -> bool {
                let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
                let a = (*l).partial_cmp(&(((*r).clone() / &km) * &km));
                let b = ((*l).clone() / &km).partial_cmp(&((*r).clone() / &km));
                let x = f::Length::new::<meter>((*l).clone()).partial_cmp(
                    &k::Length::new::<meter>((*r).clone()));
                let y = k::Length::new::<meter>((*l).clone()).partial_cmp(
                    &f::Length::new::<meter>((*r).clone()));

                a == x && b == y
            }

            #[allow(trivial_casts)]
            fn lt(l: A<V>, r: A<V>) -> bool {
                let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
                let a = (*l).lt(&(((*r).clone() / &km) * &km));
                let b = ((*l).clone() / &km).lt(&((*r).clone() / &km));
                let x = f::Length::new::<meter>((*l).clone()).lt(
                    &k::Length::new::<meter>((*r).clone()));
                let y = k::Length::new::<meter>((*l).clone()).lt(
                    &f::Length::new::<meter>((*r).clone()));

                a == x && b == y
            }

            #[allow(trivial_casts)]
            fn le(l: A<V>, r: A<V>) -> bool {
                let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
                let a = (*l).le(&(((*r).clone() / &km) * &km));
                let b = ((*l).clone() / &km).le(&((*r).clone() / &km));
                let x = f::Length::new::<meter>((*l).clone()).le(
                    &k::Length::new::<meter>((*r).clone()));
                let y = k::Length::new::<meter>((*l).clone()).le(
                    &f::Length::new::<meter>((*r).clone()));

                a == x && b == y
            }

            #[allow(trivial_casts)]
            fn gt(l: A<V>, r: A<V>) -> bool {
                let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
                let a = (*l).gt(&(((*r).clone() / &km) * &km));
                let b = ((*l).clone() / &km).gt(&((*r).clone() / &km));
                let x = f::Length::new::<meter>((*l).clone()).gt(
                    &k::Length::new::<meter>((*r).clone()));
                let y = k::Length::new::<meter>((*l).clone()).gt(
                    &f::Length::new::<meter>((*r).clone()));

                a == x && b == y
            }

            #[allow(trivial_casts)]
            fn ge(l: A<V>, r: A<V>) -> bool {
                let km: V = <kilometer as crate::Conversion<V>>::coefficient().value();
                let a = (*l).ge(&(((*r).clone() / &km) * &km));
                let b = ((*l).clone() / &km).ge(&((*r).clone() / &km));
                let x = f::Length::new::<meter>((*l).clone()).ge(
                    &k::Length::new::<meter>((*r).clone()));
                let y = k::Length::new::<meter>((*l).clone()).ge(
                    &f::Length::new::<meter>((*r).clone()));

                a == x && b == y
            }
        }
    }
}
