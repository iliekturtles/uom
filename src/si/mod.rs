//! [International System of Units][si] (SI) and [International System of Quantities][isq] (ISQ)
//! implementations.
//!
//! [si]: http://jcgm.bipm.org/vim/en/1.16.html
//! [isq]: http://jcgm.bipm.org/vim/en/1.6.html

#[macro_use]
mod prefix;

system! {
    /// [International System of Quantities](http://jcgm.bipm.org/vim/en/1.6.html) (ISQ).
    quantities: ISQ {
        length: meter, L;
        mass: kilogram, M;
        time: second, T;
        electric_current: ampere, I;
        thermodynamic_temperature: kelvin, Th;
        amount_of_substance: mole, N;
        luminous_intensity: candela, J;
    }

    /// [International System of Units](http://jcgm.bipm.org/vim/en/1.16.html) (SI).
    units: SI {
        acceleration::Acceleration,
        amount_of_substance::AmountOfSubstance,
        area::Area,
        electric_current::ElectricCurrent,
        length::Length,
        luminous_intensity::LuminousIntensity,
        mass::Mass,
        thermodynamic_temperature::ThermodynamicTemperature,
        time::Time,
        velocity::Velocity,
        volume::Volume,
    }
}

/// [`Quantity`](struct.Quantity.html) type aliases using `f32` as the underlying
/// storage type.
#[cfg(feature = "f32")]
pub mod f32 {
    ISQ!(si, f32);
}

/// [`Quantity`](struct.Quantity.html) type aliases using `f64` as the underlying
/// storage type.
#[cfg(feature = "f64")]
pub mod f64 {
    ISQ!(si, f64);
}
