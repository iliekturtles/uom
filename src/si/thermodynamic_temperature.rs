//! Thermodynamic temperature (base unit kelvin, K^(1)).

use si::ISQ;

quantity! {
    /// Thermodynamic temperature (base unit kelvin, K^(1)).
    quantity: ThermodynamicTemperature;
    /// Thermodynamic temperature dimension, K^(1).
    dimension: ISQ<
        ::typenum::Z0,  // length
        ::typenum::Z0,  // mass
        ::typenum::Z0,  // time
        ::typenum::Z0,  // electric current
        ::typenum::P1,  // thermodynamic temperature
        ::typenum::Z0,  // amount of substance
        ::typenum::Z0>; // luminous intensity
    units {
        yottakelvin: prefix!(yotta);
        zettakelvin: prefix!(zetta);
        exakelvin: prefix!(exa);
        petakelvin: prefix!(peta);
        terakelvin: prefix!(tera);
        megakelvin: prefix!(mega);
        kilokelvin: prefix!(kilo);
        hectokelvin: prefix!(hecto);
        decakelvin: prefix!(deca);
        kelvin: prefix!(none);
        decikelvin: prefix!(deci);
        centikelvin: prefix!(centi);
        millikelvin: prefix!(milli);
        microkelvin: prefix!(micro);
        nanokelvin: prefix!(nano);
        picokelvin: prefix!(pico);
        femtokelvin: prefix!(femto);
        attokelvin: prefix!(atto);
        zeptokelvin: prefix!(zepto);
        yoctokelvin: prefix!(yocto);
    }
}
