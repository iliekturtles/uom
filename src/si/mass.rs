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

        /// Roman units
        @uncia: 2.74_E-2; "uncia", "uncia", "uncia";
        @sescunx: 4.11_E-2; "sescunx", "sescunx", "sescunx";
        @sextans: 5.48_E-2; "sextans", "sextans", "sextans";
        @quadrans_teruncius: 8.22_E-2; "quadrans teruncius", "quadrans teruncius", "quadrans teruncius";
        @tirens: 1.096_E-1; "tirens", "tirens", "tirens";
        @quincunx: 1.37_E-1; "quincunx", "quincunx", "quincunx";
        @semis: 1.645_E-1; "semis", "semis", "semis";
        @septunx: 1.919_E-1; "septunx", "septunx", "septunx";
        @bes: 2.193_E-1; "bes", "bes", "bes";
        @dodrans: 2.467_E-1; "dodrans", "dodrans", "dodrans";
        @dextans: 2.741_E-1; "dextans", "dextans", "dextans";
        @denux: 3.015_E-1; "denux", "denux", "denux";
        @libra: 3.289_E-1; "libra", "libra", "libra";

        @siliqua: 1.9_E-4; "siliqua", "siliqua", "siliqua";
        @obolus: 5.7_E-4; "obolus", "obolus", "obolus";
        @scrupulum: 1.14_E-3; "scrupulum", "scrupulum", "scrupulum";
        @semisextula: 2.28_E-3; "semisextula", "semisextula", "semisextula";
        @sextula: 4.57_E-3; "sextula", "sextula", "sextula";
        @siciliquus: 6.85_E-3; "siciliquus", "siciliquus", "siciliquus";
        @duella: 9.14_E-3; "duella", "duella", "duella";
        @semuncia: 1.37_E-2; "semuncia", "semuncia", "semuncia";
    }
}
