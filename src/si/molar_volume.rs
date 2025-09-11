//! Molar volume (base unit cubic meter per mole, m³ · mol⁻¹).

quantity! {
    /// Molar volume (base unit cubic meter per mole, m³ · mol⁻¹).
    quantity: MolarVolume; "molar volume";
    /// Dimension of molar volume, L³N⁻¹ (base unit cubic meter per mole, m³ · mol⁻¹).
    dimension: ISQ<
        P3,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        N1,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @cubic_meter_per_mole: prefix!(none); "m³/mol", "cubic meter per mole",
            "cubic meters per mole";
        @cubic_decimeter_per_mole: prefix!(deci) * prefix!(deci) * prefix!(deci); "dm³/mol",
            "cubic decimeter per mole", "cubic decimeters per mole";
        @cubic_centimeter_per_mole: prefix!(centi) * prefix!(centi) * prefix!(centi); "cm³/mol",
            "cubic centimeter per mole", "cubic centimeters per mole";

        @cubic_meter_per_particle: 6.022_140_76_E23; "m³/particle", "cubic meter per particle",
            "cubic meters per particle";
        @cubic_micrometer_per_particle:
            prefix!(micro) * prefix!(micro) * prefix!(micro) * 6.022_140_76_E23; "µm³/particle",
            "cubic micrometer per particle", "cubic micrometers per particle";
        @cubic_nanometer_per_particle:
            prefix!(nano) * prefix!(nano) * prefix!(nano) * 6.022_140_76_E23; "nm³/particle",
            "cubic nanometer per particle", "cubic nanometers per particle";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::amount_of_substance as aos;
        use crate::si::molar_volume as mv;
        use crate::si::quantities::*;
        use crate::si::volume as v;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: MolarVolume<V> = Volume::new::<v::cubic_meter>(V::one())
                / AmountOfSubstance::new::<aos::mole>(V::one());
        }

        #[test]
        fn check_units() {
            test::<v::cubic_meter, aos::mole, mv::cubic_meter_per_mole>();
            test::<v::cubic_decimeter, aos::mole, mv::cubic_decimeter_per_mole>();
            test::<v::cubic_centimeter, aos::mole, mv::cubic_centimeter_per_mole>();
            test::<v::cubic_meter, aos::particle, mv::cubic_meter_per_particle>();
            test::<v::cubic_micrometer, aos::particle, mv::cubic_micrometer_per_particle>();
            test::<v::cubic_nanometer, aos::particle, mv::cubic_nanometer_per_particle>();

            fn test<U: v::Conversion<V>, AOS: aos::Conversion<V>, MV: mv::Conversion<V>>() {
                Test::assert_approx_eq(&MolarVolume::new::<MV>(V::one()),
                    &(Volume::new::<U>(V::one()) / AmountOfSubstance::new::<AOS>(V::one())));
            }
        }
    }
}
