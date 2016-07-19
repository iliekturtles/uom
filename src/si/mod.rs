mod units;

pub mod prefix;

use ::{Scalar};

pub use self::units::*;

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

pub type Time<V> = Scalar<SI<Z0, Z0, P1, Z0, Z0, Z0, Z0>, V>;
pub type ElectricCurrent<V> = Scalar<SI<Z0, Z0, Z0, P1, Z0, Z0, Z0>, V>;
pub type ThermodynamicTemperature<V> = Scalar<SI<Z0, Z0, Z0, Z0, P1, Z0, Z0>, V>;
pub type AmountOfSubstance<V> = Scalar<SI<Z0, Z0, Z0, Z0, Z0, P1, Z0>, V>;
pub type LuminousIntensity<V> = Scalar<SI<Z0, Z0, Z0, Z0, Z0, Z0, P1>, V>;
