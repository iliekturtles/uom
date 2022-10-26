//! Action (base unit joule second, kg ⋅ m² ⋅ s⁻¹).

quantity! {
    /// Action (base unit joule second, kg ⋅ m² ⋅ s⁻¹).
    quantity: Action; "action";
    /// Dimension of action, L²MT⁻¹ (base unit joule second, kg ⋅ m² ⋅ s⁻¹).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @joule_second: prefix!(none); "J · s", "joule second", "joule seconds";

        /// Reduced Planck constant ħ.
        @atomic_unit_of_action: 1.054_571_817_E-34; "ħ", "atomic unit of action",
            "atomic units of action";
        @reduced_planck_constant: 1.054_571_817_E-34; "ħ", "reduced planck constant",
            "reduced planck constants";
        @planck_constant: 6.626_070_15_E-34; "h", "planck constant", "planck constants";
        @erg_second: 1.0_E-7; "erg · s", "erg second", "erg seconds";
        @electronvolt_second: 1.602_176_634_E-19; "eV · s", "electronvolt second",
            "electronvolt seconds";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::energy as e;
        use crate::si::action as act;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: Action<V> = Energy::new::<e::joule>(V::one())
                * Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<e::joule, t::second, act::joule_second>();
            test::<e::erg, t::second, act::erg_second>();
            test::<e::electronvolt, t::second, act::electronvolt_second>();

            fn test<E: e::Conversion<V>, T: t::Conversion<V>, ACT: act::Conversion<V>>() {
                Test::assert_approx_eq(&Action::new::<ACT>(V::one()),
                    &(Energy::new::<E>(V::one()) * Time::new::<T>(V::one())));
            }
        }
    }
}
