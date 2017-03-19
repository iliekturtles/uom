//! Mass (base unit kilogram, kg^(1)).

quantity! {
    /// Mass (base unit kilogram, kg^(1)).
    quantity: Mass; "mass";
    /// Mass dimension, kg^(1).
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
        @megagram: prefix!(mega) / prefix!(kilo); "Mg", "megagram", "megagrams";
        /// The kilogram is the unit of mass; it is equal to the mass of the international prototype
        /// of the kilogram.
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
    }
}
