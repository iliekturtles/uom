use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type LuminousIntensity<V> = Quantity<SI<Z0, Z0, Z0, Z0, Z0, Z0, P1>, V>;

subunits!(luminous_intensity; Units: LuminousIntensity {
    yottacandela: yotta;
    zettacandela: zetta;
    exacandela: exa;
    petacandela: peta;
    teracandela: tera;
    megacandela: mega;
    kilocandela: kilo;
    hectocandela: hecto;
    decacandela: deca;
    candela: 1.0E0;
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
