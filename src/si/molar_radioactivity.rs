//! Molar radioactivity (base unit becquerel per mole, s⁻¹ · mol⁻¹).

quantity! {
    /// Molar radioactivity (base unit becquerel per mole, s⁻¹ · mol⁻¹).
    quantity: MolarRadioactivity; "molar radioactivity";
    /// Dimension of molar radioactivity, T⁻¹N⁻¹ (base unit becquerel per mole, s⁻¹ · mol⁻¹).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        N1,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ConstituentConcentrationKind;
    units {
        @becquerel_per_mole: prefix!(none); "Bq/mol", "becquerel per mole", "becquerels per mole";

        @curie_per_mole: 3.7_E10; "Ci/mol", "curie per mole", "curies per mole";

        @disintegrations_per_minute_per_mole: 1.0 / 6.0_E1; "dpm/mol",
            "disintegration per minute per mole", "disintegrations per minute per mole";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::radioactivity as rad;
        use crate::si::molar_radioactivity as mrad;
        use crate::si::quantities::*;
        use crate::si::amount_of_substance as aos;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: MolarRadioactivity<V> = (Radioactivity::new::<rad::becquerel>(V::one())
                / AmountOfSubstance::new::<aos::mole>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<rad::becquerel, aos::mole, mrad::becquerel_per_mole>();
            test::<rad::curie, aos::mole, mrad::curie_per_mole>();
            test::<rad::disintegrations_per_minute, aos::mole,
                mrad::disintegrations_per_minute_per_mole>();

            fn test<RAD: rad::Conversion<V>, AOS: aos::Conversion<V>, SRAD: mrad::Conversion<V>>() {
                Test::assert_approx_eq(&MolarRadioactivity::new::<SRAD>(V::one()),
                    &(Radioactivity::new::<RAD>(V::one())
                        / AmountOfSubstance::new::<AOS>(V::one())).into());
            }
        }
    }
}
