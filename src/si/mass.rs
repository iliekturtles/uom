use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type Mass<V> = Quantity<SI<Z0, P1, Z0, Z0, Z0, Z0, Z0>, V>;

subunits!(mass; Units: Mass {
    yottagram: yotta / kilo;
    zettagram: zetta / kilo;
    exagram: exa / kilo;
    petagram: peta / kilo;
    teragram: tera / kilo;
    megagram: mega / kilo;
    kilogram: kilo / kilo;
    hectogram: hecto / kilo;
    decagram: deca / kilo;
    gram: 1.0E0 / kilo;
    decigram: deci / kilo;
    centigram: centi / kilo;
    milligram: milli / kilo;
    microgram: micro / kilo;
    nanogram: nano / kilo;
    picogram: pico / kilo;
    femtogram: femto / kilo;
    attogram: atto / kilo;
    zeptogram: zepto / kilo;
    yoctogram: yocto / kilo;
});
