//! Magnetic moment (base unit ampere square meter A · m²).

quantity! {
    /// Magnetic moment (base unit ampere square meter A · m²).
    quantity: MagneticMoment; "magnetic moment";
    /// Dimension of magnetic moment, IL² (base unit ampere square meter A · m²).
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        Z0,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @ampere_square_meter: prefix!(none); "A · m²", "ampere square meter",
            "ampere square meters";
        @joule_per_tesla: prefix!(none); "J/T", "joule per tesla", "joules per tesla";
        @newton_meter_per_tesla: prefix!(none); "N · m/T", "newton meter per tesla",
            "newton meters per tesla";
        @ampere_square_centimeter: prefix!(none) * prefix!(centi) * prefix!(centi); "A · cm²",
            "ampere square centimeter", "ampere square centimeters";

        @statampere_square_centimeter: 3.335_641_E-10 * prefix!(centi) * prefix!(centi);
            "statA · cm²", "statampere square centimeter", "statampere square centimeters";
        @erg_per_gauss:  1.0_E-7 / 1.0_E-4; "erg/G", "erg per gauss", "ergs per gauss";

        @bohr_magneton: 9.274_010_0783_E-24; "µ(Bohr)", "Bohr magneton", "Bohr magnetons";
        @nuclear_magneton: 5.050_783_7461_E-27; "μ(Nuclear)", "nuclear magneton",
            "nuclear magnetons";
        @atomic_unit_of_magnetic_dipole_moment: 1.854_802_015_66_E-23; "ħ · e/mₑ",
            "atomic unit of magnetic dipole moment", " atomic units of magnetic dipole moment";
        @deuteron_magnetic_moment: 4.330_735_094_E-27; "μ(deuteron)", "deuteron magnetic moment",
            "deuteron magnetic moments";
        @electron_magnetic_moment: -9.284_764_704_3_E-24; "μₑ", "electron magnetic moment",
            "electron magnetic moments";
        @helion_magnetic_moment: -1.074_617_532_E-26; "μ(helion)", "helion magnetic moment",
            "helion magnetic moments";
        @muon_magnetic_moment: -4.490_448_30_E-26; "μ(muon)", "muon magnetic moment",
            "muon magnetic moments";
        @neutron_magnetic_moment: -9.662_365_1_E-27; "μ(neutron)", "neutron magnetic moment",
            "neutron magnetic moments";
        @proton_magnetic_moment: 1.410_606_797_36_E-26; "μ(proton)", "proton magnetic moment",
            "proton magnetic moments";
        @shielded_helion_magnetic_moment: -1.074_553_090_E-26; "μ’(helion)",
            "shielded helion magnetic moment", "shielded helion magnetic moments";
        @shielded_proton_magnetic_moment: 1.410_570_560_E-26; "μ’(proton)",
            "shielded proton magnetic moment", "shielded proton magnetic moments";
        @triton_magnetic_moment: 1.504_609_520_2_E-26; "μ(triton)", "triton magnetic moment",
            "triton magnetic moments";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::magnetic_moment as mm;
        use crate::si::electric_current as ec;
        use crate::si::quantities::*;
        use crate::si::energy as e;
        use crate::si::magnetic_flux_density as mfd;
        use crate::si::area as area;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: MagneticMoment<V> = ElectricCurrent::new::<ec::ampere>(V::one())
                * Area::new::<area::square_meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<ec::ampere, area::square_meter, mm::ampere_square_meter>();
            test::<ec::ampere, area::square_meter, mm::joule_per_tesla>();
            test::<ec::ampere, area::square_meter, mm::newton_meter_per_tesla>();
            test::<ec::ampere, area::square_centimeter, mm::ampere_square_centimeter>();
            test::<ec::statampere, area::square_centimeter, mm::statampere_square_centimeter>();

            fn test<EC: ec::Conversion<V>, A: area::Conversion<V>, MM: mm::Conversion<V>>() {
                Test::assert_approx_eq(&MagneticMoment::new::<MM>(V::one()),
                    &(ElectricCurrent::new::<EC>(V::one())
                        * Area::new::<A>(V::one())));
            }
        }

        #[test]
        fn check_units_energy_per_field() {
            test::<e::joule, mfd::tesla, mm::ampere_square_meter>();
            test::<e::joule, mfd::tesla, mm::joule_per_tesla>();
            test::<e::joule, mfd::tesla, mm::newton_meter_per_tesla>();
            test::<e::erg, mfd::gauss, mm::erg_per_gauss>();

            fn test<E: e::Conversion<V>, MFD: mfd::Conversion<V>, MM: mm::Conversion<V>>() {
                Test::assert_approx_eq(&MagneticMoment::new::<MM>(V::one()),
                    &(Energy::new::<E>(V::one())
                        / MagneticFluxDensity::new::<MFD>(V::one())));
            }
        }
    }
}
