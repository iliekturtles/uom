use typenum::{Z0, P1};
use ::{Scalar};
use ::si::{SI};
use ::si::prefix::*;

pub type LuminousIntensity<V> = Scalar<SI<Z0, Z0, Z0, Z0, Z0, Z0, P1>, V>;

subunits!(LuminousIntensitySubunits: LuminousIntensity<V> {
    candela: 1.0E0;

    yottacandela: yotta;
    zettacandela: zetta;
    exacandela: exa;
    petacandela: peta;
    teracandela: tera;
    megacandela: mega;
    kilocandela: kilo;
    hectocandela: hecto;
    decacandela: deca;
    //candela: 1.0E0;
    decicandela: deci;
    centicandela: centi;
    millicandela: milli;
    microcandela: micro;
    nanocandela: nano;
    picocandela: pico;
    femtocandela: femto;
    attocandela: atto;
    zeptocandela: zepto;
    yoctocandela: yocto;
});
