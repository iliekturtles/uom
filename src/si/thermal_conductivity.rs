//! Thermal conductivity (base unit watt per meter kelvin, kg · m · s⁻³ · K⁻¹).
//!
//! Thermal conductivity has the same kind as [temperature interval][ti], as this quantity relates
//! to change of temperature. Not of kind `TemperatureKind`, used by [thermodynamic
//! temperature][tt]. See [thermodynamic temperature][tt] for a full explanation.
//!
//! [ti]: ../temperature_interval/index.html
//! [tt]: ../thermodynamic_temperature/index.html

quantity! {
    /// Thermal conductivity (base unit watt per meter kelvin, kg · m · s⁻³ · K⁻¹).
    quantity: ThermalConductivity; "thermal conductivity";
    /// Dimension of thermal conductivity, LMT⁻³Th⁻¹ (base unit watt per meter kelvin, kg · m · s⁻³
    /// · K⁻¹).
    dimension: ISQ<
        P1,     // length
        P1,     // mass
        N3,     // time
        Z0,     // electric current
        N1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottagram_meter_per_second_cubed_kelvin: prefix!(yotta) / prefix!(kilo); "Yg · m/(s³ · K)",
            "yottagram meter per second cubed kelvin", "yottagrams meter per second cubed kelvin";
        @zettagram_meter_per_second_cubed_kelvin: prefix!(zetta) / prefix!(kilo); "Zg · m/(s³ · K)",
            "zettagram meter per second cubed kelvin", "zettagrams meter per second cubed kelvin";
        @exagram_meter_per_second_cubed_kelvin: prefix!(exa) / prefix!(kilo); "Eg · m/(s³ · K)",
            "exagram meter per second cubed kelvin", "exagrams meter per second cubed kelvin";
        @petagram_meter_per_second_cubed_kelvin: prefix!(peta) / prefix!(kilo); "Pg · m/(s³ · K)",
            "petagram meter per second cubed kelvin", "petagrams meter per second cubed kelvin";
        @teragram_meter_per_second_cubed_kelvin: prefix!(tera) / prefix!(kilo); "Tg · m/(s³ · K)",
            "teragram meter per second cubed kelvin", "teragrams meter per second cubed kelvin";
        @gigagram_meter_per_second_cubed_kelvin: prefix!(giga) / prefix!(kilo); "Gg · m/(s³ · K)",
            "gigagram meter per second cubed kelvin", "gigagrams meter per second cubed kelvin";
        @megagram_meter_per_second_cubed_kelvin: prefix!(mega) / prefix!(kilo); "Mg · m/(s³ · K)",
            "megagram meter per second cubed kelvin", "megagrams meter per second cubed kelvin";
        /// Derived unit of thermal conductivity in base units. Equivalent to W/(m · K).
        @kilogram_meter_per_second_cubed_kelvin: prefix!(kilo) / prefix!(kilo); "kg · m/(s³ · K)",
            "kilogram meter per second cubed kelvin", "kilograms meter per second cubed kelvin";
        @hectogram_meter_per_second_cubed_kelvin: prefix!(hecto) / prefix!(kilo); "hg · m/(s³ · K)",
            "hectogram meter per second cubed kelvin", "hectograms meter per second cubed kelvin";
        @decagram_meter_per_second_cubed_kelvin: prefix!(deca) / prefix!(kilo); "dag · m/(s³ · K)",
            "decagram meter per second cubed kelvin", "decagrams meter per second cubed kelvin";
        @gram_meter_per_second_cubed_kelvin: prefix!(none) / prefix!(kilo); "g · m/(s³ · K)",
            "gram meter per second cubed kelvin", "grams meter per second cubed kelvin";
        @decigram_meter_per_second_cubed_kelvin: prefix!(deci) / prefix!(kilo); "dg · m/(s³ · K)",
            "decigram meter per second cubed kelvin", "decigrams meter per second cubed kelvin";
        @centigram_meter_per_second_cubed_kelvin: prefix!(centi) / prefix!(kilo); "cg · m/(s³ · K)",
            "centigram meter per second cubed kelvin", "centigrams meter per second cubed kelvin";
        @milligram_meter_per_second_cubed_kelvin: prefix!(milli) / prefix!(kilo); "mg · m/(s³ · K)",
            "milligram meter per second cubed kelvin", "milligrams meter per second cubed kelvin";
        @microgram_meter_per_second_cubed_kelvin: prefix!(micro) / prefix!(kilo); "µg · m/(s³ · K)",
            "microgram meter per second cubed kelvin", "micrograms meter per second cubed kelvin";
        @nanogram_meter_per_second_cubed_kelvin: prefix!(nano) / prefix!(kilo); "ng · m/(s³ · K)",
            "nanogram meter per second cubed kelvin", "nanograms meter per second cubed kelvin";
        @picogram_meter_per_second_cubed_kelvin: prefix!(pico) / prefix!(kilo); "pg · m/(s³ · K)",
            "picogram meter per second cubed kelvin", "picograms meter per second cubed kelvin";
        @femtogram_meter_per_second_cubed_kelvin: prefix!(femto) / prefix!(kilo); "fg · m/(s³ · K)",
            "femtogram meter per second cubed kelvin", "femtograms meter per second cubed kelvin";
        @attogram_meter_per_second_cubed_kelvin: prefix!(atto) / prefix!(kilo); "ag · m/(s³ · K)",
            "attogram meter per second cubed kelvin", "attograms meter per second cubed kelvin";
        @zeptogram_meter_per_second_cubed_kelvin: prefix!(zepto) / prefix!(kilo); "zg · m/(s³ · K)",
            "zeptogram meter per second cubed kelvin", "zeptograms meter per second cubed kelvin";
        @yoctogram_meter_per_second_cubed_kelvin: prefix!(yocto) / prefix!(kilo); "yg · m/(s³ · K)",
            "yoctogram meter per second cubed kelvin", "yoctograms meter per second cubed kelvin";

        // Thermal conductivity is much more commonly expressed in terms of
        // power / (length · temperature).
        @yottawatt_per_meter_kelvin: prefix!(yotta); "YW/(m · K)",
            "yottawatt per meter kelvin", "yottawatts per meter kelvin";
        @zettawatt_per_meter_kelvin: prefix!(zetta); "ZW/(m · K)",
            "zettawatt per meter kelvin", "zettawatts per meter kelvin";
        @exawatt_per_meter_kelvin: prefix!(exa); "EW/(m · K)",
            "exawatt per meter kelvin", "exawatts per meter kelvin";
        @petawatt_per_meter_kelvin: prefix!(peta); "PW/(m · K)",
            "petawatt per meter kelvin", "petawatts per meter kelvin";
        @terawatt_per_meter_kelvin: prefix!(tera); "TW/(m · K)",
            "terawatt per meter kelvin", "terawatts per meter kelvin";
        @gigawatt_per_meter_kelvin: prefix!(giga); "GW/(m · K)",
            "gigawatt per meter kelvin", "gigawatts per meter kelvin";
        @megawatt_per_meter_kelvin: prefix!(mega); "MW/(m · K)",
            "megawatt per meter kelvin", "megawatts per meter kelvin";
        @kilowatt_per_meter_kelvin: prefix!(kilo); "kW/(m · K)",
            "kilowatt per meter kelvin", "kilowatts per meter kelvin";
        @hectowatt_per_meter_kelvin: prefix!(hecto); "hW/(m · K)",
            "hectowatt per meter kelvin", "hectowatts per meter kelvin";
        @decawatt_per_meter_kelvin: prefix!(deca); "daW/(m · K)",
            "decawatt per meter kelvin", "decawatts per meter kelvin";
        /// Derived unit of thermal conductivity in derived units. Equivalent to kg · m/(s³ · K).
        @watt_per_meter_kelvin: prefix!(none); "W/(m · K)",
            "watt per meter kelvin", "watts per meter kelvin";
        @deciwatt_per_meter_kelvin: prefix!(deci); "dW/(m · K)",
            "deciwatt per meter kelvin", "deciwatts per meter kelvin";
        @centiwatt_per_meter_kelvin: prefix!(centi); "cW/(m · K)",
            "centiwatt per meter kelvin", "centiwatts per meter kelvin";
        @milliwatt_per_meter_kelvin: prefix!(milli); "mW/(m · K)",
            "milliwatt per meter kelvin", "milliwatts per meter kelvin";
        @microwatt_per_meter_kelvin: prefix!(micro); "µW/(m · K)",
            "microwatt per meter kelvin", "microwatts per meter kelvin";
        @nanowatt_per_meter_kelvin: prefix!(nano); "nW/(m · K)",
            "nanowatt per meter kelvin", "nanowatts per meter kelvin";
        @picowatt_per_meter_kelvin: prefix!(pico); "pW/(m · K)",
            "picowatt per meter kelvin", "picowatts per meter kelvin";
        @femtowatt_per_meter_kelvin: prefix!(femto); "fW/(m · K)",
            "femtowatt per meter kelvin", "femtowatts per meter kelvin";
        @attowatt_per_meter_kelvin: prefix!(atto); "aW/(m · K)",
            "attowatt per meter kelvin", "attowatts per meter kelvin";
        @zeptowatt_per_meter_kelvin: prefix!(zepto); "zW/(m · K)",
            "zeptowatt per meter kelvin", "zeptowatts per meter kelvin";
        @yoctowatt_per_meter_kelvin: prefix!(yocto); "yW/(m · K)",
            "yoctowatt per meter kelvin", "yoctowatts per meter kelvin";

        // Celsius for convenience.
        @kilogram_meter_per_second_cubed_degree_celsius: prefix!(kilo) / prefix!(kilo);
            "kg · m/(s³ · °C)", "kilogram meter per second cubed degree celsius",
            "kilograms meter per second cubed degree celsius";
        @kilowatt_per_meter_degree_celsius: prefix!(kilo); "kW/(m · °C)",
            "kilowatt per meter degree celsius", "kilowatts per meter degree celsius";
        /// Derived unit of thermal conductivity in derived units. Equivalent to kg · m/(s³ · K).
        @watt_per_meter_degree_celsius: prefix!(none); "W/(m · °C)",
            "watt per meter degree celsius", "watts per meter degree celsius";
        @milliwatt_per_meter_degree_celsius: prefix!(milli); "mW/(m · °C)",
            "milliwatt per meter degree celsius", "milliwatts per meter degree celsius";
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
        use si::power as p;
        use si::length as l;
        use si::thermal_conductivity as tc;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: ThermalConductivity<V> = Mass::new::<m::kilogram>(V::one())
                * Length::new::<l::meter>(V::one())
                / (Time::new::<t::second>(V::one())
                    * Time::new::<t::second>(V::one())
                    * Time::new::<t::second>(V::one())
                    * TemperatureInterval::new::<ti::kelvin>(V::one()));
        }

        #[test]
        fn check_base_units() {
            test::<m::yottagram, ti::kelvin, tc::yottagram_meter_per_second_cubed_kelvin>();
            test::<m::zettagram, ti::kelvin, tc::zettagram_meter_per_second_cubed_kelvin>();
            test::<m::exagram, ti::kelvin, tc::exagram_meter_per_second_cubed_kelvin>();
            test::<m::petagram, ti::kelvin, tc::petagram_meter_per_second_cubed_kelvin>();
            test::<m::teragram, ti::kelvin, tc::teragram_meter_per_second_cubed_kelvin>();
            test::<m::gigagram, ti::kelvin, tc::gigagram_meter_per_second_cubed_kelvin>();
            test::<m::megagram, ti::kelvin, tc::megagram_meter_per_second_cubed_kelvin>();
            test::<m::kilogram, ti::kelvin, tc::kilogram_meter_per_second_cubed_kelvin>();
            test::<m::hectogram, ti::kelvin, tc::hectogram_meter_per_second_cubed_kelvin>();
            test::<m::decagram, ti::kelvin, tc::decagram_meter_per_second_cubed_kelvin>();
            test::<m::gram, ti::kelvin, tc::gram_meter_per_second_cubed_kelvin>();
            test::<m::decigram, ti::kelvin, tc::decigram_meter_per_second_cubed_kelvin>();
            test::<m::centigram, ti::kelvin, tc::centigram_meter_per_second_cubed_kelvin>();
            test::<m::milligram, ti::kelvin, tc::milligram_meter_per_second_cubed_kelvin>();
            test::<m::microgram, ti::kelvin, tc::microgram_meter_per_second_cubed_kelvin>();
            test::<m::nanogram, ti::kelvin, tc::nanogram_meter_per_second_cubed_kelvin>();
            test::<m::picogram, ti::kelvin, tc::picogram_meter_per_second_cubed_kelvin>();
            test::<m::femtogram, ti::kelvin, tc::femtogram_meter_per_second_cubed_kelvin>();
            test::<m::attogram, ti::kelvin, tc::attogram_meter_per_second_cubed_kelvin>();
            test::<m::zeptogram, ti::kelvin, tc::zeptogram_meter_per_second_cubed_kelvin>();
            test::<m::yoctogram, ti::kelvin, tc::yoctogram_meter_per_second_cubed_kelvin>();

            test::<m::kilogram, ti::degree_celsius,
                tc::kilogram_meter_per_second_cubed_degree_celsius>();

            fn test<
                M: m::Conversion<V>,
                TI: ti::Conversion<V>,
                TC: tc::Conversion<V>>()
            {
                Test::assert_approx_eq(&ThermalConductivity::new::<TC>(V::one()),
                    &(Mass::new::<M>(V::one())
                        * Length::new::<l::meter>(V::one())
                        / (Time::new::<t::second>(V::one())
                            * Time::new::<t::second>(V::one())
                            * Time::new::<t::second>(V::one())
                            * TemperatureInterval::new::<TI>(V::one()))));
            }
        }

        #[test]
        fn check_power_per_length_ti_units() {
            test::<p::yottawatt, l::meter, ti::kelvin, tc::yottawatt_per_meter_kelvin>();
            test::<p::zettawatt, l::meter, ti::kelvin, tc::zettawatt_per_meter_kelvin>();
            test::<p::exawatt, l::meter, ti::kelvin, tc::exawatt_per_meter_kelvin>();
            test::<p::petawatt, l::meter, ti::kelvin, tc::petawatt_per_meter_kelvin>();
            test::<p::terawatt, l::meter, ti::kelvin, tc::terawatt_per_meter_kelvin>();
            test::<p::gigawatt, l::meter, ti::kelvin, tc::gigawatt_per_meter_kelvin>();
            test::<p::megawatt, l::meter, ti::kelvin, tc::megawatt_per_meter_kelvin>();
            test::<p::kilowatt, l::meter, ti::kelvin, tc::kilowatt_per_meter_kelvin>();
            test::<p::hectowatt, l::meter, ti::kelvin, tc::hectowatt_per_meter_kelvin>();
            test::<p::decawatt, l::meter, ti::kelvin, tc::decawatt_per_meter_kelvin>();
            test::<p::watt, l::meter, ti::kelvin, tc::watt_per_meter_kelvin>();
            test::<p::deciwatt, l::meter, ti::kelvin, tc::deciwatt_per_meter_kelvin>();
            test::<p::centiwatt, l::meter, ti::kelvin, tc::centiwatt_per_meter_kelvin>();
            test::<p::milliwatt, l::meter, ti::kelvin, tc::milliwatt_per_meter_kelvin>();
            test::<p::microwatt, l::meter, ti::kelvin, tc::microwatt_per_meter_kelvin>();
            test::<p::nanowatt, l::meter, ti::kelvin, tc::nanowatt_per_meter_kelvin>();
            test::<p::picowatt, l::meter, ti::kelvin, tc::picowatt_per_meter_kelvin>();
            test::<p::femtowatt, l::meter, ti::kelvin, tc::femtowatt_per_meter_kelvin>();
            test::<p::attowatt, l::meter, ti::kelvin, tc::attowatt_per_meter_kelvin>();
            test::<p::zeptowatt, l::meter, ti::kelvin, tc::zeptowatt_per_meter_kelvin>();
            test::<p::yoctowatt, l::meter, ti::kelvin, tc::yoctowatt_per_meter_kelvin>();

            test::<p::kilowatt, l::meter, ti::degree_celsius,
                tc::kilowatt_per_meter_degree_celsius>();
            test::<p::watt, l::meter, ti::degree_celsius, tc::watt_per_meter_degree_celsius>();
            test::<p::milliwatt, l::meter, ti::degree_celsius,
                tc::milliwatt_per_meter_degree_celsius>();

            fn test<
                P: p::Conversion<V>,
                L: l::Conversion<V>,
                TI: ti::Conversion<V>,
                TC: tc::Conversion<V>>()
            {
                Test::assert_approx_eq(&ThermalConductivity::new::<TC>(V::one()),
                    &(Power::new::<P>(V::one())
                        / (Length::new::<L>(V::one()) * TemperatureInterval::new::<TI>(V::one()))));
            }
        }
    }
}
