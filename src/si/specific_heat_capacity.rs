//! Specific heat capacity (base unit joule per kilogram kelvin, m² · s⁻² · K⁻¹).
//!
//! This quantity might be used to define the heat capacity of a material. To define heat capacity
//! of an object, use [heat capacity][hc].
//!
//! Specific heat capacity has the same kind as [temperature interval][ti], as this quantity
//! relates to change of temperature. Not of kind `TemperatureKind`, used by [thermodynamic
//! temperature][tt]. See [thermodynamic temperature][tt] for a full explanation.
//!
//! [ti]: ../temperature_interval/index.html
//! [tt]: ../thermodynamic_temperature/index.html
//! [hc]: ../heat_capacity/index.html

quantity! {
    /// Specific heat capacity (base unit joule per kilogram kelvin, m² · s⁻² · K⁻¹).
    quantity: SpecificHeatCapacity; "specific heat capacity";
    /// Dimension of specific heat capacity, L²T⁻²Th⁻¹ (base unit joule per kilogram kelvin, m² ·
    /// s⁻² · K⁻¹).
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        N1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @square_kilometer_per_second_squared_kelvin: prefix!(kilo) * prefix!(kilo); "km²/(s² · K)",
            "square kilometer per second squared kelvin",
            "square kilometers per second squared kelvin";
        /// The derived unit of specific heat capacity expressed in base units. Equivalent to
        /// J/(kg · K).
        @square_meter_per_second_squared_kelvin: prefix!(none); "m²/(s² · K)",
            "square meter per second squared kelvin", "square meters per second squared kelvin";
        @square_centimeter_per_second_squared_kelvin: prefix!(centi) * prefix!(centi);
            "cm²/(s² · K)", "square centimeter per second squared kelvin",
            "square centimeters per second squared kelvin";
        @square_millimeter_per_second_squared_kelvin: prefix!(milli) * prefix!(milli);
            "mm²/(s² · K)", "square millimeter per second squared kelvin",
            "square millimeters per second squared kelvin";
        @square_micrometer_per_second_squared_kelvin: prefix!(micro) * prefix!(micro);
            "µm²/(s² · K)", "square micrometer per second squared kelvin",
            "square micrometers per second squared kelvin";

        // Specific heat capacity is much more commonly expressed in terms of energy / (mass
        // temperature).
        @yottajoule_per_kilogram_kelvin: prefix!(yotta); "YJ/(kg · K)",
            "yottajoule per kilogram kelvin", "yottajoules per kilogram kelvin";
        @zettajoule_per_kilogram_kelvin: prefix!(zetta); "ZJ/(kg · K)",
            "zettajoule per kilogram kelvin", "zettajoules per kilogram kelvin";
        @exajoule_per_kilogram_kelvin: prefix!(exa); "EJ/(kg · K)",
            "exajoule per kilogram kelvin", "exajoules per kilogram kelvin";
        @petajoule_per_kilogram_kelvin: prefix!(peta); "PJ/(kg · K)",
            "petajoule per kilogram kelvin", "petajoules per kilogram kelvin";
        @terajoule_per_kilogram_kelvin: prefix!(tera); "TJ/(kg · K)",
            "terajoule per kilogram kelvin", "terajoules per kilogram kelvin";
        @gigajoule_per_kilogram_kelvin: prefix!(giga); "GJ/(kg · K)",
            "gigajoule per kilogram kelvin", "gigajoules per kilogram kelvin";
        @megajoule_per_kilogram_kelvin: prefix!(mega); "MJ/(kg · K)",
            "megajoule per kilogram kelvin", "megajoules per kilogram kelvin";
        @kilojoule_per_kilogram_kelvin: prefix!(kilo); "kJ/(kg · K)",
            "kilojoule per kilogram kelvin", "kilojoules per kilogram kelvin";
        @hectojoule_per_kilogram_kelvin: prefix!(hecto); "hJ/(kg · K)",
            "hectojoule per kilogram kelvin", "hectojoules per kilogram kelvin";
        @decajoule_per_kilogram_kelvin: prefix!(deca); "daJ/(kg · K)",
            "decajoule per kilogram kelvin", "decajoules per kilogram kelvin";
        /// Derived unit of specific heat capacity expressed in derived units. Equivalent to
        /// m²/(s² · K).
        @joule_per_kilogram_kelvin: prefix!(none); "J/(kg · K)",
            "joule per kilogram kelvin", "joules per kilogram kelvin";
        @decijoule_per_kilogram_kelvin: prefix!(deci); "dJ/(kg · K)",
            "decijoule per kilogram kelvin", "decijoules per kilogram kelvin";
        @centijoule_per_kilogram_kelvin: prefix!(centi); "cJ/(kg · K)",
            "centijoule per kilogram kelvin", "centijoules per kilogram kelvin";
        @millijoule_per_kilogram_kelvin: prefix!(milli); "mJ/(kg · K)",
            "millijoule per kilogram kelvin", "millijoules per kilogram kelvin";
        @microjoule_per_kilogram_kelvin: prefix!(micro); "µJ/(kg · K)",
            "microjoule per kilogram kelvin", "microjoules per kilogram kelvin";
        @nanojoule_per_kilogram_kelvin: prefix!(nano); "nJ/(kg · K)",
            "nanojoule per kilogram kelvin", "nanojoules per kilogram kelvin";
        @picojoule_per_kilogram_kelvin: prefix!(pico); "pJ/(kg · K)",
            "picojoule per kilogram kelvin", "picojoules per kilogram kelvin";
        @femtojoule_per_kilogram_kelvin: prefix!(femto); "fJ/(kg · K)",
            "femtojoule per kilogram kelvin", "femtojoules per kilogram kelvin";
        @attojoule_per_kilogram_kelvin: prefix!(atto); "aJ/(kg · K)",
            "attojoule per kilogram kelvin", "attojoules per kilogram kelvin";
        @zeptojoule_per_kilogram_kelvin: prefix!(zepto); "zJ/(kg · K)",
            "zeptojoule per kilogram kelvin", "zeptojoules per kilogram kelvin";
        @yoctojoule_per_kilogram_kelvin: prefix!(yocto); "yJ/(kg · K)",
            "yoctojoule per kilogram kelvin", "yoctojoules per kilogram kelvin";

        @kilojoule_per_kilogram_degree_celsius: 1.0_E3; "kJ/(kg · °C)",
            "kilojoule per kilogram degree celsius", "kilojoules per kilogram degree celsius";
        @kilojoule_per_gram_degree_celsius: 1.0_E6; "kJ/(g · °C)",
            "kilojoule per gram degree celsius", "kilojoules per gram degree celsius";
        @joule_per_kilogram_degree_celsius: 1.0_E0; "J/(kg · °C)",
            "joule per kilogram degree celsius", "joules per kilogram degree celsius";
        @joule_per_gram_degree_celsius: 1.0_E3; "J/(g · °C)",
            "joule per gram degree celsius", "joules per gram degree celsius";
        @millijoule_per_kilogram_degree_celsius: 1.0_E-3; "mJ/(kg · °C)",
            "millijoule per kilogram degree celsius", "millijoules per kilogram degree celsius";
        @millijoule_per_gram_degree_celsius: 1.0_E0; "mJ/(g · °C)",
            "millijoule per gram degree celsius", "millijoules per gram degree celsius";

        @btu_per_ounce_degree_fahrenheit: 6.694_399_058_608_398_E4; "Btu/(oz · °F)",
            "British thermal unit per ounce degree Fahrenheit",
            "British thermal units per ounce degree Fahrenheit";
        @btu_it_per_ounce_degree_fahrenheit: 6.698_881_674_187_076_E4; "Btu (IT)/(oz · °F)",
            "British thermal unit (IT) per ounce degree Fahrenheit",
            "British thermal units (IT) per ounce degree Fahrenheit";
        @btu_per_pound_degree_fahrenheit: 4.183_998_673_699_118_E3; "Btu/(lb · °F)",
            "British thermal unit per pound degree Fahrenheit",
            "British thermal units per pound degree Fahrenheit";
        @btu_it_per_pound_degree_fahrenheit: 4.186_800_307_941_667_E3; "Btu (IT)/(lb · °F)",
            "British thermal unit (IT) per pound degree Fahrenheit",
            "British thermal units (IT) per pound degree Fahrenheit";
        @btu_per_ton_degree_fahrenheit: 1.897_829_999_999_999_999_E0; "Btu/(t · °F)",
            "British thermal unit per ton degree Fahrenheit",
            "British thermal units per ton degree Fahrenheit";
        @btu_it_per_ton_degree_fahrenheit: 1.899_100_799_999_999_999_E0; "Btu (IT)/(t · °F)",
            "British thermal unit (IT) per ton degree Fahrenheit",
            "British thermal units (IT) per ton degree Fahrenheit";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::mass as m;
        use si::time as t;
        use si::temperature_interval as ti;
        use si::energy as e;
        use si::length as l;
        use si::specific_heat_capacity as shc;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: SpecificHeatCapacity<V> = Length::new::<l::meter>(V::one())
                * Length::new::<l::meter>(V::one())
                / (Time::new::<t::second>(V::one())
                    * Time::new::<t::second>(V::one())
                    * TemperatureInterval::new::<ti::kelvin>(V::one()));
        }

        #[test]
        fn check_base_units() {
            test::<l::kilometer, t::second, ti::kelvin,
                shc::square_kilometer_per_second_squared_kelvin>();
            test::<l::meter, t::second, ti::kelvin,
                shc::square_meter_per_second_squared_kelvin>();
            test::<l::centimeter, t::second, ti::kelvin,
                shc::square_centimeter_per_second_squared_kelvin>();
            test::<l::millimeter, t::second, ti::kelvin,
                shc::square_millimeter_per_second_squared_kelvin>();
            test::<l::micrometer, t::second, ti::kelvin,
                shc::square_micrometer_per_second_squared_kelvin>();

            fn test<
                L: l::Conversion<V>,
                T: t::Conversion<V>,
                TI: ti::Conversion<V>,
                SHC: shc::Conversion<V>>()
            {
                Test::assert_approx_eq(&SpecificHeatCapacity::new::<SHC>(V::one()),
                    &((Length::new::<L>(V::one()) * Length::new::<L>(V::one()))
                        / (Time::new::<T>(V::one())
                            * Time::new::<T>(V::one())
                            * TemperatureInterval::new::<TI>(V::one()))));
            }
        }

        #[test]
        fn check_energy_per_mass_ti_shc_units() {
            test::<e::yottajoule, m::kilogram, ti::kelvin, shc::yottajoule_per_kilogram_kelvin>();
            test::<e::zettajoule, m::kilogram, ti::kelvin, shc::zettajoule_per_kilogram_kelvin>();
            test::<e::exajoule, m::kilogram, ti::kelvin, shc::exajoule_per_kilogram_kelvin>();
            test::<e::petajoule, m::kilogram, ti::kelvin, shc::petajoule_per_kilogram_kelvin>();
            test::<e::terajoule, m::kilogram, ti::kelvin, shc::terajoule_per_kilogram_kelvin>();
            test::<e::gigajoule, m::kilogram, ti::kelvin, shc::gigajoule_per_kilogram_kelvin>();
            test::<e::megajoule, m::kilogram, ti::kelvin, shc::megajoule_per_kilogram_kelvin>();
            test::<e::kilojoule, m::kilogram, ti::kelvin, shc::kilojoule_per_kilogram_kelvin>();
            test::<e::hectojoule, m::kilogram, ti::kelvin, shc::hectojoule_per_kilogram_kelvin>();
            test::<e::decajoule, m::kilogram, ti::kelvin, shc::decajoule_per_kilogram_kelvin>();
            test::<e::joule, m::kilogram, ti::kelvin, shc::joule_per_kilogram_kelvin>();
            test::<e::decijoule, m::kilogram, ti::kelvin, shc::decijoule_per_kilogram_kelvin>();
            test::<e::centijoule, m::kilogram, ti::kelvin, shc::centijoule_per_kilogram_kelvin>();
            test::<e::millijoule, m::kilogram, ti::kelvin, shc::millijoule_per_kilogram_kelvin>();
            test::<e::microjoule, m::kilogram, ti::kelvin, shc::microjoule_per_kilogram_kelvin>();
            test::<e::nanojoule, m::kilogram, ti::kelvin, shc::nanojoule_per_kilogram_kelvin>();
            test::<e::picojoule, m::kilogram, ti::kelvin, shc::picojoule_per_kilogram_kelvin>();
            test::<e::femtojoule, m::kilogram, ti::kelvin, shc::femtojoule_per_kilogram_kelvin>();
            test::<e::attojoule, m::kilogram, ti::kelvin, shc::attojoule_per_kilogram_kelvin>();
            test::<e::zeptojoule, m::kilogram, ti::kelvin, shc::zeptojoule_per_kilogram_kelvin>();
            test::<e::yoctojoule, m::kilogram, ti::kelvin, shc::yoctojoule_per_kilogram_kelvin>();

            test::<e::kilojoule, m::kilogram, ti::degree_celsius,
                shc::kilojoule_per_kilogram_degree_celsius>();
            test::<e::kilojoule, m::gram, ti::degree_celsius,
                shc::kilojoule_per_gram_degree_celsius>();
            test::<e::joule, m::kilogram, ti::degree_celsius,
                shc::joule_per_kilogram_degree_celsius>();
            test::<e::joule, m::gram, ti::degree_celsius, shc::joule_per_gram_degree_celsius>();
            test::<e::millijoule, m::kilogram, ti::degree_celsius,
                shc::millijoule_per_kilogram_degree_celsius>();
            test::<e::millijoule, m::gram, ti::degree_celsius,
                shc::millijoule_per_gram_degree_celsius>();

            test::<e::btu, m::ounce, ti::degree_fahrenheit, shc::btu_per_ounce_degree_fahrenheit>();
            test::<e::btu_it, m::ounce, ti::degree_fahrenheit,
                shc::btu_it_per_ounce_degree_fahrenheit>();
            test::<e::btu, m::pound, ti::degree_fahrenheit, shc::btu_per_pound_degree_fahrenheit>();
            test::<e::btu_it, m::pound, ti::degree_fahrenheit,
                shc::btu_it_per_pound_degree_fahrenheit>();
            test::<e::btu, m::ton, ti::degree_fahrenheit, shc::btu_per_ton_degree_fahrenheit>();
            test::<e::btu_it, m::ton, ti::degree_fahrenheit,
                shc::btu_it_per_ton_degree_fahrenheit>();

            fn test<
                E: e::Conversion<V>,
                M: m::Conversion<V>,
                TI: ti::Conversion<V>,
                SHC: shc::Conversion<V>>()
            {
                Test::assert_approx_eq(&SpecificHeatCapacity::new::<SHC>(V::one()),
                    &(Energy::new::<E>(V::one())
                        / (Mass::new::<M>(V::one()) * TemperatureInterval::new::<TI>(V::one()))));
            }
        }
    }
}
