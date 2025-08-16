//! Areal density of states (base unit 1 / square meter joule, kg⁻¹ · m⁻⁴ · s²).

quantity! {
    /// Areal density of states (base unit 1 / square meter joule, kg⁻¹ · m⁻⁴ · s²).
    quantity: ArealDensityOfStates; "areal density of states";
    /// Dimension of areal density of states, L⁻⁴M⁻¹T² (base unit 1 / square meter joule, kg⁻¹ · m⁻⁴ · s²).
    dimension: ISQ<
        N4,     // length
        N1,     // mass
        P2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ConstituentConcentrationKind;
    units {
        @state_per_square_meter_joule: prefix!(none); "1/(m² · J)", "state per square meter joule",
            "states per square meter joule";
        @state_per_square_centimeter_joule: prefix!(none) / prefix!(centi) / prefix!(centi);
            "1/(cm² · J)", "state per square centimeter joule",
            "states per square centimeter joule";
        @state_per_square_centimeter_electronvolt:
            prefix!(none) / prefix!(centi) / prefix!(centi) / 1.602_176_634_E-19; "1/(cm² · eV)",
            "state per square centimeter electronvolt", "states per square centimeter electronvolt";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::areal_density_of_states as ados;
        use crate::si::energy as e;
        use crate::si::quantities::*;
        use crate::si::area as a;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ArealDensityOfStates<V> = (V::one()
                / Energy::new::<e::joule>(V::one())
                / Area::new::<a::square_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<a::square_meter, e::joule, ados::state_per_square_meter_joule>();
            test::<a::square_centimeter, e::joule, ados::state_per_square_centimeter_joule>();
            test::<a::square_centimeter, e::electronvolt, ados::state_per_square_centimeter_electronvolt>();

            fn test<A: a::Conversion<V>, E: e::Conversion<V>, ADOS: ados::Conversion<V>>() {
                Test::assert_approx_eq(&ArealDensityOfStates::new::<ADOS>(V::one()),
                    &(V::one()
                        / Energy::new::<E>(V::one())
                        / Area::new::<A>(V::one())).into());
            }
        }
    }
}
