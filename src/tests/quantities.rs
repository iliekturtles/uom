//! Tests for the `$quantities!` macro created by the `system!` macro.

use tests::*;

// Module level constant to verify that creation is possible.
#[allow(dead_code)]
#[allow(trivial_numeric_casts)]
#[cfg(feature = "f32")]
const LENGTH: length::Length<U<f32>, f32> =
    Quantity { dimension: PhantomData, units: PhantomData, value: 1.0 };

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

        Test::assert_eq(&V::from_f64(1000.0).unwrap(), &l1.get::<meter>());
        Test::assert_eq(&V::one(), &l2.get::<meter>());
        Test::assert_eq(&V::one(), &l1.get::<kilometer>());
        Test::assert_eq(&V::from_f64(0.001).unwrap(), &l2.get::<kilometer>());
        Test::assert_eq(&V::one(), &m1.get::<kilogram>());
    }

    #[test]
    fn coefficient() {
        Test::assert_eq(&V::from_f64(1000.0).unwrap(),
            &<kilometer as Conversion<V>>::coefficient().value());
        Test::assert_eq(&V::one(), &<meter as Conversion<V>>::coefficient().value());
        Test::assert_eq(&V::one(), &<kilogram as Conversion<V>>::coefficient().value());
        Test::assert_eq(&V::from_f64(5.0 / 9.0).unwrap(),
            &<degree_fahrenheit as Conversion<V>>::coefficient().value());
    }

    #[test]
    fn constant() {
        Test::assert_eq(&V::zero(),
            &<kilogram as Conversion<V>>::constant(ConstantOp::Add).value());
        Test::assert_eq(&V::from_f64(459.67).unwrap(),
            &<degree_fahrenheit as Conversion<V>>::constant(ConstantOp::Add).value());
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
            let l1 = Length::new::<kilometer>(3.9999);
            let l2 = Length::new::<kilometer>(3.0001);
            let l3 = Length::new::<meter>(3.9999);
            let l4 = Length::new::<meter>(3.0001);
            let m1 = Mass::new::<kilogram>(3.9999);
            let m2 = Mass::new::<kilogram>(3.0001);

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
            let l1 = Length::new::<kilometer>(3.3);
            let l2 = Length::new::<kilometer>(3.5);
            let l3 = Length::new::<meter>(3.3);
            let l4 = Length::new::<meter>(3.5);
            let m1 = Mass::new::<kilogram>(3.3);
            let m2 = Mass::new::<kilogram>(3.5);

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
            let l1 = Length::new::<kilometer>(3.3);
            let l2 = Length::new::<kilometer>(3.5);
            let l3 = Length::new::<meter>(3.3);
            let l4 = Length::new::<meter>(3.5);
            let m1 = Mass::new::<kilogram>(3.3);
            let m2 = Mass::new::<kilogram>(3.5);

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
            let l1 = Length::new::<kilometer>(3.3);
            let l2 = Length::new::<meter>(3.3);
            let m1 = Mass::new::<kilogram>(3.3);

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
    }
}
