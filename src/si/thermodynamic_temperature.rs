//! Thermodynamic temperature (base unit kelvin, K).
//!
//! Thermodynamic temperature has the same dimensions as [temperature
//! interval](../temperature_interval/index.html) but is not directly comparable. Thermodynamic
//! temperature is the absolute measure of temperature and is one of the [base quantities][base] in
//! the [ISQ][isq]. Temperature interval is the measure of relative temperature difference between
//! thermodynamic temperatures.
//!
#![cfg_attr(feature = "f32", doc = " ```rust,compile_fail")]
#![cfg_attr(not(feature = "f32"), doc = " ```rust,ignore")]
//! # use uom::si::{temperature_interval, thermodynamic_temperature};
//! # use uom::si::f32::*;
//! let tt = ThermodynamicTemperature::new::<thermodynamic_temperature::kelvin>(1.0);
//! let ti = TemperatureInterval::new::<temperature_interval::kelvin>(1.0);
//!
//! // error[E0308]: mismatched types
//! let err = tt == ti;
//! ```
//!
//! Additionally, addition and subtraction are not implemented for thermodynamic temperature.
//!
#![cfg_attr(feature = "f32", doc = " ```rust,compile_fail")]
#![cfg_attr(not(feature = "f32"), doc = " ```rust,ignore")]
//! # use uom::si::{temperature_interval, thermodynamic_temperature};
//! # use uom::si::f32::*;
//! let t1 = ThermodynamicTemperature::new::<thermodynamic_temperature::kelvin>(1.0);
//! let t2 = ThermodynamicTemperature::new::<thermodynamic_temperature::kelvin>(1.0);
//!
//! // error[E0308]: mismatched types
//! let err = t1 + t2;
//! ```
//!
//! A temperature interval can be added to or subtracted from a thermodynamic temperature.
//!
#![cfg_attr(all(feature = "si", feature = "f32"), doc = " ```rust")]
#![cfg_attr(not(all(feature = "si", feature = "f32")), doc = " ```rust,ignore")]
//! # use uom::si::{temperature_interval, thermodynamic_temperature};
//! # use uom::si::f32::*;
//! let tt = ThermodynamicTemperature::new::<thermodynamic_temperature::kelvin>(1.0);
//! let ti = TemperatureInterval::new::<temperature_interval::kelvin>(1.0);
//!
//! let result = tt + ti;
//! ```
//!
//! [base]: http://jcgm.bipm.org/vim/en/1.4.html
//! [isq]: http://jcgm.bipm.org/vim/en/1.6.html

use si::temperature_interval::TemperatureInterval;

quantity! {
    /// Thermodynamic temperature (base unit kelvin, K).
    quantity: ThermodynamicTemperature; "thermodynamic temperature";
    /// Dimension of thermodynamic temperature, Th (base unit kelvin, K).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        P1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (::si::marker::TemperatureKind);
    units {
        @yottakelvin: prefix!(yotta); "YK", "yottakelvin", "yottakelvins";
        @zettakelvin: prefix!(zetta); "ZK", "zettakelvin", "zettakelvins";
        @exakelvin: prefix!(exa); "EK", "exakelvin", "exakelvins";
        @petakelvin: prefix!(peta); "PK", "petakelvin", "petakelvins";
        @terakelvin: prefix!(tera); "TK", "terakelvin", "terakelvins";
        @gigakelvin: prefix!(giga); "GK", "gigakelvin", "gigakelvins";
        @megakelvin: prefix!(mega); "MK", "megakelvin", "megakelvins";
        @kilokelvin: prefix!(kilo); "kK", "kilokelvin", "kilokelvins";
        @hectokelvin: prefix!(hecto); "hK", "hectokelvin", "hectokelvins";
        @decakelvin: prefix!(deca); "daK", "decakelvin", "decakelvins";
        /// The kelvin is the SI unit of thermodynamic temperature. It is defined by taking the
        /// fixed numerical value of the Boltzmann constant *k* to be 1.380 649 × 10⁻²³ when
        /// expressed in the unit J K⁻¹, which is equal to kg m² s⁻² K⁻¹, where the kilogram, meter,
        /// and second are defined in terms of *h*, *c*, and ∆*ν*<sub>Cs</sub>.
        @kelvin: prefix!(none); "K", "kelvin", "kelvins";
        @decikelvin: prefix!(deci); "dK", "decikelvin", "decikelvins";
        @centikelvin: prefix!(centi); "cK", "centikelvin", "centikelvins";
        @millikelvin: prefix!(milli); "mK", "millikelvin", "millikelvins";
        @microkelvin: prefix!(micro); "µK", "microkelvin", "microkelvins";
        @nanokelvin: prefix!(nano); "nK", "nanokelvin", "nanokelvins";
        @picokelvin: prefix!(pico); "pK", "picokelvin", "picokelvins";
        @femtokelvin: prefix!(femto); "fK", "femtokelvin", "femtokelvins";
        @attokelvin: prefix!(atto); "aK", "attokelvin", "attokelvins";
        @zeptokelvin: prefix!(zepto); "zK", "zeptokelvin", "zeptokelvins";
        @yoctokelvin: prefix!(yocto); "yK", "yoctokelvin", "yoctokelvins";

        @degree_celsius: 1.0_E0, 273.15_E0; "°C", "degree Celsius", "degrees Celsius";
        @degree_fahrenheit: 5.0_E0 / 9.0_E0, 459.67_E0; "°F", "degree Fahrenheit",
            "degrees Fahrenheit";
        @degree_rankine: 5.0_E0 / 9.0_E0; "°R", "degree Rankine", "degrees Rankine";
    }
}

#[doc(hidden)]
macro_rules! impl_ops {
    (
        $AddSubTrait:ident, $addsub_fun:ident, $addsub_op:tt,
        $AddSubAssignTrait:ident, $addsubassign_fun:ident, $addsubassign_op:tt,
        $AddSubAlias:ident
    ) => {
        #[cfg(feature = "autoconvert")]
        impl<Ul, Ur, V> $crate::lib::ops::$AddSubTrait<TemperatureInterval<Ur, V>>
            for ThermodynamicTemperature<Ul, V>
        where
            Ul: super::Units<V> + ?Sized,
            Ur: super::Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            type Output = ThermodynamicTemperature<Ul, V>;

            #[inline(always)]
            fn $addsub_fun(self, rhs: TemperatureInterval<Ur, V>) -> Self::Output {
                super::Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value
                        $addsub_op super::change_base::<Dimension, Ul, Ur, V>(&rhs.value),
                }
            }
        }

        #[cfg(not(feature = "autoconvert"))]
        impl<U, V> $crate::lib::ops::$AddSubTrait<TemperatureInterval<U, V>>
            for ThermodynamicTemperature<U, V>
        where
            U: super::Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V>,
        {
            type Output = ThermodynamicTemperature<U, V>;

            #[inline(always)]
            fn $addsub_fun(self, rhs: TemperatureInterval<U, V>) -> Self::Output {
                super::Quantity {
                    dimension: $crate::lib::marker::PhantomData,
                    units: $crate::lib::marker::PhantomData,
                    value: self.value $addsub_op rhs.value,
                }
            }
        }

        #[cfg(feature = "autoconvert")]
        impl<Ul, Ur, V> $crate::lib::ops::$AddSubAssignTrait<TemperatureInterval<Ur, V>>
            for ThermodynamicTemperature<Ul, V>
        where
            Ul: super::Units<V> + ?Sized,
            Ur: super::Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::ops::$AddSubAssignTrait<V>,
        {
            #[inline(always)]
            fn $addsubassign_fun(&mut self, rhs: TemperatureInterval<Ur, V>) {
                self.value $addsubassign_op super::change_base::<Dimension, Ul, Ur, V>(&rhs.value);
            }
        }

        #[cfg(not(feature = "autoconvert"))]
        impl<U, V> $crate::lib::ops::$AddSubAssignTrait<TemperatureInterval<U, V>>
            for ThermodynamicTemperature<U, V>
        where
            U: super::Units<V> + ?Sized,
            V: $crate::num::Num + $crate::Conversion<V> + $crate::lib::ops::$AddSubAssignTrait<V>,
        {
            #[inline(always)]
            fn $addsubassign_fun(&mut self, rhs: TemperatureInterval<U, V>) {
                self.value $addsubassign_op rhs.value;
            }
        }
    };
}

impl_ops!(Add, add, +, AddAssign, add_assign, +=, Sum);
impl_ops!(Sub, sub, -, SubAssign, sub_assign, -=, Diff);

#[cfg(test)]
mod tests {
    use si::quantities::*;
    use si::temperature_interval as ti;
    use si::thermodynamic_temperature as tt;

    storage_types! {
        use tests::*;
        use super::*;

        quickcheck! {
            #[allow(trivial_casts)]
            fn add(l: A<V>, r: A<V>) -> bool {
                Test::eq(&ThermodynamicTemperature::<V>::new::<tt::kelvin>(&*l + &*r),
                    &(ThermodynamicTemperature::<V>::new::<tt::kelvin>((*l).clone())
                        + TemperatureInterval::<V>::new::<ti::kelvin>((*r).clone())))
            }

            #[allow(trivial_casts)]
            fn sub(l: A<V>, r: A<V>) -> bool {
                Test::eq(&ThermodynamicTemperature::<V>::new::<tt::kelvin>(&*l - &*r),
                    &(ThermodynamicTemperature::<V>::new::<tt::kelvin>((*l).clone())
                        - TemperatureInterval::<V>::new::<ti::kelvin>((*r).clone())))
            }
        }
    }

    mod non_big {
        storage_types! {
            types: PrimInt, Rational, Rational32, Rational64, Float;

            use tests::*;
            use super::super::*;

            quickcheck! {
                #[allow(trivial_casts)]
                fn add_assign(l: A<V>, r: A<V>) -> bool {
                    let mut f = *l;
                    let mut v = ThermodynamicTemperature::<V>::new::<tt::kelvin>(*l);

                    f += *r;
                    v += TemperatureInterval::<V>::new::<ti::kelvin>(*r);

                    Test::approx_eq(&ThermodynamicTemperature::<V>::new::<tt::kelvin>(f), &v)
                }

                #[allow(trivial_casts)]
                fn sub_assign(l: A<V>, r: A<V>) -> bool {
                    let mut f = *l;
                    let mut v = ThermodynamicTemperature::<V>::new::<tt::kelvin>(*l);

                    f -= *r;
                    v -= TemperatureInterval::<V>::new::<ti::kelvin>(*r);

                    Test::approx_eq(&ThermodynamicTemperature::<V>::new::<tt::kelvin>(f), &v)
                }
            }
        }
    }
}
