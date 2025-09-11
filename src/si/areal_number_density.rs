//! Areal number density (base unit 1 per square meter, m⁻²).

quantity! {
    /// Areal number density (base unit 1 per square meter, m⁻²).
    quantity: ArealNumberDensity; "areal number density";
    /// Dimension of areal number density, L⁻² (base unit 1 per square meter, m⁻²).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ConstituentConcentrationKind;
    units {
        @per_square_kilometer: prefix!(none) / prefix!(kilo) / prefix!(kilo); "km⁻²",
            "per square kilometer", "per square kilometer";
        @per_square_meter: prefix!(none); "m⁻²", "per square meter", "per square meter";
        @per_square_decimeter: prefix!(none) / prefix!(deci) / prefix!(deci); "dm⁻²",
            "per square decimeter", "per square decimeter";
        @per_square_centimeter: prefix!(none) / prefix!(centi) / prefix!(centi); "cm⁻²",
            "per square centimeter", "per square centimeter";
        @per_square_millimeter: prefix!(none) / prefix!(milli) / prefix!(milli); "mm⁻²",
            "per square millimeter", "per square millimeter";
        @per_square_micrometer: prefix!(none) / prefix!(micro) / prefix!(micro); "µm⁻²",
            "per square micrometer", "per square micrometer";

        @per_acre: prefix!(none) / 4.046_873_E3; "ac⁻²", "per acre", "per acre";
        @per_are: prefix!(none) / 1.0_E2; "a⁻²", "per are", "per are";
        @per_barn: prefix!(none) / 1.0_E-28; "b⁻²", "per barn", "per barn";
        @per_circular_mil: prefix!(none) / 5.067_075_E-10; "cmil⁻²", "per circular mil",
            "per circular mil";
        @per_hectare: prefix!(none) / 1.0_E4; "ha⁻²", "per hectare", "per hectare";
        @per_square_foot: prefix!(none) / 9.290_304_E-2; "ft⁻²", "per square foot",
            "per square foot";
        @per_square_inch: prefix!(none) / 6.451_6_E-4; "in⁻²", "per square inch", "per square inch";
        @per_square_mile: prefix!(none) / 2.589_988_110_336_E6; "mi⁻²", "per square mile",
            "per square mile";
        @per_square_yard: prefix!(none) / 8.361_273_6_E-1; "yd⁻²", "per square yard",
            "per square yard";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::area as a;
        use crate::si::areal_number_density as n;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ArealNumberDensity<V> = (V::one()
                / Area::new::<a::square_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<n::per_square_kilometer, a::square_kilometer>();
            test::<n::per_square_meter, a::square_meter>();
            test::<n::per_square_decimeter, a::square_decimeter>();
            test::<n::per_square_centimeter, a::square_centimeter>();
            test::<n::per_square_millimeter, a::square_millimeter>();
            test::<n::per_square_micrometer, a::square_micrometer>();

            test::<n::per_acre, a::acre>();
            test::<n::per_are, a::are>();
            test::<n::per_barn, a::barn>();
            test::<n::per_circular_mil, a::circular_mil>();
            test::<n::per_hectare, a::hectare>();
            test::<n::per_square_foot, a::square_foot>();
            test::<n::per_square_inch, a::square_inch>();
            test::<n::per_square_mile, a::square_mile>();
            test::<n::per_square_yard, a::square_yard>();

            fn test<N: n::Conversion<V>, A: a::Conversion<V>>() {
                Test::assert_approx_eq(&ArealNumberDensity::new::<N>(V::one()),
                    &(V::one() / Area::new::<A>(V::one())).into());
            }
        }
    }
}
