//! Temperature interval (base unit kelvin, K).
//!
//! Temperature interval has the same dimensions as [thermodynamic temperature][tt] but is not
//! directly comparable. See [thermodynamic temperature][tt] for a full explanation.
//!
//! [tt]: ../thermodynamic_temperature/index.html

use si::thermodynamic_temperature::ThermodynamicTemperature;

quantity! {
    /// Temperature interval (base unit kelvin, K).
    quantity: TemperatureInterval; "temperature interval";
    /// Dimension of temperature interval, Th (base unit kelvin, K).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        P1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
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

        @degree_celsius: 1.0_E0; "°C", "degree Celsius", "degrees Celsius";
        @degree_fahrenheit: 5.0_E0 / 9.0_E0; "°F", "degree Fahrenheit", "degrees Fahrenheit";
        @degree_rankine: 5.0_E0 / 9.0_E0; "°R", "degree Rankine", "degrees Rankine";
    }
}

#[cfg(feature = "autoconvert")]
impl<Ul, Ur, V> ::lib::ops::Add<ThermodynamicTemperature<Ur, V>> for TemperatureInterval<Ul, V>
where
    Ul: super::Units<V> + ?Sized,
    Ur: super::Units<V> + ?Sized,
    V: ::num::Num + ::Conversion<V>,
{
    type Output = ThermodynamicTemperature<Ul, V>;

    #[inline(always)]
    fn add(self, rhs: ThermodynamicTemperature<Ur, V>) -> Self::Output {
        super::Quantity {
            dimension: ::lib::marker::PhantomData,
            units: ::lib::marker::PhantomData,
            value: self.value + super::change_base::<Dimension, Ul, Ur, V>(&rhs.value),
        }
    }
}

#[cfg(not(feature = "autoconvert"))]
impl<U, V> ::lib::ops::Add<ThermodynamicTemperature<U, V>> for TemperatureInterval<U, V>
where
    U: super::Units<V> + ?Sized,
    V: ::num::Num + ::Conversion<V>,
{
    type Output = ThermodynamicTemperature<U, V>;

    #[inline(always)]
    fn add(self, rhs: ThermodynamicTemperature<U, V>) -> Self::Output {
        super::Quantity {
            dimension: ::lib::marker::PhantomData,
            units: ::lib::marker::PhantomData,
            value: self.value + rhs.value,
        }
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use si::quantities::*;
        use si::temperature_interval as ti;
        use si::thermodynamic_temperature as tt;
        use tests::{A, Test};

        quickcheck! {
            #[allow(trivial_casts)]
            fn add(l: A<V>, r: A<V>) -> bool {
                Test::eq(&ThermodynamicTemperature::<V>::new::<tt::kelvin>(&*l + &*r),
                    &(TemperatureInterval::<V>::new::<ti::kelvin>((*l).clone())
                        + ThermodynamicTemperature::<V>::new::<tt::kelvin>((*r).clone())))
            }
        }
    }
}
