//! Length (base unit meter, m^(1)).

quantity! {
    /// Length (base unit meter, m^(1)).
    quantity: Length;
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
        @yottameter: prefix!(yotta);
        @zettameter: prefix!(zetta);
        @exameter: prefix!(exa);
        @petameter: prefix!(peta);
        @terameter: prefix!(tera);
        @megameter: prefix!(mega);
        @kilometer: prefix!(kilo);
        @hectometer: prefix!(hecto);
        @decameter: prefix!(deca);
        /// The meter is the length of the path travelled by light in a vacuum during a time
        /// interval of 1/299 792 458 of a second.
        @meter: prefix!(none);
        @decimeter: prefix!(deci);
        @centimeter: prefix!(centi);
        @millimeter: prefix!(milli);
        @micrometer: prefix!(micro);
        @nanometer: prefix!(nano);
        @picometer: prefix!(pico);
        @femtometer: prefix!(femto);
        @attometer: prefix!(atto);
        @zeptometer: prefix!(zepto);
        @yoctometer: prefix!(yocto);
    }
}
