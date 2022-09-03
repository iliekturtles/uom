//! [International System of Units][si] (SI) and [International System of Quantities][isq] (ISQ)
//! implementations.
//!
//! [si]: https://jcgm.bipm.org/vim/en/1.16.html
//! [isq]: https://jcgm.bipm.org/vim/en/1.6.html

#[macro_use]
mod prefix;

system! {
    /// [International System of Quantities](https://jcgm.bipm.org/vim/en/1.6.html) (ISQ).
    ///
    /// ## Generic Parameters
    /// * `L`: Length dimension.
    /// * `M`: Mass dimension.
    /// * `T`: Time dimension.
    /// * `I`: Electric current dimension.
    /// * `Th`: Thermodynamic temperature dimension.
    /// * `N`: Amount of substance dimension.
    /// * `J`: Luminous intensity dimension.
    /// * `K`: Kind.
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

    /// [International System of Units](https://jcgm.bipm.org/vim/en/1.16.html) (SI).
    units: SI {
        absement::Absement,
        acceleration::Acceleration,
        amount_of_substance::AmountOfSubstance,
        angle::Angle,
        angular_acceleration::AngularAcceleration,
        angular_jerk::AngularJerk,
        angular_velocity::AngularVelocity,
        area::Area,
        areal_mass_density::ArealMassDensity,
        available_energy::AvailableEnergy,
        capacitance::Capacitance,
        catalytic_activity::CatalyticActivity,
        catalytic_activity_concentration::CatalyticActivityConcentration,
        curvature::Curvature,
        diffusion_coefficient::DiffusionCoefficient,
        dynamic_viscosity::DynamicViscosity,
        electric_charge::ElectricCharge,
        electric_charge_areal_density::ElectricChargeArealDensity,
        electric_charge_linear_density::ElectricChargeLinearDensity,
        electric_charge_volumetric_density::ElectricChargeVolumetricDensity,
        electric_current::ElectricCurrent,
        electric_displacement_field::ElectricDisplacementField,
        electric_permittivity::ElectricPermittivity,
        electric_potential::ElectricPotential,
        electrical_conductance::ElectricalConductance,
        electrical_conductivity::ElectricalConductivity,
        electrical_resistance::ElectricalResistance,
        electrical_resistivity::ElectricalResistivity,
        energy::Energy,
        force::Force,
        frequency::Frequency,
        heat_capacity::HeatCapacity,
        heat_flux_density::HeatFluxDensity,
        heat_transfer::HeatTransfer,
        inductance::Inductance,
        information::Information,
        information_rate::InformationRate,
        jerk::Jerk,
        length::Length,
        linear_mass_density::LinearMassDensity,
        linear_power_density::LinearPowerDensity,
        luminance::Luminance,
        luminous_intensity::LuminousIntensity,
        magnetic_field_strength::MagneticFieldStrength,
        magnetic_flux::MagneticFlux,
        magnetic_flux_density::MagneticFluxDensity,
        magnetic_permeability::MagneticPermeability,
        mass::Mass,
        mass_concentration::MassConcentration,
        mass_density::MassDensity,
        mass_flux::MassFlux,
        mass_rate::MassRate,
        molar_concentration::MolarConcentration,
        molar_energy::MolarEnergy,
        molar_flux::MolarFlux,
        molar_heat_capacity::MolarHeatCapacity,
        molar_mass::MolarMass,
        molar_volume::MolarVolume,
        momentum::Momentum,
        power::Power,
        pressure::Pressure,
        radiant_exposure::RadiantExposure,
        ratio::Ratio,
        solid_angle::SolidAngle,
        specific_heat_capacity::SpecificHeatCapacity,
        surface_electric_current_density::SurfaceElectricCurrentDensity,
        temperature_coefficient::TemperatureCoefficient,
        temperature_interval::TemperatureInterval,
        thermal_conductivity::ThermalConductivity,
        thermodynamic_temperature::ThermodynamicTemperature,
        time::Time,
        torque::Torque,
        velocity::Velocity,
        volume::Volume,
        volume_rate::VolumeRate,
        volumetric_power_density::VolumetricPowerDensity,
    }
}

/// [`Quantity`](struct.Quantity.html) type aliases using the default base units and parameterized
/// on the underlying storage type.
pub mod quantities {
    ISQ!(crate::si);
}

storage_types! {
    /// [`Quantity`](struct.Quantity.html) type aliases using the default base units.
    pub types: All;

    ISQ!(crate::si, V);
}

/// Primitive traits and types representing basic properties of types specific to the SI.
pub mod marker {
    use super::{Dimension, Quantity, Units};
    use crate::Kind;

    /// `AngleKind` is a `Kind` for separating angular quantities from their identically dimensioned
    /// non-angular quantity counterparts. Conversions to and from `AngleKind` quantities are
    /// supported through implementations of the `From` trait.
    ///
    #[cfg_attr(feature = "f32", doc = " ```rust")]
    #[cfg_attr(not(feature = "f32"), doc = " ```rust,ignore")]
    /// # use uom::si::f32::*;
    /// # use uom::si::angle::radian;
    /// let a: Angle = Angle::new::<radian>(1.0);
    /// let r: Ratio = a.into();
    /// ```
    pub trait AngleKind: Kind {}

    /// `SolidAngleKind` is a `Kind` for separating quantities of solid angles from other
    /// identically dimensioned quantities. Conversions to and from `SolidAngleKind` quantities are
    /// supported through implementations of the `From` trait.
    ///
    #[cfg_attr(feature = "f32", doc = " ```rust")]
    #[cfg_attr(not(feature = "f32"), doc = " ```rust,ignore")]
    /// # use uom::si::f32::*;
    /// # use uom::si::solid_angle::steradian;
    /// let a: SolidAngle = SolidAngle::new::<steradian>(1.0);
    /// let r: Ratio = a.into();
    /// ```
    pub trait SolidAngleKind: Kind {}

    /// `InformationKind` is a `Kind` for separating information quantities from their identically
    /// dimensioned non-information quantity counterparts. Conversions to and from `InformationKind`
    /// quantities are supported through implementations of the `From` trait.
    ///
    #[cfg_attr(feature = "f32", doc = " ```rust")]
    #[cfg_attr(not(feature = "f32"), doc = " ```rust,ignore")]
    /// # use uom::si::f32::*;
    /// # use uom::si::information::kilobyte;
    /// let i: Information = Information::new::<kilobyte>(1.0);
    /// let r: Ratio = i.into();
    /// ```
    pub trait InformationKind: Kind {}

    /// Kind of thermodynamic temperature.
    pub trait TemperatureKind:
        crate::marker::Mul
        + crate::marker::MulAssign
        + crate::marker::Div
        + crate::marker::DivAssign
        + crate::marker::Rem
        + crate::marker::RemAssign
    {
    }

    /// Kind of constituent concentration in chemical mixtures, which separates mass concentration
    /// from mass density. This `Kind` is also applied to molar concentration and to catalytic
    /// activity concentration.
    pub trait ConstituentConcentrationKind: Kind {}

    /// `impl_from` generates generic inter-Kind implementations of `From`.
    #[cfg(feature = "autoconvert")]
    #[macro_export]
    macro_rules! impl_from {
        ($a:ident, $b:ident) => {
            impl<L, M, T, I, Th, N, J, Ul, Ur, V>
                From<
                    Quantity<
                        dyn Dimension<
                            L = L,
                            M = M,
                            T = T,
                            I = I,
                            Th = Th,
                            N = N,
                            J = J,
                            Kind = dyn $a,
                        >,
                        Ur,
                        V,
                    >,
                >
                for Quantity<
                    dyn Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = dyn $b>,
                    Ul,
                    V,
                >
            where
                L: $crate::typenum::Integer,
                M: $crate::typenum::Integer,
                T: $crate::typenum::Integer,
                I: $crate::typenum::Integer,
                Th: $crate::typenum::Integer,
                N: $crate::typenum::Integer,
                J: $crate::typenum::Integer,
                Ul: Units<V> + ?Sized,
                Ur: Units<V> + ?Sized,
                V: $crate::num_traits::Num + $crate::Conversion<V>,
            {
                fn from(
                    val: Quantity<
                        dyn Dimension<
                            L = L,
                            M = M,
                            T = T,
                            I = I,
                            Th = Th,
                            N = N,
                            J = J,
                            Kind = dyn $a,
                        >,
                        Ur,
                        V,
                    >,
                ) -> Quantity<
                    dyn Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = dyn $b>,
                    Ul,
                    V,
                > {
                    Self {
                        dimension: $crate::lib::marker::PhantomData,
                        units: $crate::lib::marker::PhantomData,
                        value: super::change_base::<
                            dyn Dimension<
                                L = L,
                                M = M,
                                T = T,
                                I = I,
                                Th = Th,
                                N = N,
                                J = J,
                                Kind = dyn $b,
                            >,
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
                        dyn Dimension<
                            L = L,
                            M = M,
                            T = T,
                            I = I,
                            Th = Th,
                            N = N,
                            J = J,
                            Kind = dyn $a,
                        >,
                        U,
                        V,
                    >,
                >
                for Quantity<
                    dyn Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = dyn $b>,
                    U,
                    V,
                >
            where
                L: $crate::typenum::Integer,
                M: $crate::typenum::Integer,
                T: $crate::typenum::Integer,
                I: $crate::typenum::Integer,
                Th: $crate::typenum::Integer,
                N: $crate::typenum::Integer,
                J: $crate::typenum::Integer,
                U: Units<V> + ?Sized,
                V: $crate::num_traits::Num + $crate::Conversion<V>,
            {
                fn from(
                    val: Quantity<
                        dyn Dimension<
                            L = L,
                            M = M,
                            T = T,
                            I = I,
                            Th = Th,
                            N = N,
                            J = J,
                            Kind = dyn $a,
                        >,
                        U,
                        V,
                    >,
                ) -> Quantity<
                    dyn Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = dyn $b>,
                    U,
                    V,
                > {
                    Self {
                        dimension: $crate::lib::marker::PhantomData,
                        units: $crate::lib::marker::PhantomData,
                        value: val.value,
                    }
                }
            }
        };
    }

    impl_from!(AngleKind, Kind);
    impl_from!(Kind, AngleKind);
    impl_from!(SolidAngleKind, Kind);
    impl_from!(Kind, SolidAngleKind);
    impl_from!(InformationKind, Kind);
    impl_from!(Kind, InformationKind);
    impl_from!(ConstituentConcentrationKind, Kind);
    impl_from!(Kind, ConstituentConcentrationKind);
}
