use typenum::{Z0, P1, N1};
use ::{Quantity};
use ::si::{SI};

pub type Dimension = SI<P1, Z0, N1, Z0, Z0, Z0, Z0>;
pub type Velocity<U, V> = Quantity<Dimension, U, V>;

units!(velocity::Velocity {
    yottameter_per_second: prefix!(yotta);
    zettameter_per_second: prefix!(zetta);
    exameter_per_second: prefix!(exa);
    petameter_per_second: prefix!(peta);
    terameter_per_second: prefix!(tera);
    megameter_per_second: prefix!(mega);
    kilometer_per_second: prefix!(kilo);
    hectometer_per_second: prefix!(hecto);
    decameter_per_second: prefix!(deca);
    meter_per_second: 1.0E0;
    decimeter_per_second: prefix!(deci);
    centimeter_per_second: prefix!(centi);
    millimeter_per_second: prefix!(milli);
    micrometer_per_second: prefix!(micro);
    nanometer_per_second: prefix!(nano);
    picometer_per_second: prefix!(pico);
    femtometer_per_second: prefix!(femto);
    attometer_per_second: prefix!(atto);
    zeptometer_per_second: prefix!(zepto);
    yoctometer_per_second: prefix!(yocto);

    foot_per_second: 3.048E-1;
});
