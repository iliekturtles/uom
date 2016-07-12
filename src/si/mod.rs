use super::{Scalar};

pub mod f32;
pub mod f64;

system!(SI:
    dimensions {
        length: L,
        mass: M,
        time: T,
        electric_current: I,
        thermodynamic_temperature: Th,
        amount_of_substance: N,
        luminous_intensity: J
    });

pub type Length<V> = Scalar<SI<P1, Z0, Z0, Z0, Z0, Z0, Z0>, V>;
pub type Mass<V> = Scalar<SI<Z0, P1, Z0, Z0, Z0, Z0, Z0>, V>;
pub type Time<V> = Scalar<SI<Z0, Z0, P1, Z0, Z0, Z0, Z0>, V>;
pub type ElectricCurrent<V> = Scalar<SI<Z0, Z0, Z0, P1, Z0, Z0, Z0>, V>;
pub type ThermodynamicTemperature<V> = Scalar<SI<Z0, Z0, Z0, Z0, P1, Z0, Z0>, V>;
pub type AmountOfSubstance<V> = Scalar<SI<Z0, Z0, Z0, Z0, Z0, P1, Z0>, V>;
pub type LuminousIntensity<V> = Scalar<SI<Z0, Z0, Z0, Z0, Z0, Z0, P1>, V>;
