use typenum::{Z0, P1};
use ::{Scalar};
use ::si::{SI};
use ::si::prefix::*;

pub type ThermodynamicTemperature<V> = Scalar<SI<Z0, Z0, Z0, Z0, P1, Z0, Z0>, V>;

subunits!(ThermodynamicTemperatureSubunits: ThermodynamicTemperature<V> {
    kelvin: 1.0E0;

    yottakelvin: yotta;
    zettakelvin: zetta;
    exakelvin: exa;
    petakelvin: peta;
    terakelvin: tera;
    megakelvin: mega;
    kilokelvin: kilo;
    hectokelvin: hecto;
    decakelvin: deca;
    //kelvin: 1.0E0;
    decikelvin: deci;
    centikelvin: centi;
    millikelvin: milli;
    microkelvin: micro;
    nanokelvin: nano;
    picokelvin: pico;
    femtokelvin: femto;
    attokelvin: atto;
    zeptokelvin: zepto;
    yoctokelvin: yocto;
});
