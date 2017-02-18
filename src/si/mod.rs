//! International System of Units (SI) and International System of Quantities (ISQ) implementations.

#[macro_use]
mod prefix;

#[macro_use]
pub mod amount_of_substance;
#[macro_use]
pub mod electric_current;
#[macro_use]
pub mod length;
#[macro_use]
pub mod luminous_intensity;
#[macro_use]
pub mod mass;
#[macro_use]
pub mod thermodynamic_temperature;
#[macro_use]
pub mod time;

system! {
    /// International System of Quantities (ISQ).
    quantities: ISQ {
        length: meter, L;
        mass: kilogram, M;
        time: second, T;
        electric_current: ampere, I;
        thermodynamic_temperature: kelvin, Th;
        amount_of_substance: mole, N;
        luminous_intensity: candela, J;
    }

    /// International System of Units (SI).
    units: SI {
        amount_of_substance::AmountOfSubstance,
        electric_current::ElectricCurrent,
        length::Length,
        luminous_intensity::LuminousIntensity,
        mass::Mass,
        thermodynamic_temperature::ThermodynamicTemperature,
        time::Time,
    }
}

/// Quantity type aliases using `f32` as the underlying storage type.
pub mod f32 {
    use super::{ISQ, SI};

    ISQ!(SI<f32>, f32, si);
}

/// Quantity type aliases using `f64` as the underlying storage type.
pub mod f64 {
    use super::{ISQ, SI};

    ISQ!(SI<f64>, f64, si);
}
