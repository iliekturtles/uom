use typenum::{Z0, P1};
use ::{Scalar};
use ::si::{SI};
use ::si::prefix::*;

pub type AmountOfSubstanceDimensions<L, M, T, I, Th, N, J> =
    SI<L, Z0, M, Z0, T, Z0, I, Z0, Th, Z0, N, P1, J, Z0>;
pub type AmountOfSubstance<L, M, T, I, Th, N, J, V> =
    Scalar<AmountOfSubstanceDimensions<L, M, T, I, Th, N, J>, V>;

subunits!(AmountOfSubstanceSubunits: AmountOfSubstance<V> {
    mole: 1.0E0;

    yottamole: yotta;
    zettamole: zetta;
    examole: exa;
    petamole: peta;
    teramole: tera;
    megamole: mega;
    kilomole: kilo;
    hectomole: hecto;
    decamole: deca;
    //mole: 1.0E0;
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
