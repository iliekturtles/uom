//! Linear number density (base unit 1 per meter, m⁻¹).

quantity! {
    /// Linear number density (base unit 1 per meter, m⁻¹).
    quantity: LinearNumberDensity; "linear number density";
    /// Dimension of linear number density, L⁻¹ (base 1 unit per meter, m⁻¹).
    dimension: ISQ<
        N1,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ConstituentConcentrationKind;
    units {
        @per_kilometer: prefix!(none) / prefix!(kilo); "km⁻¹", "per kilometer", "per kilometer";
        @per_meter: prefix!(none); "m⁻¹", "per meter", "per meter";
        @per_decimeter: prefix!(none) / prefix!(deci); "dm⁻¹", "per decimeter", "per decimeter";
        @per_centimeter: prefix!(none) / prefix!(centi); "cm⁻¹", "per centimeter", "per centimeter";
        @per_millimeter: prefix!(none) / prefix!(milli); "mm⁻¹", "per millimeter", "per millimeter";

        @per_foot: prefix!(none) / 3.048_E-1; "ft⁻¹", "per foot", "per foot";
        @per_foot_survey: prefix!(none) / 3.048_006_E-1; "ft (U.S. survey)", "foot (U.S. survey)",
            "foot (U.S. survey)";
        @per_inch: prefix!(none) / 2.54_E-2; "in⁻¹", "per inch", "per inch";
        @per_mile: prefix!(none) / 1.609_344_E3; "mi⁻¹", "per mile", "per mile";
        @per_mile_survey: prefix!(none) / 1.609_347_E3; "mi⁻¹ (U.S. survey)",
            "per mile (U.S. survey)", "per mile (U.S. survey)";
        @per_nautical_mile: prefix!(none) / 1.852_E3; "M⁻¹", "per nautical mile",
            "per nautical mile";
        @per_yard: prefix!(none) / 9.144_E-1; "yd⁻¹", "per yard", "per yard";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::length as l;
        use crate::si::linear_number_density as n;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: LinearNumberDensity<V> = (V::one()
                / Length::new::<l::meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<n::per_kilometer, l::kilometer>();
            test::<n::per_meter, l::meter>();
            test::<n::per_decimeter, l::decimeter>();
            test::<n::per_centimeter, l::centimeter>();
            test::<n::per_millimeter, l::millimeter>();

            fn test<N: n::Conversion<V>, L: l::Conversion<V>>() {
                Test::assert_approx_eq(&LinearNumberDensity::new::<N>(V::one()),
                    &(V::one() / Length::new::<L>(V::one())).into());
            }
        }
    }
}
