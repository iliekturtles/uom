//! Time (base unit second, s^(1)).

use si::ISQ;

quantity! {
    /// Time (base unit second, s^(1)).
    quantity: Time;
    /// Time dimension, s^(1).
    dimension: ISQ<
        ::typenum::Z0,  // length
        ::typenum::Z0,  // mass
        ::typenum::P1,  // time
        ::typenum::Z0,  // electric current
        ::typenum::Z0,  // thermodynamic temperature
        ::typenum::Z0,  // amount of substance
        ::typenum::Z0>; // luminous intensity
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
