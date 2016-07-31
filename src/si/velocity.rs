use typenum::{Z0, P1, N1};
use ::{Quantity};
use ::si::{SI};
use ::si::prefix::*;

pub type Velocity<V> = Quantity<SI<P1, Z0, N1, Z0, Z0, Z0, Z0>, V>;

subunits!(VelocitySubunits: Velocity<V> {
    meter_per_second: 1.0E0;

    yottameter_per_second: yotta;
    zettameter_per_second: zetta;
    exameter_per_second: exa;
    petameter_per_second: peta;
    terameter_per_second: tera;
    megameter_per_second: mega;
    kilometer_per_second: kilo;
    hectometer_per_second: hecto;
    decameter_per_second: deca;
    //meter_per_second: 1.0E0;
    decimeter_per_second: deci;
    centimeter_per_second: centi;
    millimeter_per_second: milli;
    micrometer_per_second: micro;
    nanometer_per_second: nano;
    picometer_per_second: pico;
    femtometer_per_second: femto;
    attometer_per_second: atto;
    zeptometer_per_second: zepto;
    yoctometer_per_second: yocto;

    foot_per_second: 3.048E-1;
});
