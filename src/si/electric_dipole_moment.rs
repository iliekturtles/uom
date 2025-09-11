//! Electric dipole moment (base unit coulomb meter, m · s · A).

quantity! {
    /// Electric dipole moment (base unit coulomb meter, m · s · A).
    quantity: ElectricDipoleMoment; "electric dipole moment";
    /// Dimension of electric dipole moment, LTI (base unit coulomb meter, m · s · A).
    dimension: ISQ<
        P1,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @coulomb_meter: prefix!(none); "C · m", "coulomb meter", "coulomb meters";

        @atomic_unit_of_charge_centimeter: 1.602_176_634_E-19 * prefix!(centi);
            "a.u. of charge · cm", "atomic unit of charge centimeter",
            "atomic unit of charge centimeters";
        @elementary_charge_centimeter: 1.602_176_634_E-19 * prefix!(centi); "e · cm",
            "elementary charge centimeter", "elementary charge centimeters";
        @debye: 1.0 / 299_792_458.0 * 1.0_E-21; "D", "debye", "debyes";
        /// Hartree unit of electric dipole moment e·a₀, where e is elementary charge and a₀ is Bohr
        /// radius.
        @atomic_unit_of_electric_dipole_moment: 8.478_353_625_540_766_E-30; "e · a₀",
            "atomic unit of electric dipole moment", "atomic units of electric dipole moment";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::electric_dipole_moment as edm;
        use crate::si::electric_charge as ec;
        use crate::si::length as l;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricDipoleMoment<V> = ElectricCharge::new::<ec::coulomb>(V::one())
                * Length::new::<l::meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<ec::coulomb, l::meter, edm::coulomb_meter>();
            test::<ec::statcoulomb, l::angstrom, edm::debye>();
            test::<ec::elementary_charge, l::centimeter, edm::elementary_charge_centimeter>();
            test::<ec::elementary_charge, l::centimeter, edm::atomic_unit_of_charge_centimeter>();
            test::<ec::elementary_charge, l::bohr_radius,
                edm::atomic_unit_of_electric_dipole_moment>();

            fn test<EC: ec::Conversion<V>, L: l::Conversion<V>, EDM: edm::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricDipoleMoment::new::<EDM>(V::one()),
                    &(ElectricCharge::new::<EC>(V::one()) * Length::new::<L>(V::one())));
            }
        }
    }
}
