//! Length (base unit meter, m^(1)).

use si::ISQ;

quantity! {
    /// Length (base unit meter, m^(1)).
    quantity: Length;
    /// Length dimension, m^(1).
    dimension: ISQ<
        ::typenum::P1,  // length
        ::typenum::Z0,  // mass
        ::typenum::Z0,  // time
        ::typenum::Z0,  // electric current
        ::typenum::Z0,  // thermodynamic temperature
        ::typenum::Z0,  // amount of substance
        ::typenum::Z0>; // luminous intensity
    units {
        yottameter: prefix!(yotta);
        zettameter: prefix!(zetta);
        exameter: prefix!(exa);
        petameter: prefix!(peta);
        terameter: prefix!(tera);
        megameter: prefix!(mega);
        kilometer: prefix!(kilo);
        hectometer: prefix!(hecto);
        decameter: prefix!(deca);
        meter: prefix!(none);
        decimeter: prefix!(deci);
        centimeter: prefix!(centi);
        millimeter: prefix!(milli);
        micrometer: prefix!(micro);
        nanometer: prefix!(nano);
        picometer: prefix!(pico);
        femtometer: prefix!(femto);
        attometer: prefix!(atto);
        zeptometer: prefix!(zepto);
        yoctometer: prefix!(yocto);
    }
}
