//! Electric charge volumetric density (base unit coulomb per cubic meter, m⁻³ · A · s).

quantity! {
    ///Electric charge volumetric density (base unit coulomb per cubic meter, m⁻³ · A · s).
    quantity: ElectricChargeVolumetricDensity; "electric charge volumetric density";
    /// Dimension of electric charge volumetric density, TIL⁻³ (base unit coulomb per cubic meter,
    /// m⁻³ · A · s).
    dimension: ISQ<
        N3,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
        kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @coulomb_per_cubic_meter: prefix!(none); "C/m³", "coulomb per cubic meter",
            "coulombs per cubic meter";
        @coulomb_per_cubic_centimeter:
            prefix!(none) / prefix!(centi) / prefix!(centi) / prefix!(centi); "C/cm³",
            "coulomb per cubic centimeter", "coulombs per cubic centimeter";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::electric_charge as q;
        use crate::si::electric_charge_volumetric_density as ecvd;
        use crate::si::quantities::*;
        use crate::si::volume as v;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricChargeVolumetricDensity<V> = (ElectricCharge::new::<q::coulomb>(V::one())
                / Volume::new::<v::cubic_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<q::coulomb, v::cubic_meter, ecvd::coulomb_per_cubic_meter>();
            test::<q::coulomb, v::cubic_centimeter, ecvd::coulomb_per_cubic_centimeter>();

            fn test<Q: q::Conversion<V>, VOL: v::Conversion<V>, ECVD: ecvd::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricChargeVolumetricDensity::new::<ECVD>(V::one()),
                    &(ElectricCharge::new::<Q>(V::one()) / Volume::new::<VOL>(V::one())).into());
            }
        }
    }
}
