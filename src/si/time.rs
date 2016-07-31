use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};
use ::si::prefix::*;

pub type Time<V> = Quantity<SI<Z0, Z0, P1, Z0, Z0, Z0, Z0>, V>;

subunits!(Units: Time<V> {
    yottasecond: yotta;
    zettasecond: zetta;
    exasecond: exa;
    petasecond: peta;
    terasecond: tera;
    megasecond: mega;
    kilosecond: kilo;
    hectosecond: hecto;
    decasecond: deca;
    second: 1.0E0;
    decisecond: deci;
    centisecond: centi;
    millisecond: milli;
    microsecond: micro;
    nanosecond: nano;
    picosecond: pico;
    femtosecond: femto;
    attosecond: atto;
    zeptosecond: zepto;
    yoctosecond: yocto;
});
