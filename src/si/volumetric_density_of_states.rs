//! Volumetric density of states (base unit 1 / cubic meter joule, kg⁻¹ · m⁻⁵ · s²).

quantity! {
    /// Volumetric density of states (base unit 1 / cubic meter joule, kg⁻¹ · m⁻⁵ · s²).
    quantity: VolumetricDensityOfStates; "volumetric density of states";
    /// Dimension of volumetric density of states, L⁻⁵M⁻¹T² (base unit 1 / cubic meter joule,
    /// kg⁻¹ · m⁻⁵ · s²).
    dimension: ISQ<
        N5,     // length
        N1,     // mass
        P2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @state_per_cubic_meter_joule: prefix!(none); "1/(m³ · J)",
            "state per cubic meter joule", "states per cubic meter joule";
        @state_per_cubic_centimeter_joule:
            prefix!(none) / prefix!(centi) / prefix!(centi) / prefix!(centi); "1/(cm³ · J)",
            "state per cubic centimeter joule", "states per cubic centimeter joule";
        @state_per_cubic_centimeter_electronvolt:
            prefix!(none) / prefix!(centi) / prefix!(centi) / prefix!(centi) / 1.602_176_634_E-19;
            "1/(cm³ · eV)", "state per cubic centimeter electronvolt",
            "states per cubic centimeter electronvolt";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::volumetric_density_of_states as vdos;
        use crate::si::energy as e;
        use crate::si::quantities::*;
        use crate::si::volume as v;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: VolumetricDensityOfStates<V> = (V::one()
                / Energy::new::<e::joule>(V::one())
                / Volume::new::<v::cubic_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<v::cubic_meter, e::joule, vdos::state_per_cubic_meter_joule>();
            test::<v::cubic_centimeter, e::joule, vdos::state_per_cubic_centimeter_joule>();
            test::<v::cubic_centimeter, e::electronvolt,
                vdos::state_per_cubic_centimeter_electronvolt>();

            fn test<U: v::Conversion<V>, E: e::Conversion<V>, VDOS: vdos::Conversion<V>>() {
                Test::assert_approx_eq(&VolumetricDensityOfStates::new::<VDOS>(V::one()),
                    &(V::one()
                        / Energy::new::<E>(V::one())
                        / Volume::new::<U>(V::one())).into());
            }
        }
    }
}
