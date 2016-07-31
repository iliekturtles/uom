use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};
use ::si::prefix::*;

pub type ThermodynamicTemperature<V> = Quantity<SI<Z0, Z0, Z0, Z0, P1, Z0, Z0>, V>;

// Needs temperature specific conversion implentation for Fahrenheit/Rankine.
//subunits!(Units: ThermodynamicTemperature<V> {
//    yottakelvin: yotta;
//    zettakelvin: zetta;
//    exakelvin: exa;
//    petakelvin: peta;
//    terakelvin: tera;
//    megakelvin: mega;
//    kilokelvin: kilo;
//    hectokelvin: hecto;
//    decakelvin: deca;
//    kelvin: 1.0E0;
//    decikelvin: deci;
//    centikelvin: centi;
//    millikelvin: milli;
//    microkelvin: micro;
//    nanokelvin: nano;
//    picokelvin: pico;
//    femtokelvin: femto;
//    attokelvin: atto;
//    zeptokelvin: zepto;
//    yoctokelvin: yocto;
//});
