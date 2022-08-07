//! Linear number rate (base unit 1 per meter second, m⁻¹ · s⁻¹).

quantity! {
    /// Linear number rate (base unit 1 per meter second, m⁻¹ · s⁻¹).
    quantity: LinearNumberRate; "linear number rate";
    /// Dimension of linear number rate, L⁻¹T⁻¹ (base unit 1 per meter second, m⁻¹ · s⁻¹).
    dimension: ISQ<
        N1,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @per_kilometer_second: prefix!(none) / prefix!(kilo); "km⁻¹ · s⁻¹", "per kilometer second",
            "per kilometer second";
        @per_meter_second: prefix!(none); "m⁻¹ · s⁻¹", "per meter second", "per meter second";
        @per_centimeter_second: prefix!(none) / prefix!(centi); "cm⁻¹ · s⁻¹",
            "per centimeter second", "per centimeter second";
        @per_millimeter_second: prefix!(none) / prefix!(milli); "mm⁻¹ · s⁻¹",
            "per millimeter second", "per millimeter second";

        @per_foot_second: prefix!(none) / 3.048_E-1; "ft⁻¹ · s⁻¹", "per foot second",
            "per foot second";
        @per_foot_survey_second: prefix!(none) / 3.048_006_E-1; "ft⁻¹ (U.S. survey) · s⁻¹",
            "per foot (U.S. survey) second", "per foot (U.S. survey) second";
        @per_inch_second: prefix!(none) / 2.54_E-2; "in⁻¹ · s⁻¹", "per inch second",
            "per inch second";
        @per_mile_second: prefix!(none) / 1.609_344_E3; "mi⁻¹ · s⁻¹", "per mile second",
            "per mile second";
        @per_mile_survey_second: prefix!(none) / 1.609_347_E3; "mi⁻¹ (U.S. survey) · s⁻¹",
            "per mile (U.S. survey) second", "per mile (U.S. survey) second";
        @per_yard_second: prefix!(none) / 9.144_E-1; "yd⁻¹ · s⁻¹", "per yard second",
            "per yard second";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::linear_number_rate as lnr;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::length as l;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: LinearNumberRate<V> = (V::one()
                / Time::new::<t::second>(V::one())
                / Length::new::<l::meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<lnr::per_kilometer_second, l::kilometer, t::second>();
            test::<lnr::per_meter_second, l::meter, t::second>();
            test::<lnr::per_centimeter_second, l::centimeter, t::second>();
            test::<lnr::per_millimeter_second, l::millimeter, t::second>();

            test::<lnr::per_foot_second, l::foot, t::second>();
            test::<lnr::per_foot_survey_second, l::foot_survey, t::second>();
            test::<lnr::per_inch_second, l::inch, t::second>();
            test::<lnr::per_mile_second, l::mile, t::second>();
            test::<lnr::per_mile_survey_second, l::mile_survey, t::second>();
            test::<lnr::per_yard_second, l::yard, t::second>();

            fn test<LNR: lnr::Conversion<V>, L: l::Conversion<V>, T: t::Conversion<V>>() {
                Test::assert_approx_eq(&LinearNumberRate::new::<LNR>(V::one()),
                    &(V::one()
                        / Time::new::<T>(V::one())
                        / Length::new::<L>(V::one())).into());
            }
        }
    }
}
