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
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::mass as m;
        use si::amount_of_substance as aos;
        use si::molar_mass as mm;
        use tests::Test;

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
    }
}
