use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type Length<V> = Quantity<SI<P1, Z0, Z0, Z0, Z0, Z0, Z0>, V>;

subunits!(length; Units: Length {
    yottameter: prefix!(yotta);
    zettameter: prefix!(zetta);
    exameter: prefix!(exa);
    petameter: prefix!(peta);
    terameter: prefix!(tera);
    megameter: prefix!(mega);
    kilometer: prefix!(kilo);
    hectometer: prefix!(hecto);
    decameter: prefix!(deca);
    meter: 1.0E0;
    decimeter: prefix!(deci);
    centimeter: prefix!(centi);
    millimeter: prefix!(milli);
    micrometer: prefix!(micro);
    nanometer: prefix!(nano);
    picometer: prefix!(pico);
    femtometer: prefix!(femto);
    attometer: prefix!(atto);
    zeptometer: prefix!(zepto);
    yoctometer: prefix!(yocto);

    yard: 9.144E-1;
});
