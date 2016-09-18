use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type AmountOfSubstance<U, V> = Quantity<SI<Z0, Z0, Z0, Z0, Z0, P1, Z0>, U, V>;

units!(amount_of_substance AmountOfSubstance {
    yottamole: prefix!(yotta);
    zettamole: prefix!(zetta);
    examole: prefix!(exa);
    petamole: prefix!(peta);
    teramole: prefix!(tera);
    megamole: prefix!(mega);
    kilomole: prefix!(kilo);
    hectomole: prefix!(hecto);
    decamole: prefix!(deca);
    mole: 1.0E0;
    decimole: prefix!(deci);
    centimole: prefix!(centi);
    millimole: prefix!(milli);
    micromole: prefix!(micro);
    nanomole: prefix!(nano);
    picomole: prefix!(pico);
    femtomole: prefix!(femto);
    attomole: prefix!(atto);
    zeptomole: prefix!(zepto);
    yoctomole: prefix!(yocto);
});
