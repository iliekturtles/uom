//! Velocity (base unit meter per second, m^(1) · s^(-1)).

quantity! {
    /// Velocity (base unit meter per second, m^(1) · s^(-1)).
    quantity: Velocity; "velocity";
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
        @yottameter_per_second: prefix!(yotta); "Ym/s", "yottameter per second",
            "yottameters per second";
        @zettameter_per_second: prefix!(zetta); "Zm/s", "zettameter per second",
            "zettameters per second";
        @exameter_per_second: prefix!(exa); "Em/s", "exameter per second",
            "exameters per second";
        @petameter_per_second: prefix!(peta); "Pm/s", "petameter per second",
            "petameters per second";
        @terameter_per_second: prefix!(tera); "Tm/s", "terameter per second",
            "terameters per second";
        @megameter_per_second: prefix!(mega); "Mm/s", "megameter per second",
            "megameters per second";
        @kilometer_per_second: prefix!(kilo); "km/s", "kilometer per second",
            "kilometers per second";
        @hectometer_per_second: prefix!(hecto); "hm/s", "hectometer per second",
            "hectometers per second";
        @decameter_per_second: prefix!(deca); "dam/s", "decameter per second",
            "decameters per second";
        @meter_per_second: prefix!(none); "m/s", "meter per second", "meters per second";
        @decimeter_per_second: prefix!(deci); "dm/s", "decimeter per second",
            "decimeters per second";
        @centimeter_per_second: prefix!(centi); "cm/s", "centimeter per second",
            "centimeters per second";
        @millimeter_per_second: prefix!(milli); "mm/s", "millimeter per second",
            "millimeters per second";
        @micrometer_per_second: prefix!(micro); "µm/s", "micrometer per second",
            "micrometers per second";
        @nanometer_per_second: prefix!(nano); "nanom/s", "nanometer per second",
            "nanometers per second";
        @picometer_per_second: prefix!(pico); "pm/s", "picometer per second",
            "picometers per second";
        @femtometer_per_second: prefix!(femto); "fm/s", "femtometer per second",
            "femtometers per second";
        @attometer_per_second: prefix!(atto); "am/s", "attometer per second",
            "attometers per second";
        @zeptometer_per_second: prefix!(zepto); "zm/s", "zeptometer per second",
            "zeptometers per second";
        @yoctometer_per_second: prefix!(yocto); "ym/s", "yoctometer per second",
            "yoctometers per second";
    }
}
