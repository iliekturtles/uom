//! Velocity (base unit meter per second, m^(1) · s^(-1)).

quantity! {
    /// Velocity (base unit meter per second, m^(1) · s^(-1)).
    quantity: Velocity;
    /// Velocity dimension, m^(1) · s^(-1).
    dimension: ISQ<
        P1,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        yottameter_per_second: prefix!(yotta);
        zettameter_per_second: prefix!(zetta);
        exameter_per_second: prefix!(exa);
        petameter_per_second: prefix!(peta);
        terameter_per_second: prefix!(tera);
        megameter_per_second: prefix!(mega);
        kilometer_per_second: prefix!(kilo);
        hectometer_per_second: prefix!(hecto);
        decameter_per_second: prefix!(deca);
        meter_per_second: prefix!(none);
        decimeter_per_second: prefix!(deci);
        centimeter_per_second: prefix!(centi);
        millimeter_per_second: prefix!(milli);
        micrometer_per_second: prefix!(micro);
        nanometer_per_second: prefix!(nano);
        picometer_per_second: prefix!(pico);
        femtometer_per_second: prefix!(femto);
        attometer_per_second: prefix!(atto);
        zeptometer_per_second: prefix!(zepto);
        yoctometer_per_second: prefix!(yocto);
    }
}
