use super::{Scalar};

pub mod f32;
pub mod f64;

system!(SI:
    dimensions {
        length: L, m;
        mass: M, kg;
        time: T, s;
        electric_current: I, A;
        thermodynamic_temperature: Th, K;
        amount_of_substance: N, mol;
        luminous_intensity: J, cd;
    });

pub type Length<V> = Scalar<SI<P1, Z0, Z0, Z0, Z0, Z0, Z0>, V>;
pub type Mass<V> = Scalar<SI<Z0, P1, Z0, Z0, Z0, Z0, Z0>, V>;
pub type Time<V> = Scalar<SI<Z0, Z0, P1, Z0, Z0, Z0, Z0>, V>;
pub type ElectricCurrent<V> = Scalar<SI<Z0, Z0, Z0, P1, Z0, Z0, Z0>, V>;
pub type ThermodynamicTemperature<V> = Scalar<SI<Z0, Z0, Z0, Z0, P1, Z0, Z0>, V>;
pub type AmountOfSubstance<V> = Scalar<SI<Z0, Z0, Z0, Z0, Z0, P1, Z0>, V>;
pub type LuminousIntensity<V> = Scalar<SI<Z0, Z0, Z0, Z0, Z0, Z0, P1>, V>;

subunits!(LengthSubunits: Length<V> {
    kilogram: 1.0E3 / 1.0E3;

    yottagram: 1.0E24 / 1.0E3;
    zettagram: 1.0E21 / 1.0E3;
    exagram: 1.0E18 / 1.0E3;
    petagram: 1.0E15 / 1.0E3;
    teragram: 1.0E12 / 1.0E3;
    megagram: 1.0E9 / 1.0E3;
    //kilogram: 1.0E0;
    hectogram: 1.0E2 / 1.0E3;
});
