//! Thermodynamic temperature (base unit kelvin, K^(1)).

quantity! {
    /// Thermodynamic temperature (base unit kelvin, K^(1)).
    quantity: ThermodynamicTemperature;
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
