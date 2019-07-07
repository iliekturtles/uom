//! Luminous intensity (base unit candela, cd).

quantity! {
    /// Luminous intensity (base unit candela, cd).
    quantity: LuminousIntensity; "luminous intensity";
    /// Dimension of luminous intensity, J (base unit candela, cd).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        P1>;    // luminous intensity
    units {
        @yottacandela: prefix!(yotta); "Ycd", "yottacandela", "yottacandelas";
        @zettacandela: prefix!(zetta); "Zcd", "zettacandela", "zettacandelas";
        @exacandela: prefix!(exa); "Ecd", "exacandela", "exacandelas";
        @petacandela: prefix!(peta); "Pcd", "petacandela", "petacandelas";
        @teracandela: prefix!(tera); "Tcd", "teracandela", "teracandelas";
        @gigacandela: prefix!(giga); "Gcd", "gigacandela", "gigacandelas";
        @megacandela: prefix!(mega); "Mcd", "megacandela", "megacandelas";
        @kilocandela: prefix!(kilo); "kcd", "kilocandela", "kilocandelas";
        @hectocandela: prefix!(hecto); "hcd", "hectocandela", "hectocandelas";
        @decacandela: prefix!(deca); "dacd", "decacandela", "decacandelas";
        /// The candela is the luminous intensity, in a given direction, of a source that emits
        /// monochromatic radiation of frequency 540 × 10¹² hertz and that has radiant intensity in
        /// that direction of 1/683 watt per steradian.
        @candela: prefix!(none); "cd", "candela", "candelas";
        @decicandela: prefix!(deci); "dcd", "decicandela", "decicandelas";
        @centicandela: prefix!(centi); "ccd", "centicandela", "centicandelas";
        @millicandela: prefix!(milli); "mcd", "millicandela", "millicandelas";
        @microcandela: prefix!(micro); "µcd", "microcandela", "microcandelas";
        @nanocandela: prefix!(nano); "ncd", "nanocandela", "nanocandelas";
        @picocandela: prefix!(pico); "pcd", "picocandela", "picocandelas";
        @femtocandela: prefix!(femto); "fcd", "femtocandela", "femtocandelas";
        @attocandela: prefix!(atto); "acd", "attocandela", "attocandelas";
        @zeptocandela: prefix!(zepto); "zcd", "zeptocandela", "zeptocandelas";
        @yoctocandela: prefix!(yocto); "ycd", "yoctocandela", "yoctocandelas";
    }
}
