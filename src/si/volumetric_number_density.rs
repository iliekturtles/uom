//! Volumetric number density (base unit 1 per cubic meter, m⁻³).

quantity! {
    /// Volumetric number density (base unit 1 per cubic meter, m⁻³).
    quantity: VolumetricNumberDensity; "volumetric number density";
    /// Dimension of volumetric number density, L⁻³ (base unit 1 per cubic meter, m⁻³).
    dimension: ISQ<
        N3,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ConstituentConcentrationKind;
    units {
        @per_cubic_kilometer: prefix!(none) / prefix!(kilo) / prefix!(kilo) / prefix!(kilo); "km⁻³",
            "per cubic kilometer", "per cubic kilometer";
        @per_cubic_meter: prefix!(none); "m⁻³", "per cubic meter", "per cubic meter";
        @per_cubic_decimeter: prefix!(none) / prefix!(deci) / prefix!(deci) / prefix!(deci); "dm⁻³",
            "per cubic decimeter", "per cubic decimeter";
        @per_cubic_centimeter: prefix!(none) / prefix!(centi) / prefix!(centi) / prefix!(centi);
            "cm⁻³", "per cubic centimeter", "per cubic centimeter";
        @per_cubic_millimeter: prefix!(none) / prefix!(milli) / prefix!(milli) / prefix!(milli);
            "mm⁻³", "per cubic millimeter", "per cubic millimeter";

        @per_cubic_foot: prefix!(none) / 2.831_685_E-2; "ft⁻³", "per cubic foot", "per cubic foot";
        @per_cubic_inch: prefix!(none) / 1.638_706_E-5; "in⁻³", "per cubic inch", "per cubic inch";
        @per_cubic_mile: prefix!(none) / 4.168_182_E9; "mi⁻³", "per cubic mile", "per cubic mile";
        @per_cubic_yard: prefix!(none) / 7.645_549_E-1; "yd⁻³", "per cubic yard", "per cubic yard";
        @per_fluid_ounce: prefix!(none) / 2.957_353_E-5; "per fl oz", "per fluid ounce",
            "per fluid ounce";
        @per_fluid_ounce_imperial: prefix!(none) / 2.841_306_E-5; "per fl oz (UK)",
            "per Imperial fluid ounce", "per Imperial fluid ounce";
        @per_gallon_imperial: prefix!(none) / 4.546_09_E-3; "per gal (UK)", "per Imperial gallon",
            "per Imperial gallon";
        @per_gallon: prefix!(none) / 3.785_412_E-3; "per gal", "per gallon", "per gallon";
        @per_liter: prefix!(none) / prefix!(milli); "L⁻¹", "per liter", "per liter";
        @per_deciliter: prefix!(none) / prefix!(milli) / prefix!(deci); "dL⁻¹", "per deciliter",
            "per deciliter";
        @per_centiliter: prefix!(none) / prefix!(milli) / prefix!(centi); "cL⁻¹",
            "per centiliter", "per centiliter";
        @per_milliliter: prefix!(none) / prefix!(milli) / prefix!(milli); "mL⁻¹",
            "per milliliter", "per milliliter";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::volume as v;
        use crate::si::volumetric_number_density as n;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: VolumetricNumberDensity<V> = (V::one()
                / Volume::new::<v::cubic_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<n::per_cubic_kilometer, v::cubic_kilometer>();
            test::<n::per_cubic_meter, v::cubic_meter>();
            test::<n::per_cubic_decimeter, v::cubic_decimeter>();
            test::<n::per_cubic_centimeter, v::cubic_centimeter>();
            test::<n::per_cubic_millimeter, v::cubic_millimeter>();

            test::<n::per_cubic_foot, v::cubic_foot>();
            test::<n::per_cubic_inch, v::cubic_inch>();
            test::<n::per_cubic_mile, v::cubic_mile>();
            test::<n::per_cubic_yard, v::cubic_yard>();
            test::<n::per_fluid_ounce, v::fluid_ounce>();
            test::<n::per_fluid_ounce_imperial, v::fluid_ounce_imperial>();
            test::<n::per_gallon_imperial, v::gallon_imperial>();
            test::<n::per_gallon, v::gallon>();
            test::<n::per_liter, v::liter>();
            test::<n::per_deciliter, v::deciliter>();
            test::<n::per_centiliter, v::centiliter>();
            test::<n::per_milliliter, v::milliliter>();

            fn test<N: n::Conversion<V>, U: v::Conversion<V>>() {
                Test::assert_approx_eq(&VolumetricNumberDensity::new::<N>(V::one()),
                    &(V::one() / Volume::new::<U>(V::one())).into());
            }
        }
    }
}
