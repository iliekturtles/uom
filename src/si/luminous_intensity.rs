//! Luminous intensity (base unit candela, cd^(1)).

quantity! {
    /// Luminous intensity (base unit candela, cd^(1)).
    quantity: LuminousIntensity; "luminous intensity";
    /// Luminous intensity dimension, cd^(1).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        P1>;    // luminous intensity
    units {
        @yottacandela: prefix!(yotta); "Ycd"; "yottacandela"; "yottacandelas";
        @zettacandela: prefix!(zetta); "Zcd"; "zettacandela"; "zettacandelas";
        @exacandela: prefix!(exa); "Ecd"; "exacandela"; "exacandelas";
        @petacandela: prefix!(peta); "Pcd"; "petacandela"; "petacandelas";
        @teracandela: prefix!(tera); "Tcd"; "teracandela"; "teracandelas";
        @megacandela: prefix!(mega); "Mcd"; "megacandela"; "megacandelas";
        @kilocandela: prefix!(kilo); "kcd"; "kilocandela"; "kilocandelas";
        @hectocandela: prefix!(hecto); "hcd"; "hectocandela"; "hectocandelas";
        @decacandela: prefix!(deca); "dacd"; "decacandela"; "decacandelas";
        /// The candela is the luminous intensity, in a given direction, of a source that emits
        /// monochromatic radiation of frequency 540 × 10^(12) hertz and that has radiant intensity
        /// in that direction of 1/683 watt per steradian.
        @candela: prefix!(none); "cd"; "candela"; "candelas";
        @decicandela: prefix!(deci); "dcd"; "decicandela"; "decicandelas";
        @centicandela: prefix!(centi); "ccd"; "centicandela"; "centicandelas";
        @millicandela: prefix!(milli); "mcd"; "millicandela"; "millicandelas";
        @microcandela: prefix!(micro); "mcd"; "microcandela"; "microcandelas";
        @nanocandela: prefix!(nano); "ncd"; "nanocandela"; "nanocandelas";
        @picocandela: prefix!(pico); "pcd"; "picocandela"; "picocandelas";
        @femtocandela: prefix!(femto); "fcd"; "femtocandela"; "femtocandelas";
        @attocandela: prefix!(atto); "acd"; "attocandela"; "attocandelas";
        @zeptocandela: prefix!(zepto); "zcd"; "zeptocandela"; "zeptocandelas";
        @yoctocandela: prefix!(yocto); "ycd"; "yoctocandela"; "yoctocandelas";
    }
}
