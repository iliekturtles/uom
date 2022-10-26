//! Electric field (base unit volt per meter, m ⋅ kg ⋅ s⁻³ ⋅ A⁻¹).

quantity! {
    /// Electric field (base unit volt per meter, m ⋅ kg ⋅ s⁻³ ⋅ A⁻¹).
    quantity: ElectricField; "electric field";
    /// Dimension of electric field, LMT⁻³I⁻¹ (base unit volt per meter, m ⋅ kg ⋅ s⁻³ ⋅ A⁻¹).
    dimension: ISQ<
        P1,     // length
        P1,     // mass
        N3,     // time
        N1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @volt_per_meter: prefix!(none); "V/m", "volt per meter", "volts per meter";
        @volt_per_centimeter: prefix!(none) / prefix!(centi); "V/cm", "volt per centimeter",
            "volts per centimeter";
        @volt_per_millimeter: prefix!(none) / prefix!(milli); "V/mm", "volt per millimeter",
            "volts per millimeter";
        @volt_per_micrometer: prefix!(none) / prefix!(micro); "V/μm", "volt per micrometer",
            "volts per micrometer";
        @kilovolt_per_millimeter: prefix!(kilo) / prefix!(milli); "kV/mm",
            "kilovolt per millimeter", "kilovolts per millimeter";
        @megavolt_per_meter: prefix!(mega); "MV/m", "megavolt per meter", "megavolts per meter";
        @megavolt_per_centimeter: prefix!(mega) / prefix!(centi); "MV/cm",
            "megavolt per centimeter", "megavolts per centimeter";
        @volt_per_mil: prefix!(none) / 2.54_E-5; "V/mil", "volt per mil", "volts per mil";

        /// Hartree atomic unit of electric field Eₕ / (e ⋅ a₀), where Eₕ is Hartree energy, e is
        /// elementary charge, and a₀ is Bohr radius.
        @atomic_unit_of_electric_field: 5.142_206_747_632_595_E11; "a.u. of electric field",
            "atomic unit of electric field", "atomic units of electric field";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::electric_field as ef;
        use crate::si::quantities::*;
        use crate::si::electric_potential as ep;
        use crate::si::energy as en;
        use crate::si::electric_charge as ec;
        use crate::si::length as l;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricField<V> = ElectricPotential::new::<ep::volt>(V::one())
                / Length::new::<l::meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<ep::volt, l::meter, ef::volt_per_meter>();
            test::<ep::volt, l::centimeter, ef::volt_per_centimeter>();
            test::<ep::volt, l::millimeter, ef::volt_per_millimeter>();
            test::<ep::volt, l::micrometer, ef::volt_per_micrometer>();
            test::<ep::kilovolt, l::millimeter, ef::kilovolt_per_millimeter>();
            test::<ep::megavolt, l::centimeter, ef::megavolt_per_centimeter>();
            test::<ep::megavolt, l::meter, ef::megavolt_per_meter>();
            test::<ep::volt, l::mil, ef::volt_per_mil>();

            fn test<EP: ep::Conversion<V>, L: l::Conversion<V>, EF: ef::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricField::new::<EF>(V::one()),
                    &(ElectricPotential::new::<EP>(V::one())
                        / Length::new::<L>(V::one())));
            }
        }

        #[test]
        fn check_units_eql() {
            test::<en::joule, ec::coulomb, l::meter, ef::volt_per_meter>();
            test::<en::hartree, ec::elementary_charge, l::bohr_radius,
                ef::atomic_unit_of_electric_field>();

            fn test<EN: en::Conversion<V>, Q: ec::Conversion<V>, L: l::Conversion<V>,
                EF: ef::Conversion<V>>()
            {
                Test::assert_approx_eq(&ElectricField::new::<EF>(V::one()),
                    &(Energy::new::<EN>(V::one())
                        / ElectricCharge::new::<Q>(V::one())
                        / Length::new::<L>(V::one())));
            }
        }
    }
}
