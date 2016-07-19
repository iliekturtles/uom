// TODO: Needed?
#![allow(dead_code)]

use typenum::{Z0, P1};
use ::{Scalar};
use ::si::{SI};
use ::si::prefix::*;

pub type Length<V> = Scalar<SI<P1, Z0, Z0, Z0, Z0, Z0, Z0>, V>;

pub mod f32 { pub type Length = super::Length<f32>; }
pub mod f64 { pub type Length = super::Length<f64>; }

subunits!(LengthSubunits: Length<V> {
    kilogram: kilo / kilo;

    yottagram: yotta / kilo;
    zettagram: zetta / kilo;
    exagram: exa / kilo;
    petagram: peta / kilo;
    teragram: tera / kilo;
    megagram: mega / kilo;
    //kilogram: kilo / kilo;
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
