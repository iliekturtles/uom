#[macro_use]
mod length {
    quantity! {
        quantity: TLength; "length";
        dimension: Q<P1, Z0>;
        units {
            @kilometer: prefix!(kilo); "km", "kilometer", "kilometers";
            @meter: prefix!(none); "m", "meter", "meters";
        }
    }
}

#[macro_use]
mod mass {
    quantity! {
        quantity: TMass; "mass";
        dimension: Q<Z0, P1>;
        units {
            @kilogram: prefix!(kilo) / prefix!(kilo); "kg", "kilogram", "kilograms";
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

mod f32 {
    Q!(tests, f32);
}

mod quantity {
    use super::*;
    use super::f32::*;
    use super::length::{kilometer, meter};
    use super::mass::kilogram;

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
}
