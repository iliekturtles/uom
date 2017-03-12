//! Thermodynamic temperature (base unit kelvin, K^(1)).

quantity! {
    /// Thermodynamic temperature (base unit kelvin, K^(1)).
    quantity: ThermodynamicTemperature; "thermodynamic temperature";
    /// Thermodynamic temperature dimension, K^(1).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        P1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottakelvin: prefix!(yotta); "YK"; "yottakelvin"; "yottakelvins";
        @zettakelvin: prefix!(zetta); "ZK"; "zettakelvin"; "zettakelvins";
        @exakelvin: prefix!(exa); "EK"; "exakelvin"; "exakelvins";
        @petakelvin: prefix!(peta); "PK"; "petakelvin"; "petakelvins";
        @terakelvin: prefix!(tera); "TK"; "terakelvin"; "terakelvins";
        @megakelvin: prefix!(mega); "MK"; "megakelvin"; "megakelvins";
        @kilokelvin: prefix!(kilo); "kK"; "kilokelvin"; "kilokelvins";
        @hectokelvin: prefix!(hecto); "hK"; "hectokelvin"; "hectokelvins";
        @decakelvin: prefix!(deca); "daK"; "decakelvin"; "decakelvins";
        /// The kelvin, unit of thermodynamic temperature, is the fraction of 1/273.16 of the
        /// thermodynamic temperature of the triple point of water.
        @kelvin: prefix!(none); "K"; "kelvin"; "kelvins";
        @decikelvin: prefix!(deci); "dK"; "decikelvin"; "decikelvins";
        @centikelvin: prefix!(centi); "cK"; "centikelvin"; "centikelvins";
        @millikelvin: prefix!(milli); "mK"; "millikelvin"; "millikelvins";
        @microkelvin: prefix!(micro); "µK"; "microkelvin"; "microkelvins";
        @nanokelvin: prefix!(nano); "nK"; "nanokelvin"; "nanokelvins";
        @picokelvin: prefix!(pico); "pK"; "picokelvin"; "picokelvins";
        @femtokelvin: prefix!(femto); "fK"; "femtokelvin"; "femtokelvins";
        @attokelvin: prefix!(atto); "aK"; "attokelvin"; "attokelvins";
        @zeptokelvin: prefix!(zepto); "zK"; "zeptokelvin"; "zeptokelvins";
        @yoctokelvin: prefix!(yocto); "yK"; "yoctokelvin"; "yoctokelvins";
    }
}
