use typenum::{Z0, P1};
use ::{Scalar};
use ::si::{SI};
use ::si::prefix::*;

pub type ElectricCurrent<V> = Scalar<SI<Z0, Z0, Z0, P1, Z0, Z0, Z0>, V>;

subunits!(ElectricCurrentSubunits: ElectricCurrent<V> {
    ampere: 1.0E0;

    yottaampere: yotta;
    zettaampere: zetta;
    exaampere: exa;
    petaampere: peta;
    teraampere: tera;
    megaampere: mega;
    kiloampere: kilo;
    hectoampere: hecto;
    decaampere: deca;
    //ampere: 1.0E0;
    deciampere: deci;
    centiampere: centi;
    milliampere: milli;
    microampere: micro;
    nanoampere: nano;
    picoampere: pico;
    femtoampere: femto;
    attoampere: atto;
    zeptoampere: zepto;
    yoctoampere: yocto;
});
