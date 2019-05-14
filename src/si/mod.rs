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
        angular_acceleration::AngularAcceleration,
        angular_velocity::AngularVelocity,
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
        torque::Torque,
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

/// Primitive traits and types representing basic properties of types specific to the SI.
pub mod marker {
    use si::{Dimension, Quantity, Units};
    use Kind;

    /// AngleKind is a `Kind` for separating angular quantities from their indentically dimensioned
    /// non-angular quantity counterparts. Conversions to and from `AngleKind` quantities is
    /// supported through implementations of the `From` trait.
    ///
    #[cfg_attr(feature = "f32", doc = " ```rust")]
    #[cfg_attr(not(feature = "f32"), doc = " ```rust,ignore")]
    /// # use uom::si::f32::*;
    /// # use uom::si::angle::radian;
    /// let a: Angle = Angle::new::<radian>(1.0);
    /// let r: Ratio = a.into();
    /// ```
    pub trait AngleKind: ::Kind {}

    /// Kind of thermodynamic temperature.
    pub trait TemperatureKind:
        ::marker::Mul
        + ::marker::MulAssign
        + ::marker::Div
        + ::marker::DivAssign
        + ::marker::Rem
        + ::marker::RemAssign
    {
    }

    /// `impl_from` generates generic inter-Kind implementations of `From`.
    #[cfg(feature = "autoconvert")]
    #[macro_export]
    macro_rules! impl_from {
        ($a:ident, $b:ident) => {
            impl<L, M, T, I, Th, N, J, Ul, Ur, V>
                From<
                    Quantity<
                        Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $a>,
                        Ur,
                        V,
                    >,
                >
                for Quantity<
                    Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $b>,
                    Ul,
                    V,
                >
            where
                Ul: Units<V> + ?Sized,
                Ur: Units<V> + ?Sized,
                V: ::num_traits::Num + ::Conversion<V>,
            {
                fn from(
                    val: Quantity<
                        Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $a>,
                        Ur,
                        V,
                    >,
                ) -> Quantity<
                    Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $b>,
                    Ul,
                    V,
                > {
                    Self {
                        dimension: ::lib::marker::PhantomData,
                        units: ::lib::marker::PhantomData,
                        value: super::change_base::<
                            Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $b>,
                            Ul,
                            Ur,
                            V,
                        >(&val.value),
                    }
                }
            }
        };
    }

    /// `impl_from` generates generic inter-Kind implementations of `From`.
    #[cfg(not(feature = "autoconvert"))]
    #[macro_export]
    macro_rules! impl_from {
        ($a:ident, $b:ident) => {
            impl<L, M, T, I, Th, N, J, U, V>
                From<
                    Quantity<
                        Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $a>,
                        U,
                        V,
                    >,
                >
                for Quantity<
                    Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $b>,
                    U,
                    V,
                >
            where
                U: Units<V> + ?Sized,
                V: ::num_traits::Num + ::Conversion<V>,
            {
                fn from(
                    val: Quantity<
                        Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $a>,
                        U,
                        V,
                    >,
                ) -> Quantity<
                    Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = $b>,
                    U,
                    V,
                > {
                    Self {
                        dimension: ::lib::marker::PhantomData,
                        units: ::lib::marker::PhantomData,
                        value: val.value,
                    }
                }
            }
        };
    }

    impl_from!(AngleKind, Kind);
    impl_from!(Kind, AngleKind);
}
