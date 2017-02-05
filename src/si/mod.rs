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
        length: L,
        mass: M,
        time: T,
        electric_current: I,
        thermodynamic_temperature: Th,
        amount_of_substance: N,
        luminous_intensity: J,
    }
}
