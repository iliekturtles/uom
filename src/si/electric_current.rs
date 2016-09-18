use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type ElectricCurrent<U, V> = Quantity<SI<Z0, Z0, Z0, P1, Z0, Z0, Z0>, U, V>;

units!(electric_current ElectricCurrent {
    yottaampere: prefix!(yotta);
    zettaampere: prefix!(zetta);
    exaampere: prefix!(exa);
    petaampere: prefix!(peta);
    teraampere: prefix!(tera);
    megaampere: prefix!(mega);
    kiloampere: prefix!(kilo);
    hectoampere: prefix!(hecto);
    decaampere: prefix!(deca);
    ampere: 1.0E0;
    deciampere: prefix!(deci);
    centiampere: prefix!(centi);
    milliampere: prefix!(milli);
    microampere: prefix!(micro);
    nanoampere: prefix!(nano);
    picoampere: prefix!(pico);
    femtoampere: prefix!(femto);
    attoampere: prefix!(atto);
    zeptoampere: prefix!(zepto);
    yoctoampere: prefix!(yocto);
});
