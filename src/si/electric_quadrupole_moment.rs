//! Electric quadrupole moment (base unit coulomb square meter, m² · s · A).

quantity! {
    /// Electric quadrupole moment (base unit coulomb square meter, m² · s · A).
    quantity: ElectricQuadrupoleMoment; "electric quadrupole moment";
    /// Dimension of electric quadrupole moment, LTI (base unit coulomb square meter, m² · s · A).
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @coulomb_square_meter: prefix!(none); "C · m²", "coulomb square meter",
            "coulomb square meters";
        @elementary_charge_barn: 1.602_176_634_E-19 * 1.0_E-28; "e · b", "elementary charge barn",
            "elementary charge barns";
        /// Hartree unit of electric quadrupole moment e · a₀², where e is elementary charge and a₀
        /// is Bohr radius.
        @atomic_unit_of_electric_quadrupole_moment: 4.486_551_524_613_E-40;  "e · a₀²",
            "atomic unit of electric quadrupole moment",
            "atomic units of electric quadrupole moment";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::electric_quadrupole_moment as edm;
        use crate::si::electric_charge as ec;
        use crate::si::area as a;
        use crate::si::length as l;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricQuadrupoleMoment<V> = ElectricCharge::new::<ec::coulomb>(V::one())
                * Area::new::<a::square_meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<ec::coulomb, a::square_meter, edm::coulomb_square_meter>();
            test::<ec::elementary_charge, a::barn, edm::elementary_charge_barn>();

            fn test<EC: ec::Conversion<V>, A: a::Conversion<V>, EDM: edm::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricQuadrupoleMoment::new::<EDM>(V::one()),
                    &(ElectricCharge::new::<EC>(V::one()) * Area::new::<A>(V::one())));
            }
        }

        #[test]
        fn check_units_charge_length() {
            test::<ec::elementary_charge, l::bohr_radius, edm::elementary_charge_barn>();

            fn test<EC: ec::Conversion<V>, L: l::Conversion<V>, EDM: edm::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricQuadrupoleMoment::new::<EDM>(V::one()),
                    &(ElectricCharge::new::<EC>(V::one()) * Length::new::<L>(V::one())
                        * Length::new::<L>(V::one())));
            }
        }
    }
}
