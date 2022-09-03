//! Electric charge linear density (base unit coulomb per meter, m⁻¹ · A · s).

quantity! {
    ///Electric charge linear density (base unit coulomb per meter, m⁻¹ · A · s).
    quantity: ElectricChargeLinearDensity; "electric charge linear density";
    /// Dimension of electric charge linear density, L⁻¹TI (base unit coulomb per meter,
    /// m⁻¹ · A · s).
    dimension: ISQ<
        N1,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
        kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @coulomb_per_meter: prefix!(none); "C/m", "coulomb per meter", "coulombs per meter";
        @coulomb_per_centimeter: prefix!(none) / prefix!(centi); "C/cm", "coulomb per centimeter",
            "coulombs per centimeter";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::electric_charge as q;
        use crate::si::electric_charge_linear_density as ecld;
        use crate::si::quantities::*;
        use crate::si::length as l;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricChargeLinearDensity<V> = (ElectricCharge::new::<q::coulomb>(V::one())
                / Length::new::<l::meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<q::coulomb, l::meter, ecld::coulomb_per_meter>();
            test::<q::coulomb, l::centimeter, ecld::coulomb_per_centimeter>();

            fn test<Q: q::Conversion<V>, L: l::Conversion<V>, ECLD: ecld::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricChargeLinearDensity::new::<ECLD>(V::one()),
                    &(ElectricCharge::new::<Q>(V::one()) / Length::new::<L>(V::one())).into());
            }
        }
    }
}
