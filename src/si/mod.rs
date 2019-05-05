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
        /// Length, one of the base quantities in the ISQ, denoted by the symbol L. The base unit
        /// for length is meter in the SI.
        length: meter, L;
        /// Mass, one of the base quantities in the ISQ, denoted by the symbol M. The base unit
        /// for mass is kilogram in the SI.
        mass: kilogram, M;
        /// Time, one of the base quantities in the ISQ, denoted by the symbol T. The base unit
        /// for time is second in the SI.
        time: second, T;
        /// Electric current, one of the base quantities in the ISQ, denoted by the symbol I. The
        /// base unit for electric current is ampere in the SI.
        electric_current: ampere, I;
        /// Thermodynamic temperature, one of the base quantities in the ISQ, denoted by the symbol
        /// Th (Θ). The base unit for thermodynamic temperature is kelvin in the SI.
        thermodynamic_temperature: kelvin, Th;
        /// Amount of substance, one of the base quantities in the ISQ, denoted by the symbol N.
        /// The base unit for amount of substance is mole in the SI.
        amount_of_substance: mole, N;
        /// Luminous intensity, one of the base quantities in the ISQ, denoted by the symbol J. The
        /// base unit for luminous intensity is candela in the SI.
        luminous_intensity: candela, J;
    }

    /// [International System of Units](http://jcgm.bipm.org/vim/en/1.16.html) (SI).
    units: SI {
        acceleration::Acceleration,
        amount_of_substance::AmountOfSubstance,
        angle::Angle,
        area::Area,
        available_energy::AvailableEnergy,
        capacitance::Capacitance,
        electric_charge::ElectricCharge,
        electric_current::ElectricCurrent,
        electric_potential::ElectricPotential,
        electrical_conductance::ElectricalConductance,
        electrical_resistance::ElectricalResistance,
        energy::Energy,
        force::Force,
        frequency::Frequency,
        inductance::Inductance,
        jerk::Jerk,
        length::Length,
        luminance::Luminance,
        luminous_intensity::LuminousIntensity,
        magnetic_flux::MagneticFlux,
        magnetic_flux_density::MagneticFluxDensity,
        mass::Mass,
        mass_density::MassDensity,
        mass_rate::MassRate,
        momentum::Momentum,
        power::Power,
        pressure::Pressure,
        ratio::Ratio,
        temperature_interval::TemperatureInterval,
        thermodynamic_temperature::ThermodynamicTemperature,
        time::Time,
        velocity::Velocity,
        volume::Volume,
        volume_rate::VolumeRate,
    }
}

/// [`Quantity`](struct.Quantity.html) type aliases using the default base units and parameterized
/// on the underlying storage type.
pub mod quantities {
    ISQ!(si);
}

storage_types! {
    /// [`Quantity`](struct.Quantity.html) type aliases using the default base units.
    pub types: All;

    ISQ!(si, V);
}
