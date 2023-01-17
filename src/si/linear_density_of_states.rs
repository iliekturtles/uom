//! Linear density of states (base unit 1 / meter joule, kg⁻¹ · m⁻³ · s²).

quantity! {
    /// Linear density of states (base unit 1 / meter joule, kg⁻¹ · m⁻³ · s²).
    quantity: LinearDensityOfStates; "linear density of states";
    /// Dimension of linear density of states, L⁻³M⁻¹T² (base unit 1 / meter joule,
    /// kg⁻¹ · m⁻³ · s²).
    dimension: ISQ<
        N3,     // length
        N1,     // mass
        P2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @state_per_meter_joule: prefix!(none); "1/(m · J)", "state per meter joule",
            "states per meter joule";
        @state_per_centimeter_joule: prefix!(none) / prefix!(centi); "1/(cm · J)",
            "state per centimeter joule", "states per centimeter joule";
        @state_per_centimeter_electronvolt: prefix!(none) / prefix!(centi) / 1.602_176_634_E-19;
            "1/(cm · eV)", "state per centimeter electronvolt",
            "states per centimeter electronvolt";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::linear_density_of_states as ldos;
        use crate::si::energy as e;
        use crate::si::quantities::*;
        use crate::si::length as l;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: LinearDensityOfStates<V> = (V::one()
                / Energy::new::<e::joule>(V::one())
                / Length::new::<l::meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<l::meter, e::joule, ldos::state_per_meter_joule>();
            test::<l::centimeter, e::joule, ldos::state_per_centimeter_joule>();
            test::<l::centimeter, e::electronvolt, ldos::state_per_centimeter_electronvolt>();

            fn test<L: l::Conversion<V>, E: e::Conversion<V>, LDOS: ldos::Conversion<V>>() {
                Test::assert_approx_eq(&LinearDensityOfStates::new::<LDOS>(V::one()),
                    &(V::one()
                        / Energy::new::<E>(V::one())
                        / Length::new::<L>(V::one())).into());
            }
        }
    }
}
