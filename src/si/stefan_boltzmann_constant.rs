//! Stefan-Boltzmann constant (base unit watt per square meter kelvin to the fourth power, kg · s⁻³ · K⁻⁴).

quantity! {
    /// Stefan-Boltzmann constant (base unit watt per square meter kelvin to the fourth power, kg · s⁻³ · K⁻⁴).
    quantity: StefanBoltzmannConstant; "Stefan-Boltzmann constant";
    /// Dimension of Stefan-Boltzmann constant, MT⁻³Th⁻⁴  (base unit watt per square meter kelvin to the fourth power,  kg · s⁻³ · K⁻⁴).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        N3,     // time
        Z0,     // electric current
        N4,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @watt_per_square_meter_kelvin_to_the_fourth_power: prefix!(none); "W/(m²·K⁴)",
            "watt per square meter kelvin to the fourth power", "watt per square meter kelvin to the fourth power";
        @stefan_boltzmann_constant: 5.670_374_419_E-8; "σ",
            "stefan-boltzmann constant", "stefan-boltzmann constants";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::power as p;
        use crate::si::stefan_boltzmann_constant as sigma;
        use crate::si::quantities::*;
        use crate::si::thermodynamic_temperature as t;
        use crate::si::area as area;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: StefanBoltzmannConstant<V> = Power::new::<p::watt>(V::one())
                / Area::new::<area::square_meter>(V::one())
                / ThermodynamicTemperature::new::<t::kelvin>(V::one())
                / ThermodynamicTemperature::new::<t::kelvin>(V::one())
                / ThermodynamicTemperature::new::<t::kelvin>(V::one())
                / ThermodynamicTemperature::new::<t::kelvin>(V::one());
        }

        #[test]
        fn check_units() {
            test::<p::watt, area::square_meter, t::kelvin, sigma::watt_per_square_meter_kelvin_to_the_fourth_power>();

            fn test<P: p::Conversion<V>,  A: area::Conversion<V>, T: t::Conversion<V>, SIGMA: sigma::Conversion<V>>() {
                Test::assert_approx_eq(&StefanBoltzmannConstant::new::<SIGMA>(V::one()),
                    &(Power::new::<P>(V::one())
                        / Area::new::<A>(V::one())
                        / ThermodynamicTemperature::new::<T>(V::one())
                        / ThermodynamicTemperature::new::<T>(V::one())
                        / ThermodynamicTemperature::new::<T>(V::one())
                        / ThermodynamicTemperature::new::<T>(V::one())
                    ));
            }
        }
    }
}
