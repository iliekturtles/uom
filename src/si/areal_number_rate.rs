//! Areal number rate (base unit 1 per square meter second, m⁻² · s⁻¹).

quantity! {
    /// Areal number rate (base unit 1 per square meter second, m⁻² · s⁻¹).
    quantity: ArealNumberRate; "areal number rate";
    /// Dimension of areal number rate, L⁻²T⁻¹ (base unit 1 per square meter second, m⁻² · s⁻¹).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ConstituentConcentrationKind;
    units {
        @per_square_meter_second: prefix!(none); "m⁻² · s⁻¹", "per square meter second",
            "per square meter second";
        @per_square_centimeter_second: prefix!(none) / prefix!(centi) / prefix!(centi);
            "cm⁻² · s⁻¹", "per square centimeter second", "per square centimeter second";

        @per_acre_second: prefix!(none) / 4.046_873_E3; "ac⁻¹ · s⁻¹", "per acre second",
            "per acre second";
        @per_are_second: prefix!(none) / 1.0_E2; "a⁻¹ · s⁻¹", "per are second", "per are second";
        @per_barn_second: prefix!(none) / 1.0_E-28; "b⁻¹ · s⁻¹", "per barn second",
            "per barn second";
        @per_circular_mil_second: prefix!(none) / 5.067_075_E-10; "cmil⁻¹ · s⁻¹",
            "per circular mil second", "per circular mil second";
        @per_hectare_second: prefix!(none) / 1.0_E4; "ha⁻¹ · s⁻¹", "per hectare second",
            "per hectare second";
        @per_square_foot_second: prefix!(none) / 9.290_304_E-2; "ft⁻² · s⁻¹",
            "per square foot second", "per square foot second";
        @per_square_inch_second: prefix!(none) / 6.451_6_E-4; "in⁻² · s⁻¹",
            "per square inch second", "per square inch second";
        @per_square_mile_second: prefix!(none) / 2.589_988_110_336_E6; "mi⁻² · s⁻¹",
            "per square mile second", "per square mile second";
        @per_square_yard_second: prefix!(none) / 8.361_273_6_E-1; "yd⁻² · s⁻¹",
            "per square yard second", "per square yard second";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::areal_number_rate as anr;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::area as a;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ArealNumberRate<V> = (V::one()
                / Time::new::<t::second>(V::one())
                / Area::new::<a::square_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<anr::per_square_meter_second, a::square_meter, t::second>();
            test::<anr::per_square_centimeter_second, a::square_centimeter, t::second>();

            test::<anr::per_acre_second, a::acre, t::second>();
            test::<anr::per_are_second, a::are, t::second>();
            test::<anr::per_barn_second, a::barn, t::second>();
            test::<anr::per_circular_mil_second, a::circular_mil, t::second>();
            test::<anr::per_hectare_second, a::hectare, t::second>();
            test::<anr::per_square_foot_second, a::square_foot, t::second>();
            test::<anr::per_square_inch_second, a::square_inch, t::second>();
            test::<anr::per_square_mile_second, a::square_mile, t::second>();
            test::<anr::per_square_yard_second, a::square_yard, t::second>();

            fn test<ANR: anr::Conversion<V>, A: a::Conversion<V>, T: t::Conversion<V>>() {
                Test::assert_approx_eq(&ArealNumberRate::new::<ANR>(V::one()),
                    &(V::one()
                        / Time::new::<T>(V::one())
                        / Area::new::<A>(V::one())).into());
            }
        }
    }
}
