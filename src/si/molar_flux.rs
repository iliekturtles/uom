//! Molar flux (base unit mole per square meter second, m⁻² · s⁻¹ · mol).

quantity! {
    /// Molar flux (base unit mole per square meter second, m⁻² · s⁻¹ · mol).
    quantity: MolarFlux; "molar flux";
    /// Dimension of molar flux, L⁻²T⁻¹N (base unit mole per square meter second, m⁻² · s⁻¹ · mol).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        P1,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @mole_per_square_meter_second: prefix!(none); "mol/(m² · s)",
            "mole per square meter second", "moles per square meter second";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::amount_of_substance as aos;
        use crate::si::molar_flux as mf;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::area as area;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: MolarFlux<V> = AmountOfSubstance::new::<aos::mole>(V::one())
                / Time::new::<t::second>(V::one())
                / Area::new::<area::square_meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<aos::mole, t::second, area::square_meter, mf::mole_per_square_meter_second>();

            fn test<N: aos::Conversion<V>, T: t::Conversion<V>, A: area::Conversion<V>, R: mf::Conversion<V>>() {
                Test::assert_approx_eq(&MolarFlux::new::<R>(V::one()),
                    &(AmountOfSubstance::new::<N>(V::one())
                        / Time::new::<T>(V::one())
                        / Area::new::<A>(V::one())));
            }
        }
    }
}
