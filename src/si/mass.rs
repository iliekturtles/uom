//! Mass (base unit kilogram, kg).

quantity! {
    /// Mass (base unit kilogram, kg).
    quantity: Mass; "mass";
    /// Mass dimension, M (base unit kilogram, kg).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottagram: prefix!(yotta) / prefix!(kilo); "Yg", "yottagram", "yottagrams";
        @zettagram: prefix!(zetta) / prefix!(kilo); "Zg", "zettagram", "zettagrams";
        @exagram: prefix!(exa) / prefix!(kilo); "Eg", "exagram", "exagrams";
        @petagram: prefix!(peta) / prefix!(kilo); "Pg", "petagram", "petagrams";
        @teragram: prefix!(tera) / prefix!(kilo); "Tg", "teragram", "teragrams";
        @gigagram: prefix!(giga) / prefix!(kilo); "Gg", "gigagram", "gigagrams";
        @megagram: prefix!(mega) / prefix!(kilo); "Mg", "megagram", "megagrams";
        /// The kilogram is the SI unit of mass. It is defined by taking the fixed numerical value
        /// of the Planck constant *h* to be 6.626 070 15 × 10⁻³⁴ when expressed in the unit J s,
        /// which is equal to kg m² s⁻¹, where the meter and the second are defined in terms of *c*
        /// and ∆*ν*<sub>Cs</sub>.
        @kilogram: prefix!(kilo) / prefix!(kilo); "kg", "kilogram", "kilograms";
        @hectogram: prefix!(hecto) / prefix!(kilo); "hg", "hectogram", "hectograms";
        @decagram: prefix!(deca) / prefix!(kilo); "dag", "decagram", "decagrams";
        @gram: prefix!(none) / prefix!(kilo); "g", "gram", "grams";
        @decigram: prefix!(deci) / prefix!(kilo); "dg", "decigram", "decigrams";
        @centigram: prefix!(centi) / prefix!(kilo); "cg", "centigram", "centigrams";
        @milligram: prefix!(milli) / prefix!(kilo); "mg", "milligram", "milligrams";
        @microgram: prefix!(micro) / prefix!(kilo); "µg", "microgram", "micrograms";
        @nanogram: prefix!(nano) / prefix!(kilo); "ng", "nanogram", "nanograms";
        @picogram: prefix!(pico) / prefix!(kilo); "pg", "picogram", "picograms";
        @femtogram: prefix!(femto) / prefix!(kilo); "fg", "femtogram", "femtograms";
        @attogram: prefix!(atto) / prefix!(kilo); "ag", "attogram", "attograms";
        @zeptogram: prefix!(zepto) / prefix!(kilo); "zg", "zeptogram", "zeptograms";
        @yoctogram: prefix!(yocto) / prefix!(kilo); "yg", "yoctogram", "yoctograms";

        @carat: 2.0_E-4; "ct", "carat", "carats";
        /// Unified atomic mass unit.
        @dalton: 1.660_539_066_60_E-27; "Da", "dalton", "daltons";
        @grain: 6.479_891_E-5; "gr", "grain", "grains";
        @hundredweight_long: 5.080_235_E1; "cwt long", "hundredweight (long)", "hundredweight (long)";
        @hundredweight_short: 4.535_924_E1; "cwt short", "hundredweight (short)", "hundredweight (short)";
        @ounce: 2.834_952_E-2; "oz", "ounce", "ounces";
        @ounce_troy: 3.110_348_E-2; "oz t", "troy ounce", "troy ounces";
        @pennyweight: 1.555_174_E-3; "dwt", "pennyweight", "pennyweight";
        @pound: 4.535_924_E-1; "lb", "pound", "pounds";
        @pound_troy: 3.732_417_E-1; "lb t", "troy pound", "troy pounds";
        @slug: 1.459_390_E1; "slug", "slug", "slugs";
        @ton_assay: 2.916_667_E-2; "AT", "assay ton", "assay tons";
        @ton_long: 1.016_047_E3; "2240 lb", "long ton", "long tons";
        @ton_short: 9.071_847_E2; "2000 lb", "short ton", "short tons";
         /// Ton Metric
        @ton: 1.0_E3; "t", "ton", "tons";

        // Ancient Roman units.
        @libra: 3.289_E-1; "libra", "libra", "librae";
        @deunx: 3.014_916_666_666_666_6_E-1; "deunx", "deunx", "deunces";
        @dextans: 2.740_833_333_333_333_5_E-1; "dextans", "dextans", "dextantes";
        @dodrans: 2.466_75_E-1; "dodrans", "dodrans", "dodrantes";
        @bes: 2.192_666_666_666_666_7_E-1; "bes", "bes", "bessis";
        @septunx: 1.918_583_333_333_333_5_E-1; "septunx", "septunx", "septunces";
        @semis: 1.644_5_E-1; "semis", "semis", "semisses";
        @quincunx: 1.370_416_666_666_666_7_E-1; "quincunx", "quincunx", "quincunx";
        @triens: 1.096_333_333_333_333_3_E-1; "triens", "triens", "trientes";
        @quadrans_teruncius: 8.222_5_E-2; "quadrans teruncius", "quadrans teruncius",
            "quadrantes teruncius";
        @sextans: 5.481_666_666_666_67_E-2; "sextans", "sextans", "sextantes";
        @sescuncia: 4.111_25_E-2; "sescuncia", "sescuncia", "sescunciae";
        @uncia: 2.740_833_333_333_333_7_E-2; "uncia", "uncia", "unciae";
        @semuncia: 1.370_416_666_666_67_E-2; "semuncia", "semuncia", "semunciae";
        @duella: 9.136_111_111_111_11_E-3; "duella", "duella", "duella";
        @sicilicus: 6.852_083_333_333_33_E-3; "sicilicus", "sicilicus", "scilici";
        @sextula: 4.568_055_555_555_56_E-3; "sextula", "sextula", "sextulae";
        @semisextula: 2.284_027_777_777_78_E-3; "semisextula", "semisextula", "semisextulae";
        @scrupulum: 1.142_013_888_888_89_E-3; "scrupulum", "scrupulum", "scrupula";
        @obolus: 5.710_069_444_444_44_E-4; "obolus", "obolus", "oboli";
        @siliqua: 1.903_356_481_481_48_E-4; "siliqua", "siliqua", "siliquae";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::{FromPrimitive, One};
        use crate::si::mass as m;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_roman() {
            test::<m::deunx>(1.1_E1 / 1.2_E1);
            test::<m::dextans>(5.0_E0 / 6.0_E0);
            test::<m::dodrans>(3.0_E0 / 4.0_E0);
            test::<m::bes>(2.0_E0 / 3.0_E0);
            test::<m::septunx>(7.0_E0 / 1.2_E1);
            test::<m::semis>(5.0_E-1);
            test::<m::quincunx>(5.0_E0 / 1.2_E1);
            test::<m::triens>(1.0_E0 / 3.0_E0);
            test::<m::quadrans_teruncius>(1.0_E0 / 4.0_E0);
            test::<m::sextans>(1.0_E0 / 6.0_E0);
            test::<m::sescuncia>(1.0_E0 / 8.0_E0);
            test::<m::uncia>(1.0_E0 / 1.2_E1);
            test::<m::semuncia>(1.0_E0 / 2.4_E1);
            test::<m::duella>(1.0_E0 / 3.6_E1);
            test::<m::sicilicus>(1.0_E0 / 4.8_E1);
            test::<m::sextula>(1.0_E0 / 7.2_E1);
            test::<m::semisextula>(1.0_E0 / 1.44_E2);
            test::<m::scrupulum>(1.0_E0 / 2.88_E2);
            test::<m::obolus>(1.0_E0 / 5.76_E2);
            test::<m::siliqua>(1.0_E0 / 1.728_E3);

            fn test<M: m::Conversion<V>>(l: f64) {
                Test::assert_approx_eq(&Mass::new::<M>(V::one()),
                    &Mass::new::<m::libra>(V::from_f64(l).unwrap()));
            }
        }
    }
}
