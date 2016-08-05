use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type Mass<V> = Quantity<SI<Z0, P1, Z0, Z0, Z0, Z0, Z0>, V>;

subunits!(mass; Units: Mass {
    yottagram: prefix!(yotta) / prefix!(kilo);
    zettagram: prefix!(zetta) / prefix!(kilo);
    exagram: prefix!(exa) / prefix!(kilo);
    petagram: prefix!(peta) / prefix!(kilo);
    teragram: prefix!(tera) / prefix!(kilo);
    megagram: prefix!(mega) / prefix!(kilo);
    kilogram: prefix!(kilo) / prefix!(kilo);
    hectogram: prefix!(hecto) / prefix!(kilo);
    decagram: prefix!(deca) / prefix!(kilo);
    gram: 1.0E0 / prefix!(kilo);
    decigram: prefix!(deci) / prefix!(kilo);
    centigram: prefix!(centi) / prefix!(kilo);
    milligram: prefix!(milli) / prefix!(kilo);
    microgram: prefix!(micro) / prefix!(kilo);
    nanogram: prefix!(nano) / prefix!(kilo);
    picogram: prefix!(pico) / prefix!(kilo);
    femtogram: prefix!(femto) / prefix!(kilo);
    attogram: prefix!(atto) / prefix!(kilo);
    zeptogram: prefix!(zepto) / prefix!(kilo);
    yoctogram: prefix!(yocto) / prefix!(kilo);
});
