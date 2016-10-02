use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type Dimension = SI<Z0, Z0, Z0, Z0, Z0, Z0, P1>;
pub type LuminousIntensity<U, V> = Quantity<Dimension, U, V>;

units!(luminous_intensity::LuminousIntensity {
    yottacandela: prefix!(yotta);
    zettacandela: prefix!(zetta);
    exacandela: prefix!(exa);
    petacandela: prefix!(peta);
    teracandela: prefix!(tera);
    megacandela: prefix!(mega);
    kilocandela: prefix!(kilo);
    hectocandela: prefix!(hecto);
    decacandela: prefix!(deca);
    candela: 1.0E0;
    decicandela: prefix!(deci);
    centicandela: prefix!(centi);
    millicandela: prefix!(milli);
    microcandela: prefix!(micro);
    nanocandela: prefix!(nano);
    picocandela: prefix!(pico);
    femtocandela: prefix!(femto);
    attocandela: prefix!(atto);
    zeptocandela: prefix!(zepto);
    yoctocandela: prefix!(yocto);
});
