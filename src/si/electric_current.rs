use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type ElectricCurrent<V> = Quantity<SI<Z0, Z0, Z0, P1, Z0, Z0, Z0>, V>;

subunits!(electric_current; Units: ElectricCurrent {
    yottaampere: yotta;
    zettaampere: zetta;
    exaampere: exa;
    petaampere: peta;
    teraampere: tera;
    megaampere: mega;
    kiloampere: kilo;
    hectoampere: hecto;
    decaampere: deca;
    ampere: 1.0E0;
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
