//! Molar mass (base unit kilogram per mole, kg · mol⁻¹).

quantity! {
    /// Molar mass (base unit kilogram per mole, kg · mol⁻¹).
    quantity: MolarMass; "molar mass";
    /// Dimension of molar mass, MN⁻¹ (base unit kilogram per mole, kg · mol⁻¹).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        N1,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottagram_per_mole: prefix!(yotta) / prefix!(kilo);
            "Yg/mol", "yottagram per mole", "yottagrams per mole";
        @zettagram_per_mole: prefix!(zetta) / prefix!(kilo);
            "Zg/mol", "zettagram per mole", "zettagrams per mole";
        @exagram_per_mole: prefix!(exa) / prefix!(kilo);
            "Eg/mol", "exagram per mole", "exagrams per mole";
        @petagram_per_mole: prefix!(peta) / prefix!(kilo);
            "Pg/mol", "petagram per mole", "petagrams per mole";
        @teragram_per_mole: prefix!(tera) / prefix!(kilo);
            "Tg/mol", "teragram per mole", "teragrams per mole";
        @gigagram_per_mole: prefix!(giga) / prefix!(kilo);
            "Gg/mol", "gigagram per mole", "gigagrams per mole";
        @megagram_per_mole: prefix!(mega) / prefix!(kilo);
            "Mg/mol", "megagram per mole", "megagrams per mole";
        /// The derived unit of molar mass.
        @kilogram_per_mole: prefix!(kilo) / prefix!(kilo);
            "kg/mol", "kilogram per mole", "kilograms per mole";
        @hectogram_per_mole: prefix!(hecto) / prefix!(kilo);
            "hg/mol", "hectogram per mole", "hectograms per mole";
        @decagram_per_mole: prefix!(deca) / prefix!(kilo);
            "dag/mol", "decagram per mole", "decagrams per mole";
        @gram_per_mole: prefix!(none) / prefix!(kilo);
            "g/mol", "gram per mole", "grams per mole";
        @decigram_per_mole: prefix!(deci) / prefix!(kilo);
            "dg/mol", "decigram per mole", "decigrams per mole";
        @centigram_per_mole: prefix!(centi) / prefix!(kilo);
            "cg/mol", "centigram per mole", "centigrams per mole";
        @milligram_per_mole: prefix!(milli) / prefix!(kilo);
            "mg/mol", "milligram per mole", "milligrams per mole";
        @microgram_per_mole: prefix!(micro) / prefix!(kilo);
            "µg/mol", "microgram per mole", "micrograms per mole";
        @nanogram_per_mole: prefix!(nano) / prefix!(kilo);
            "ng/mol", "nanogram per mole", "nanograms per mole";
        @picogram_per_mole: prefix!(pico) / prefix!(kilo);
            "pg/mol", "picogram per mole", "picograms per mole";
        @femtogram_per_mole: prefix!(femto) / prefix!(kilo);
            "fg/mol", "femtogram per mole", "femtograms per mole";
        @attogram_per_mole: prefix!(atto) / prefix!(kilo);
            "ag/mol", "attogram per mole", "attograms per mole";
        @zeptogram_per_mole: prefix!(zepto) / prefix!(kilo);
            "zg/mol", "zeptogram per mole", "zeptograms per mole";
        @yoctogram_per_mole: prefix!(yocto) / prefix!(kilo);
            "yg/mol", "yoctogram per mole", "yoctograms per mole";

        @kilogram_per_particle:  6.022_140_76_E23; "kg/particle", "kilogram per particle",
            "kilograms_per_particle";
        @gram_per_particle: prefix!(milli) * 6.022_140_76_E23; "g/particle", "gram per particle",
            "grams_per_particle";

        @molar_mass_constant: 0.999_999_999_65_E-3; "Mᵤ", "molar mass constant",
            "molar mass constants";
        @molar_mass_of_carbon_12: 11.999_999_995_8_E-3; "M(¹²C)", "molar mass of carbon-12",
            "molar masses of carbon-12";

        @alpha_particle_molar_mass: 4.001_506_177_7_E-3; "M(α)", "alpha particle molar mass",
            "alpha particle molar masses";
        @deuteron_molar_mass: 2.013_553_212_024_460_4_E-3; "M(deuteron)",   // CODATA value 2.013_553_212_05_E-3
            "deuteron molar mass", "deuteron molar masses";
        @electron_molar_mass: 5.485_799_088_728_283_E-7 ; "Mₑ",             // CODATA value 5.485_799_088_8_E-7
            "electron molar mass", "electron molar masses";
        @helion_molar_mass: 3.014_932_246_141_405_4_E-3; "M(helion)",       // CODATA value 3.014_932_246_13_E-3
            "helion molar mass", "helion molar masses";
        @muon_molar_mass: 1.134_289_258_370_581_7_E-4; "M(muon)",           // CODATA value 1.134 289 259 e-4
            "muon molar mass", "muon molar masses";
        @neutron_molar_mass: 1.008_664_915_599_150_3_E-3; "M(neutron)",     // CODATA value 1.008 664 915 60 e-3
            "neutron molar mass", "neutron molar masses";
        @proton_molar_mass: 1.007_276_466_272_316_E-3; "M(proton)",         // CODATA value 1.007 276 466 27 e-3
            "proton molar mass", "proton molar masses";
        @tau_molar_mass: 1.907_537_174_293_039_8_E-3; "M(τ)",               // CODATA value 1.907 54 e-3
            "tau molar mass", "tau molar masses";
        @triton_molar_mass: 3.015_500_715_151_657_E-3; "M(triton)",         // CODATA value 3.015 500 715 17 e-3
            "triton molar mass", "triton molar masses";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::amount_of_substance as aos;
        use crate::si::mass as m;
        use crate::si::molar_mass as mm;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: MolarMass<V> = Mass::new::<m::kilogram>(V::one())
                / AmountOfSubstance::new::<aos::mole>(V::one());
        }

        #[test]
        fn check_units() {
            test::<m::yottagram, mm::yottagram_per_mole>();
            test::<m::zettagram, mm::zettagram_per_mole>();
            test::<m::exagram, mm::exagram_per_mole>();
            test::<m::petagram, mm::petagram_per_mole>();
            test::<m::teragram, mm::teragram_per_mole>();
            test::<m::gigagram, mm::gigagram_per_mole>();
            test::<m::megagram, mm::megagram_per_mole>();
            test::<m::kilogram, mm::kilogram_per_mole>();
            test::<m::hectogram, mm::hectogram_per_mole>();
            test::<m::decagram, mm::decagram_per_mole>();
            test::<m::gram, mm::gram_per_mole>();
            test::<m::decigram, mm::decigram_per_mole>();
            test::<m::centigram, mm::centigram_per_mole>();
            test::<m::milligram, mm::milligram_per_mole>();
            test::<m::microgram, mm::microgram_per_mole>();
            test::<m::nanogram, mm::nanogram_per_mole>();
            test::<m::picogram, mm::picogram_per_mole>();
            test::<m::femtogram, mm::femtogram_per_mole>();
            test::<m::attogram, mm::attogram_per_mole>();
            test::<m::zeptogram, mm::zeptogram_per_mole>();
            test::<m::yoctogram, mm::yoctogram_per_mole>();

            fn test<M: m::Conversion<V>, MM: mm::Conversion<V>>() {
                Test::assert_approx_eq(&MolarMass::new::<MM>(V::one()),
                    &(Mass::new::<M>(V::one())
                        / AmountOfSubstance::new::<aos::mole>(V::one())));
            }
        }

        #[test]
        fn check_units_mass_aos() {
            test::<m::kilogram, aos::particle, mm::kilogram_per_particle>();
            test::<m::gram, aos::particle, mm::gram_per_particle>();
            test::<m::alpha_particle_mass, aos::particle, mm::alpha_particle_molar_mass>();
            test::<m::deuteron_mass, aos::particle, mm::deuteron_molar_mass>();
            test::<m::electron_mass, aos::particle, mm::electron_molar_mass>();
            test::<m::helion_mass, aos::particle, mm::helion_molar_mass>();
            test::<m::muon_mass, aos::particle, mm::muon_molar_mass>();
            test::<m::neutron_mass, aos::particle, mm::neutron_molar_mass>();
            test::<m::proton_mass, aos::particle, mm::proton_molar_mass>();
            test::<m::tau_mass, aos::particle, mm::tau_molar_mass>();
            test::<m::triton_mass, aos::particle, mm::triton_molar_mass>();

            fn test<M: m::Conversion<V>, AOS:aos::Conversion<V>, MM: mm::Conversion<V>>() {
                Test::assert_approx_eq(&MolarMass::new::<MM>(V::one()),
                    &(Mass::new::<M>(V::one())
                        / AmountOfSubstance::new::<AOS>(V::one())));
            }
        }
    }
}
