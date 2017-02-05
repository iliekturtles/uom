//! Time (base unit second, s^(1)).

quantity! {
    /// Time (base unit second, s^(1)).
    quantity: Time;
    /// Time dimension, s^(1).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        P1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        yottasecond: prefix!(yotta);
        zettasecond: prefix!(zetta);
        exasecond: prefix!(exa);
        petasecond: prefix!(peta);
        terasecond: prefix!(tera);
        megasecond: prefix!(mega);
        kilosecond: prefix!(kilo);
        hectosecond: prefix!(hecto);
        decasecond: prefix!(deca);
        second: prefix!(none);
        decisecond: prefix!(deci);
        centisecond: prefix!(centi);
        millisecond: prefix!(milli);
        microsecond: prefix!(micro);
        nanosecond: prefix!(nano);
        picosecond: prefix!(pico);
        femtosecond: prefix!(femto);
        attosecond: prefix!(atto);
        zeptosecond: prefix!(zepto);
        yoctosecond: prefix!(yocto);
    }
}
