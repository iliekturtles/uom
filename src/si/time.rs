use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type Time<U, V> = Quantity<SI<Z0, Z0, P1, Z0, Z0, Z0, Z0>, U, V>;

units!(time Time {
    yottasecond: prefix!(yotta);
    zettasecond: prefix!(zetta);
    exasecond: prefix!(exa);
    petasecond: prefix!(peta);
    terasecond: prefix!(tera);
    megasecond: prefix!(mega);
    kilosecond: prefix!(kilo);
    hectosecond: prefix!(hecto);
    decasecond: prefix!(deca);
    second: 1.0E0;
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

    minute: 6.0E1;
});
