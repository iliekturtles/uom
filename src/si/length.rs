//! Length (base unit meter, m^(1)).

quantity! {
    /// Length (base unit meter, m^(1)).
    quantity: Length; "length";
    /// Length dimension, m^(1).
    dimension: ISQ<
        P1,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottameter: prefix!(yotta); "Ym", "yottameter", "yottameters";
        @zettameter: prefix!(zetta); "Zm", "zettameter", "zettameters";
        @exameter: prefix!(exa); "Em", "exameter", "exameters";
        @petameter: prefix!(peta); "Pm", "petameter", "petameters";
        @terameter: prefix!(tera); "Tm", "terameter", "terameters";
        @megameter: prefix!(mega); "Mm", "megameter", "megameters";
        @kilometer: prefix!(kilo); "km", "kilometer", "kilometers";
        @hectometer: prefix!(hecto); "hm", "hectometer", "hectometers";
        @decameter: prefix!(deca); "dam", "decameter", "decameters";
        /// The meter is the length of the path travelled by light in a vacuum during a time
        /// interval of 1/299 792 458 of a second.
        @meter: prefix!(none); "m", "meter", "meters";
        @decimeter: prefix!(deci); "dm", "decimeter", "decimeters";
        @centimeter: prefix!(centi); "cm", "centimeter", "centimeters";
        @millimeter: prefix!(milli); "mm", "millimeter", "millimeters";
        @micrometer: prefix!(micro); "µm", "micrometer", "micrometers";
        @nanometer: prefix!(nano); "nm", "nanometer", "nanometers";
        @picometer: prefix!(pico); "pm", "picometer", "picometers";
        @femtometer: prefix!(femto); "fm", "femtometer", "femtometers";
        @attometer: prefix!(atto); "am", "attometer", "attometers";
        @zeptometer: prefix!(zepto); "zm", "zeptometer", "zeptometers";
        @yoctometer: prefix!(yocto); "ym", "yoctometer", "yoctometers";
    }
}
