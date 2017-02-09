//! International System of Units (SI) and International System of Quantities (ISQ) implementations.

#[macro_use]
mod prefix;

pub mod amount_of_substance;
pub mod electric_current;
pub mod length;
pub mod luminous_intensity;
pub mod mass;
pub mod thermodynamic_temperature;
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
    }
}
