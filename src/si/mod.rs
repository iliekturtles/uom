//! [International System of Units][si] (SI) and [International System of Quantities][isq] (ISQ)
//! implementations.
//!
//! [si]: http://jcgm.bipm.org/vim/en/1.16.html
//! [isq]: http://jcgm.bipm.org/vim/en/1.6.html

// #[macro_use]
// mod prefix;

uom_macros::system! {
    /// [International System of Quantities](http://jcgm.bipm.org/vim/en/1.6.html) (ISQ).
    name: ISQ;
    /// [International System of Units](http://jcgm.bipm.org/vim/en/1.16.html) (SI).
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
    units: SI;
    base {
        /// Length, one of the base quantities in the ISQ, denoted by the symbol L. The base unit
        /// for length is meter in the SI.
        length, L;
        /// Mass, one of the base quantities in the ISQ, denoted by the symbol M. The base unit
        /// for mass is kilogram in the SI.
        mass, M;
        /// Time, one of the base quantities in the ISQ, denoted by the symbol T. The base unit
        /// for time is second in the SI.
        time, T;
        /// Electric current, one of the base quantities in the ISQ, denoted by the symbol I. The
        /// base unit for electric current is ampere in the SI.
        electric_current, I;
        /// Thermodynamic temperature, one of the base quantities in the ISQ, denoted by the symbol
        /// Th (Θ). The base unit for thermodynamic temperature is kelvin in the SI.
        thermodynamic_temperature, Th;
        /// Amount of substance, one of the base quantities in the ISQ, denoted by the symbol N.
        /// The base unit for amount of substance is mole in the SI.
        amount_of_substance, N;
        /// Luminous intensity, one of the base quantities in the ISQ, denoted by the symbol J. The
        /// base unit for luminous intensity is candela in the SI.
        luminous_intensity, J;
    }
    quantities {
        mod acceleration::Acceleration;
        mod amount_of_substance::AmountOfSubstance;
        mod angle::Angle;
        mod angular_acceleration::AngularAcceleration;
        mod angular_jerk::AngularJerk;
        mod angular_velocity::AngularVelocity;
        mod area::Area;
        mod available_energy::AvailableEnergy;
        mod capacitance::Capacitance;
        mod electric_charge::ElectricCharge;
        mod electric_current::ElectricCurrent;
        mod electric_potential::ElectricPotential;
        mod electrical_conductance::ElectricalConductance;
        mod electrical_resistance::ElectricalResistance;
        mod energy::Energy;
        mod force::Force;
        mod frequency::Frequency;
        mod inductance::Inductance;
        mod information::Information;
        mod information_rate::InformationRate;
        mod jerk::Jerk;
        mod length::Length;
        mod luminance::Luminance;
        mod luminous_intensity::LuminousIntensity;
        mod magnetic_flux::MagneticFlux;
        mod magnetic_flux_density::MagneticFluxDensity;
        mod mass::Mass;
        mod mass_density::MassDensity;
        mod mass_rate::MassRate;
        mod momentum::Momentum;
        mod power::Power;
        mod pressure::Pressure;
        mod ratio::Ratio;
        mod temperature_interval::TemperatureInterval;
        mod thermodynamic_temperature::ThermodynamicTemperature;
        mod time::Time;
        mod torque::Torque;
        mod velocity::Velocity;
        mod volume::Volume;
        mod volume_rate::VolumeRate;
    }
}

///// Primitive traits and types representing basic properties of types specific to the SI.
//pub mod marker {
//    use si::{Dimension, Quantity, Units};
//    use Kind;

//    /// AngleKind is a `Kind` for separating angular quantities from their identically dimensioned
//    /// non-angular quantity counterparts. Conversions to and from `AngleKind` quantities are
//    /// supported through implementations of the `From` trait.
//    ///
//    #[cfg_attr(feature = "f32", doc = " ```rust")]
//    #[cfg_attr(not(feature = "f32"), doc = " ```rust,ignore")]
//    /// # use uom::si::f32::*;
//    /// # use uom::si::angle::radian;
//    /// let a: Angle = Angle::new::<radian>(1.0);
//    /// let r: Ratio = a.into();
//    /// ```
//    pub trait AngleKind: ::Kind {}

//    /// InformationKind is a `Kind` for separating information quantities from their identically
//    /// dimensioned non-information quantity counterparts. Conversions to and from
//    /// `InformationKind` quantities are supported through implementations of the `From` trait.
//    ///
//    #[cfg_attr(feature = "f32", doc = " ```rust")]
//    #[cfg_attr(not(feature = "f32"), doc = " ```rust,ignore")]
//    /// # use uom::si::f32::*;
//    /// # use uom::si::information::kilobyte;
//    /// let i: Information = Information::new::<kilobyte>(1.0);
//    /// let r: Ratio = i.into();
//    /// ```
//    pub trait InformationKind: ::Kind {}

//    /// Kind of thermodynamic temperature.
//    pub trait TemperatureKind:
//        ::marker::Mul
//        + ::marker::MulAssign
//        + ::marker::Div
//        + ::marker::DivAssign
//        + ::marker::Rem
//        + ::marker::RemAssign
//    {
//    }

//    /// `impl_from` generates generic inter-Kind implementations of `From`.
//    #[cfg(feature = "autoconvert")]
//    #[macro_export]
//    macro_rules! impl_from {
//        ($a:ident, $b:ident) => {
//            impl<L, M, T, I, Th, N, J, Ul, Ur, V>
//                From<
//                    Quantity<
//                        dyn Dimension<
//                            L = L,
//                            M = M,
//                            T = T,
//                            I = I,
//                            Th = Th,
//                            N = N,
//                            J = J,
//                            Kind = dyn $a,
//                        >,
//                        Ur,
//                        V,
//                    >,
//                >
//                for Quantity<
//                    dyn Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = dyn $b>,
//                    Ul,
//                    V,
//                >
//            where
//                Ul: Units<V> + ?Sized,
//                Ur: Units<V> + ?Sized,
//                V: ::num_traits::Num + ::Conversion<V>,
//            {
//                fn from(
//                    val: Quantity<
//                        dyn Dimension<
//                            L = L,
//                            M = M,
//                            T = T,
//                            I = I,
//                            Th = Th,
//                            N = N,
//                            J = J,
//                            Kind = dyn $a,
//                        >,
//                        Ur,
//                        V,
//                    >,
//                ) -> Quantity<
//                    dyn Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = dyn $b>,
//                    Ul,
//                    V,
//                > {
//                    Self {
//                        dimension: ::lib::marker::PhantomData,
//                        units: ::lib::marker::PhantomData,
//                        value: super::change_base::<
//                            dyn Dimension<
//                                L = L,
//                                M = M,
//                                T = T,
//                                I = I,
//                                Th = Th,
//                                N = N,
//                                J = J,
//                                Kind = dyn $b,
//                            >,
//                            Ul,
//                            Ur,
//                            V,
//                        >(&val.value),
//                    }
//                }
//            }
//        };
//    }

//    /// `impl_from` generates generic inter-Kind implementations of `From`.
//    #[cfg(not(feature = "autoconvert"))]
//    #[macro_export]
//    macro_rules! impl_from {
//        ($a:ident, $b:ident) => {
//            impl<L, M, T, I, Th, N, J, U, V>
//                From<
//                    Quantity<
//                        dyn Dimension<
//                            L = L,
//                            M = M,
//                            T = T,
//                            I = I,
//                            Th = Th,
//                            N = N,
//                            J = J,
//                            Kind = dyn $a,
//                        >,
//                        U,
//                        V,
//                    >,
//                >
//                for Quantity<
//                    dyn Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = dyn $b>,
//                    U,
//                    V,
//                >
//            where
//                U: Units<V> + ?Sized,
//                V: ::num_traits::Num + ::Conversion<V>,
//            {
//                fn from(
//                    val: Quantity<
//                        dyn Dimension<
//                            L = L,
//                            M = M,
//                            T = T,
//                            I = I,
//                            Th = Th,
//                            N = N,
//                            J = J,
//                            Kind = dyn $a,
//                        >,
//                        U,
//                        V,
//                    >,
//                ) -> Quantity<
//                    dyn Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = dyn $b>,
//                    U,
//                    V,
//                > {
//                    Self {
//                        dimension: ::lib::marker::PhantomData,
//                        units: ::lib::marker::PhantomData,
//                        value: val.value,
//                    }
//                }
//            }
//        };
//    }

//    impl_from!(AngleKind, Kind);
//    impl_from!(Kind, AngleKind);
//    impl_from!(InformationKind, Kind);
//    impl_from!(Kind, InformationKind);
//}
