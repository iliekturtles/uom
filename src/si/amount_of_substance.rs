use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type AmountOfSubstance<V> = Quantity<SI<Z0, Z0, Z0, Z0, Z0, P1, Z0>, V>;

subunits!(amount_of_substance; Units: AmountOfSubstance {
    yottamole: yotta;
    zettamole: zetta;
    examole: exa;
    petamole: peta;
    teramole: tera;
    megamole: mega;
    kilomole: kilo;
    hectomole: hecto;
    decamole: deca;
    mole: 1.0E0;
    decimole: deci;
    centimole: centi;
    millimole: milli;
    micromole: micro;
    nanomole: nano;
    picomole: pico;
    femtomole: femto;
    attomole: atto;
    zeptomole: zepto;
    yoctomole: yocto;
});
