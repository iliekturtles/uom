use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type ElectricCurrent<V> = Quantity<SI<Z0, Z0, Z0, P1, Z0, Z0, Z0>, V>;

subunits!(electric_current; Units: ElectricCurrent {
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
