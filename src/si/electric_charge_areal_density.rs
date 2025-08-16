//! Electric charge areal density (base unit coulomb per square meter, m⁻² · A · s).

quantity! {
    ///Electric charge areal density (base unit coulomb per square meter, m⁻² · A · s).
    quantity: ElectricChargeArealDensity; "electric charge areal density";
    /// Dimension of electric charge areal density, L⁻²TI (base unit coulomb per square meter,
    /// m⁻² · A · s).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
        kind: dyn crate::si::marker::ConstituentConcentrationKind;
    units {
        @coulomb_per_square_meter: prefix!(none); "C/m²", "coulomb per square meter",
            "coulombs per square meter";
        @coulomb_per_square_centimeter: prefix!(none) / prefix!(centi) / prefix!(centi); "C/cm²",
            "coulomb per square centimeter", "coulombs per square centimeter";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::electric_charge as q;
        use crate::si::electric_charge_areal_density as ecad;
        use crate::si::quantities::*;
        use crate::si::area as a;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricChargeArealDensity<V> = (ElectricCharge::new::<q::coulomb>(V::one())
                / Area::new::<a::square_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<q::coulomb, a::square_meter, ecad::coulomb_per_square_meter>();
            test::<q::coulomb, a::square_centimeter, ecad::coulomb_per_square_centimeter>();

            fn test<Q: q::Conversion<V>, A: a::Conversion<V>, ECAD: ecad::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricChargeArealDensity::new::<ECAD>(V::one()),
                    &(ElectricCharge::new::<Q>(V::one()) / Area::new::<A>(V::one())).into());
            }
        }
    }
}
