//! Thermal conductance (base unit watt per kelvin, kg · m² · s⁻³ · K⁻¹).
//!
//! Thermal conductance has the same kind as [temperature interval][ti], as this quantity relates
//! to change of temperature. Not of kind `TemperatureKind`, used by [thermodynamic
//! temperature][tt]. See [thermodynamic temperature][tt] for a full explanation.
//!
//! [ti]: ../temperature_interval/index.html
//! [tt]: ../thermodynamic_temperature/index.html

quantity! {
    /// Thermal conductance (base unit watt per kelvin, kg · m² · s⁻³ · K⁻¹).
    quantity: ThermalConductance; "thermal conductance";
    /// Dimension of thermal conductance, LM²T⁻³Th⁻¹ (base unit watt per kelvin, kg · m² · s⁻³
    /// · K⁻¹).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N3,     // time
        Z0,     // electric current
        N1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottagram_meter_squared_per_second_cubed_kelvin: prefix!(yotta) / prefix!(kilo);
            "Yg · m²/(s³ · K)", "yottagram meter squared per second cubed kelvin",
            "yottagrams meter squared per second cubed kelvin";
        @zettagram_meter_squared_per_second_cubed_kelvin: prefix!(zetta) / prefix!(kilo);
            "Zg · m²/(s³ · K)", "zettagram meter squared per second cubed kelvin",
            "zettagrams meter squared per second cubed kelvin";
        @exagram_meter_squared_per_second_cubed_kelvin: prefix!(exa) / prefix!(kilo); "Eg · m²/(s³ · K)",
            "exagram meter squared per second cubed kelvin",
            "exagrams meter squared per second cubed kelvin";
        @petagram_meter_squared_per_second_cubed_kelvin: prefix!(peta) / prefix!(kilo); "Pg · m²/(s³ · K)",
            "petagram meter squared per second cubed kelvin",
            "petagrams meter squared per second cubed kelvin";
        @teragram_meter_squared_per_second_cubed_kelvin: prefix!(tera) / prefix!(kilo); "Tg · m²/(s³ · K)",
            "teragram meter squared per second cubed kelvin",
            "teragrams meter squared per second cubed kelvin";
        @gigagram_meter_squared_per_second_cubed_kelvin: prefix!(giga) / prefix!(kilo); "Gg · m²/(s³ · K)",
            "gigagram meter squared per second cubed kelvin",
            "gigagrams meter squared per second cubed kelvin";
        @megagram_meter_squared_per_second_cubed_kelvin: prefix!(mega) / prefix!(kilo); "Mg · m²/(s³ · K)",
            "megagram meter squared per second cubed kelvin",
            "megagrams meter squared per second cubed kelvin";
        /// Derived unit of thermal conductance in base units. Equivalent to W/K.
        @kilogram_meter_squared_per_second_cubed_kelvin: prefix!(kilo) / prefix!(kilo); "kg · m²/(s³ · K)",
            "kilogram meter squared per second cubed kelvin",
            "kilograms meter squared per second cubed kelvin";
        @hectogram_meter_squared_per_second_cubed_kelvin: prefix!(hecto) / prefix!(kilo);
            "hg · m²/(s³ · K)", "hectogram meter squared per second cubed kelvin",
            "hectograms meter squared per second cubed kelvin";
        @decagram_meter_squared_per_second_cubed_kelvin: prefix!(deca) / prefix!(kilo); "dag · m²/(s³ · K)",
            "decagram meter squared per second cubed kelvin",
            "decagrams meter squared per second cubed kelvin";
        @gram_meter_squared_per_second_cubed_kelvin: prefix!(none) / prefix!(kilo); "g · m/(s³ · K)",
            "gram meter squared per second cubed kelvin", "grams meter squared per second cubed kelvin";
        @decigram_meter_squared_per_second_cubed_kelvin: prefix!(deci) / prefix!(kilo); "dg · m²/(s³ · K)",
            "decigram meter squared per second cubed kelvin",
            "decigrams meter squared per second cubed kelvin";
        @centigram_meter_squared_per_second_cubed_kelvin: prefix!(centi) / prefix!(kilo);
            "cg · m²/(s³ · K)", "centigram meter squared per second cubed kelvin",
            "centigrams meter squared per second cubed kelvin";
        @milligram_meter_squared_per_second_cubed_kelvin: prefix!(milli) / prefix!(kilo);
            "mg · m²/(s³ · K)", "milligram meter squared per second cubed kelvin",
            "milligrams meter squared per second cubed kelvin";
        @microgram_meter_squared_per_second_cubed_kelvin: prefix!(micro) / prefix!(kilo); "µg · m/(s³ · K)",
            "microgram meter squared per second cubed kelvin",
            "micrograms meter squared per second cubed kelvin";
        @nanogram_meter_squared_per_second_cubed_kelvin: prefix!(nano) / prefix!(kilo); "ng · m²/(s³ · K)",
            "nanogram meter squared per second cubed kelvin",
            "nanograms meter squared per second cubed kelvin";
        @picogram_meter_squared_per_second_cubed_kelvin: prefix!(pico) / prefix!(kilo); "pg · m²/(s³ · K)",
            "picogram meter squared per second cubed kelvin",
            "picograms meter squared per second cubed kelvin";
        @femtogram_meter_squared_per_second_cubed_kelvin: prefix!(femto) / prefix!(kilo);
            "fg · m²/(s³ · K)", "femtogram meter squared per second cubed kelvin",
            "femtograms meter squared per second cubed kelvin";
        @attogram_meter_squared_per_second_cubed_kelvin: prefix!(atto) / prefix!(kilo); "ag · m²/(s³ · K)",
            "attogram meter squared per second cubed kelvin",
            "attograms meter squared per second cubed kelvin";
        @zeptogram_meter_squared_per_second_cubed_kelvin: prefix!(zepto) / prefix!(kilo);
            "zg · m²/(s³ · K)", "zeptogram meter squared per second cubed kelvin",
            "zeptograms meter squared per second cubed kelvin";
        @yoctogram_meter_squared_per_second_cubed_kelvin: prefix!(yocto) / prefix!(kilo);
            "yg · m²/(s³ · K)", "yoctogram meter squared per second cubed kelvin",
            "yoctograms meter squared per second cubed kelvin";

        // Thermal conductance is much more commonly expressed in terms of power / temperature.
        @yottawatt_per_kelvin: prefix!(yotta); "YW/K", "yottawatt per kelvin",
            "yottawatts per kelvin";
        @zettawatt_per_kelvin: prefix!(zetta); "ZW/K", "zettawatt per kelvin",
            "zettawatts per kelvin";
        @exawatt_per_kelvin: prefix!(exa); "EW/K", "exawatt per kelvin", "exawatts per kelvin";
        @petawatt_per_kelvin: prefix!(peta); "PW/K", "petawatt per kelvin", "petawatts per kelvin";
        @terawatt_per_kelvin: prefix!(tera); "TW/K", "terawatt per kelvin", "terawatts per kelvin";
        @gigawatt_per_kelvin: prefix!(giga); "GW/K", "gigawatt per kelvin", "gigawatts per kelvin";
        @megawatt_per_kelvin: prefix!(mega); "MW/K", "megawatt per kelvin", "megawatts per kelvin";
        @kilowatt_per_kelvin: prefix!(kilo); "kW/K", "kilowatt per kelvin", "kilowatts per kelvin";
        @hectowatt_per_kelvin: prefix!(hecto); "hW/K", "hectowatt per kelvin",
            "hectowatts per kelvin";
        @decawatt_per_kelvin: prefix!(deca); "daW/K", "decawatt per kelvin", "decawatts per kelvin";
        /// Derived unit of thermal conductance in derived units. Equivalent to kg · m²/(s³ · K).
        @watt_per_kelvin: prefix!(none); "W/K", "watt per kelvin", "watts per kelvin";
        @deciwatt_per_kelvin: prefix!(deci); "dW/K", "deciwatt per kelvin", "deciwatts per kelvin";
        @centiwatt_per_kelvin: prefix!(centi); "cW/K", "centiwatt per kelvin",
            "centiwatts per kelvin";
        @milliwatt_per_kelvin: prefix!(milli); "mW/K", "milliwatt per kelvin",
            "milliwatts per kelvin";
        @microwatt_per_kelvin: prefix!(micro); "µW/K", "microwatt per kelvin",
            "microwatts per kelvin";
        @nanowatt_per_kelvin: prefix!(nano); "nW/K", "nanowatt per kelvin", "nanowatts per kelvin";
        @picowatt_per_kelvin: prefix!(pico); "pW/K", "picowatt per kelvin", "picowatts per kelvin";
        @femtowatt_per_kelvin: prefix!(femto); "fW/K", "femtowatt per kelvin",
            "femtowatts per kelvin";
        @attowatt_per_kelvin: prefix!(atto); "aW/K", "attowatt per kelvin", "attowatts per kelvin";
        @zeptowatt_per_kelvin: prefix!(zepto); "zW/K", "zeptowatt per kelvin",
            "zeptowatts per kelvin";
        @yoctowatt_per_kelvin: prefix!(yocto); "yW/K", "yoctowatt per kelvin",
            "yoctowatts per kelvin";

        // Celsius for convenience.
        @kilogram_meter_squared_per_second_cubed_degree_celsius: prefix!(kilo) / prefix!(kilo);
            "kg · m²/(s³ · °C)", "kilogram meter squared per second cubed degree Celsius",
            "kilograms meter squared per second cubed degree Celsius";
        @kilowatt_per_degree_celsius: prefix!(kilo); "kW/°C", "kilowatt per degree Celsius",
            "kilowatts per degree Celsius";
        /// Derived unit of thermal conductance in derived units. Equivalent to kg · m²/(s³ · K).
        @watt_per_meter_degree_celsius: prefix!(none); "W/°C", "watt per degree Celsius",
            "watts per degree Celsius";
        @milliwatt_per_degree_celsius: prefix!(milli); "mW/°C", "milliwatt per degree Celsius",
            "milliwatts per degree Celsius";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::length as l;
        use crate::si::mass as m;
        use crate::si::power as p;
        use crate::si::quantities::*;
        use crate::si::temperature_interval as ti;
        use crate::si::thermal_conductance as tc;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ThermalConductance<V> = Mass::new::<m::kilogram>(V::one())
                * Length::new::<l::meter>(V::one()) * Length::new::<l::meter>(V::one())
                / (Time::new::<t::second>(V::one())
                    * Time::new::<t::second>(V::one())
                    * Time::new::<t::second>(V::one())
                    * TemperatureInterval::new::<ti::kelvin>(V::one()));
        }

        #[test]
        fn check_base_units() {
            test::<m::yottagram, ti::kelvin, tc::yottagram_meter_squared_per_second_cubed_kelvin>();
            test::<m::zettagram, ti::kelvin, tc::zettagram_meter_squared_per_second_cubed_kelvin>();
            test::<m::exagram, ti::kelvin, tc::exagram_meter_squared_per_second_cubed_kelvin>();
            test::<m::petagram, ti::kelvin, tc::petagram_meter_squared_per_second_cubed_kelvin>();
            test::<m::teragram, ti::kelvin, tc::teragram_meter_squared_per_second_cubed_kelvin>();
            test::<m::gigagram, ti::kelvin, tc::gigagram_meter_squared_per_second_cubed_kelvin>();
            test::<m::megagram, ti::kelvin, tc::megagram_meter_squared_per_second_cubed_kelvin>();
            test::<m::kilogram, ti::kelvin, tc::kilogram_meter_squared_per_second_cubed_kelvin>();
            test::<m::hectogram, ti::kelvin, tc::hectogram_meter_squared_per_second_cubed_kelvin>();
            test::<m::decagram, ti::kelvin, tc::decagram_meter_squared_per_second_cubed_kelvin>();
            test::<m::gram, ti::kelvin, tc::gram_meter_squared_per_second_cubed_kelvin>();
            test::<m::decigram, ti::kelvin, tc::decigram_meter_squared_per_second_cubed_kelvin>();
            test::<m::centigram, ti::kelvin, tc::centigram_meter_squared_per_second_cubed_kelvin>();
            test::<m::milligram, ti::kelvin, tc::milligram_meter_squared_per_second_cubed_kelvin>();
            test::<m::microgram, ti::kelvin, tc::microgram_meter_squared_per_second_cubed_kelvin>();
            test::<m::nanogram, ti::kelvin, tc::nanogram_meter_squared_per_second_cubed_kelvin>();
            test::<m::picogram, ti::kelvin, tc::picogram_meter_squared_per_second_cubed_kelvin>();
            test::<m::femtogram, ti::kelvin, tc::femtogram_meter_squared_per_second_cubed_kelvin>();
            test::<m::attogram, ti::kelvin, tc::attogram_meter_squared_per_second_cubed_kelvin>();
            test::<m::zeptogram, ti::kelvin, tc::zeptogram_meter_squared_per_second_cubed_kelvin>();
            test::<m::yoctogram, ti::kelvin, tc::yoctogram_meter_squared_per_second_cubed_kelvin>();

            test::<m::kilogram, ti::degree_celsius,
                tc::kilogram_meter_squared_per_second_cubed_degree_celsius>();

            fn test<
                M: m::Conversion<V>,
                TI: ti::Conversion<V>,
                TC: tc::Conversion<V>>()
            {
                Test::assert_approx_eq(&ThermalConductance::new::<TC>(V::one()),
                    &(Mass::new::<M>(V::one())
                        * Length::new::<l::meter>(V::one()) * Length::new::<l::meter>(V::one())
                        / (Time::new::<t::second>(V::one())
                            * Time::new::<t::second>(V::one())
                            * Time::new::<t::second>(V::one())
                            * TemperatureInterval::new::<TI>(V::one()))));
            }
        }

        #[test]
        fn check_power_ti_units() {
            test::<p::yottawatt, ti::kelvin, tc::yottawatt_per_kelvin>();
            test::<p::zettawatt, ti::kelvin, tc::zettawatt_per_kelvin>();
            test::<p::exawatt, ti::kelvin, tc::exawatt_per_kelvin>();
            test::<p::petawatt, ti::kelvin, tc::petawatt_per_kelvin>();
            test::<p::terawatt, ti::kelvin, tc::terawatt_per_kelvin>();
            test::<p::gigawatt, ti::kelvin, tc::gigawatt_per_kelvin>();
            test::<p::megawatt, ti::kelvin, tc::megawatt_per_kelvin>();
            test::<p::kilowatt, ti::kelvin, tc::kilowatt_per_kelvin>();
            test::<p::hectowatt, ti::kelvin, tc::hectowatt_per_kelvin>();
            test::<p::decawatt, ti::kelvin, tc::decawatt_per_kelvin>();
            test::<p::watt, ti::kelvin, tc::watt_per_kelvin>();
            test::<p::deciwatt, ti::kelvin, tc::deciwatt_per_kelvin>();
            test::<p::centiwatt, ti::kelvin, tc::centiwatt_per_kelvin>();
            test::<p::milliwatt, ti::kelvin, tc::milliwatt_per_kelvin>();
            test::<p::microwatt, ti::kelvin, tc::microwatt_per_kelvin>();
            test::<p::nanowatt, ti::kelvin, tc::nanowatt_per_kelvin>();
            test::<p::picowatt, ti::kelvin, tc::picowatt_per_kelvin>();
            test::<p::femtowatt, ti::kelvin, tc::femtowatt_per_kelvin>();
            test::<p::attowatt, ti::kelvin, tc::attowatt_per_kelvin>();
            test::<p::zeptowatt, ti::kelvin, tc::zeptowatt_per_kelvin>();
            test::<p::yoctowatt, ti::kelvin, tc::yoctowatt_per_kelvin>();

            test::<p::kilowatt, ti::degree_celsius, tc::kilowatt_per_degree_celsius>();
            test::<p::watt, ti::degree_celsius, tc::watt_per_meter_degree_celsius>();
            test::<p::milliwatt, ti::degree_celsius, tc::milliwatt_per_degree_celsius>();

            fn test<
                P: p::Conversion<V>,
                TI: ti::Conversion<V>,
                TC: tc::Conversion<V>>()
            {
                Test::assert_approx_eq(&ThermalConductance::new::<TC>(V::one()),
                    &(Power::new::<P>(V::one())
                        / TemperatureInterval::new::<TI>(V::one())));
            }
        }
    }
}
