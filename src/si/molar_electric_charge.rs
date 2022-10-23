//! Molar electric charge (base unit coulomb per mole, s · A · mol⁻¹).

quantity! {
    /// Molar electric charge (base unit coulomb per mole, s · A · mol⁻¹).
    quantity: MolarElectricCharge; "molar electric charge";
    /// Dimension of molar electric charge, TIN⁻¹ (base unit coulomb per mole, s · A · mol⁻¹).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        N1,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @coulomb_per_mole: prefix!(none); "C/mol",
            "coulomb per mole", "coulombs per mole";
        @faraday_constant: 96_485.332_123_310_03 ; "F", // CODATA value 96_485.332_12
            "faraday constant", "faraday constants";

    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;

        use crate::si::amount_of_substance as aos;
        use crate::si::quantities::*;
        use crate::si::electric_charge as q;
        use crate::si::molar_electric_charge as mec;
        use crate::si::reciprocal_amount_of_substance as raos;

        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: MolarElectricCharge<V> = ElectricCharge::new::<q::coulomb>(V::one())
                / AmountOfSubstance::new::<aos::mole>(V::one());
        }

        #[test]
        fn check_units() {
            test::<q::coulomb, aos::mole, mec::coulomb_per_mole>();

            fn test<Q: q::Conversion<V>, AOS: aos::Conversion<V>, MEC: mec::Conversion<V>>() {
                Test::assert_approx_eq(&MolarElectricCharge::new::<MEC>(V::one()),
                    &(ElectricCharge::new::<Q>(V::one())
                        / AmountOfSubstance::new::<AOS>(V::one())));
            }
        }

        #[test]
        fn check_units_na() {
            test::<q::elementary_charge, raos::avogadro_constant, mec::faraday_constant>();

            fn test<Q: q::Conversion<V>, RAOS: raos::Conversion<V>, MEC: mec::Conversion<V>>() {
                Test::assert_approx_eq(&MolarElectricCharge::new::<MEC>(V::one()),
                    &(ElectricCharge::new::<Q>(V::one())
                        * ReciprocalAmountOfSubstance::new::<RAOS>(V::one())));
            }
        }
    }
}
