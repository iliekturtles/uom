//! Reciprocal amount of substance (base unit reciprocal mole, mol⁻¹).

quantity! {
    /// Reciprocal amount of substance (base unit reciprocal mole, mol⁻¹).
    quantity: ReciprocalAmountOfSubstance; "reciprocal amount of substance";
    /// Dimension of reciprocal amount of substance, N⁻¹ (base unit reciprocal mole, mol⁻¹).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        N1,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @reciprocal_mole: prefix!(none); "mol⁻¹",
            "reciprocal mole", "reciprocal moles";
        @avogadro_constant: 6.022_140_76_E23; "Nᴀ",
            "Avogadro constant", "Avogadro constants";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::amount_of_substance as aos;
        use crate::si::quantities::*;
        use crate::si::reciprocal_amount_of_substance as raos;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ReciprocalAmountOfSubstance<V> = V::one()
                / AmountOfSubstance::new::<aos::mole>(V::one());
        }

        #[test]
        fn check_units() {
            test::<aos::mole, raos::reciprocal_mole>();
            test::<aos::particle, raos::avogadro_constant>();

            fn test<AOS: aos::Conversion<V>, RAOS: raos::Conversion<V>>() {
                Test::assert_approx_eq(&ReciprocalAmountOfSubstance::new::<RAOS>(V::one()),
                    &(V::one()
                        / AmountOfSubstance::new::<AOS>(V::one())));
            }
        }
    }
}
