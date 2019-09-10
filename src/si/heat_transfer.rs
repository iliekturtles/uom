//! Heat transfer (base unit watt per square meter kelvin, kg · s⁻³ · K⁻¹).
//!
//! Heat transfer is also known as heat transfer coefficient, film coefficient, or film
//! effectiveness. Commonly expressed using derived units power and area: watt per square meter
//! kelvin.
//!
//! Heat transfer has the same kind as [temperature interval][ti], as this quantity relates to
//! change of temperature. Not of kind `TemperatureKind`, used by [thermodynamic temperature][tt].
//! See [thermodynamic temperature][tt] for a full explanation.
//!
//! [ti]: ../temperature_interval/index.html
//! [tt]: ../thermodynamic_temperature/index.html

quantity! {
    /// Heat transfer (base unit watt per square meter kelvin, kg · s⁻³ · K⁻¹).
    quantity: HeatTransfer; "heat transfer";
    /// Dimension of heat transfer, MT⁻³Th⁻¹ (base unit watt per square meter kelvin,
    /// kg · s⁻³ · K⁻¹).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        N3,     // time
        Z0,     // electric current
        N1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottagram_per_second_cubed_kelvin: prefix!(yotta) / prefix!(kilo); "Yg/(s³ · K)",
            "yottagram per second cubed kelvin", "yottagrams per second cubed kelvin";
        @zettagram_per_second_cubed_kelvin: prefix!(zetta) / prefix!(kilo); "Zg/(s³ · K)",
            "zettagram per second cubed kelvin", "zettagrams per second cubed kelvin";
        @exagram_per_second_cubed_kelvin: prefix!(exa) / prefix!(kilo); "Eg/(s³ · K)",
            "exagram per second cubed kelvin", "exagrams per second cubed kelvin";
        @petagram_per_second_cubed_kelvin: prefix!(peta) / prefix!(kilo); "Pg/(s³ · K)",
            "petagram per second cubed kelvin", "petagrams per second cubed kelvin";
        @teragram_per_second_cubed_kelvin: prefix!(tera) / prefix!(kilo); "Tg/(s³ · K)",
            "teragram per second cubed kelvin", "teragrams per second cubed kelvin";
        @gigagram_per_second_cubed_kelvin: prefix!(giga) / prefix!(kilo); "Gg/(s³ · K)",
            "gigagram per second cubed kelvin", "gigagrams per second cubed kelvin";
        @megagram_per_second_cubed_kelvin: prefix!(mega) / prefix!(kilo); "Mg/(s³ · K)",
            "megagram per second cubed kelvin", "megagrams per second cubed kelvin";
        /// Derived unit of heat transfer in base units. Equivalent to W/(m² · K).
        @kilogram_per_second_cubed_kelvin: prefix!(kilo) / prefix!(kilo); "kg/(s³ · K)",
            "kilogram per second cubed kelvin", "kilograms per second cubed kelvin";
        @hectogram_per_second_cubed_kelvin: prefix!(hecto) / prefix!(kilo); "hg/(s³ · K)",
            "hectogram per second cubed kelvin", "hectograms per second cubed kelvin";
        @decagram_per_second_cubed_kelvin: prefix!(deca) / prefix!(kilo); "dag/(s³ · K)",
            "decagram per second cubed kelvin", "decagrams per second cubed kelvin";
        @gram_per_second_cubed_kelvin: prefix!(none) / prefix!(kilo); "g/(s³ · K)",
            "gram per second cubed kelvin", "grams per second cubed kelvin";
        @decigram_per_second_cubed_kelvin: prefix!(deci) / prefix!(kilo); "dg/(s³ · K)",
            "decigram per second cubed kelvin", "decigrams per second cubed kelvin";
        @centigram_per_second_cubed_kelvin: prefix!(centi) / prefix!(kilo); "cg/(s³ · K)",
            "centigram per second cubed kelvin", "centigrams per second cubed kelvin";
        @milligram_per_second_cubed_kelvin: prefix!(milli) / prefix!(kilo); "mg/(s³ · K)",
            "milligram per second cubed kelvin", "milligrams per second cubed kelvin";
        @microgram_per_second_cubed_kelvin: prefix!(micro) / prefix!(kilo); "µg/(s³ · K)",
            "microgram per second cubed kelvin", "micrograms per second cubed kelvin";
        @nanogram_per_second_cubed_kelvin: prefix!(nano) / prefix!(kilo); "ng/(s³ · K)",
            "nanogram per second cubed kelvin", "nanograms per second cubed kelvin";
        @picogram_per_second_cubed_kelvin: prefix!(pico) / prefix!(kilo); "pg/(s³ · K)",
            "picogram per second cubed kelvin", "picograms per second cubed kelvin";
        @femtogram_per_second_cubed_kelvin: prefix!(femto) / prefix!(kilo); "fg/(s³ · K)",
            "femtogram per second cubed kelvin", "femtograms per second cubed kelvin";
        @attogram_per_second_cubed_kelvin: prefix!(atto) / prefix!(kilo); "ag/(s³ · K)",
            "attogram per second cubed kelvin", "attograms per second cubed kelvin";
        @zeptogram_per_second_cubed_kelvin: prefix!(zepto) / prefix!(kilo); "zg/(s³ · K)",
            "zeptogram per second cubed kelvin", "zeptograms per second cubed kelvin";
        @yoctogram_per_second_cubed_kelvin: prefix!(yocto) / prefix!(kilo); "yg/(s³ · K)",
            "yoctogram per second cubed kelvin", "yoctograms per second cubed kelvin";

        @kilogram_per_second_cubed_degree_celsius: prefix!(kilo) / prefix!(kilo); "kg/(s³ · °C)",
            "kilogram per second cubed degree celsius", "kilograms per second cubed degree celsius";

        // Heat transfer is much more commonly expressed in terms of power / area (heat flux).
        @yottawatt_per_square_meter_kelvin: prefix!(yotta); "YW/(m² · K)",
            "yottawatt per square meter kelvin", "yottawatts per square meter kelvin";
        @zettawatt_per_square_meter_kelvin: prefix!(zetta); "ZW/(m² · K)",
            "zettawatt per square meter kelvin", "zettawatts per square meter kelvin";
        @exawatt_per_square_meter_kelvin: prefix!(exa); "EW/(m² · K)",
            "exawatt per square meter kelvin", "exawatts per square meter kelvin";
        @petawatt_per_square_meter_kelvin: prefix!(peta); "PW/(m² · K)",
            "petawatt per square meter kelvin", "petawatts per square meter kelvin";
        @terawatt_per_square_meter_kelvin: prefix!(tera); "TW/(m² · K)",
            "terawatt per square meter kelvin", "terawatts per square meter kelvin";
        @gigawatt_per_square_meter_kelvin: prefix!(giga); "GW/(m² · K)",
            "gigawatt per square meter kelvin", "gigawatts per square meter kelvin";
        @megawatt_per_square_meter_kelvin: prefix!(mega); "MW/(m² · K)",
            "megawatt per square meter kelvin", "megawatts per square meter kelvin";
        @kilowatt_per_square_meter_kelvin: prefix!(kilo); "kW/(m² · K)",
            "kilowatt per square meter kelvin", "kilowatts per square meter kelvin";
        @hectowatt_per_square_meter_kelvin: prefix!(hecto); "hW/(m² · K)",
            "hectowatt per square meter kelvin", "hectowatts per square meter kelvin";
        @decawatt_per_square_meter_kelvin: prefix!(deca); "daW/(m² · K)",
            "decawatt per square meter kelvin", "decawatts per square meter kelvin";
        /// Derived unit of heat transfer in derived units. Equivalent to kg/(s³ · K).
        @watt_per_square_meter_kelvin: prefix!(none); "W/(m² · K)",
            "watt per square meter kelvin", "watts per square meter kelvin";
        @deciwatt_per_square_meter_kelvin: prefix!(deci); "dW/(m² · K)",
            "deciwatt per square meter kelvin", "deciwatts per square meter kelvin";
        @centiwatt_per_square_meter_kelvin: prefix!(centi); "cW/(m² · K)",
            "centiwatt per square meter kelvin", "centiwatts per square meter kelvin";
        @milliwatt_per_square_meter_kelvin: prefix!(milli); "mW/(m² · K)",
            "milliwatt per square meter kelvin", "milliwatts per square meter kelvin";
        @microwatt_per_square_meter_kelvin: prefix!(micro); "µW/(m² · K)",
            "microwatt per square meter kelvin", "microwatts per square meter kelvin";
        @nanowatt_per_square_meter_kelvin: prefix!(nano); "nW/(m² · K)",
            "nanowatt per square meter kelvin", "nanowatts per square meter kelvin";
        @picowatt_per_square_meter_kelvin: prefix!(pico); "pW/(m² · K)",
            "picowatt per square meter kelvin", "picowatts per square meter kelvin";
        @femtowatt_per_square_meter_kelvin: prefix!(femto); "fW/(m² · K)",
            "femtowatt per square meter kelvin", "femtowatts per square meter kelvin";
        @attowatt_per_square_meter_kelvin: prefix!(atto); "aW/(m² · K)",
            "attowatt per square meter kelvin", "attowatts per square meter kelvin";
        @zeptowatt_per_square_meter_kelvin: prefix!(zepto); "zW/(m² · K)",
            "zeptowatt per square meter kelvin", "zeptowatts per square meter kelvin";
        @yoctowatt_per_square_meter_kelvin: prefix!(yocto); "yW/(m² · K)",
            "yoctowatt per square meter kelvin", "yoctowatts per square meter kelvin";

        // Power per area temperature interval
        @watt_per_square_kilometer_degree_celsius: prefix!(none) / (prefix!(kilo) * prefix!(kilo));
            "W/(km² · °C)", "watt per square kilometer degree celsius",
            "watts per square kilometer degree celsius";
        @watt_per_square_meter_degree_celsius: prefix!(none); "W/(m² · °C)",
            "watt per square meter degree celsius", "watts per square meter degree celsius";
        @watt_per_square_centimeter_degree_celsius:
            prefix!(none) / (prefix!(centi) * prefix!(centi)); "W/(cm² · °C)",
            "watt per square centimeter degree celsius",
            "watts per square centimeter degree celsius";
        @watt_per_square_millimeter_degree_celsius:
            prefix!(none) / (prefix!(milli) * prefix!(milli)); "W/(mm² · °C)",
            "watt per square millimeter degree celsius",
            "watts per square millimeter degree celsius";

        // Energy per time area temperature interval
        @joule_per_second_square_meter_kelvin: prefix!(none); "J/(s · m² · K)",
            "joule per second square meter kelvin", "joules per second square meter kelvin";
        @joule_per_second_square_meter_degree_celsius: prefix!(none); "J/(s · m² · °C)",
            "joule per second square meter degree celsius",
            "joules per second square meter degree celsius";
        @btu_it_per_hour_square_foot_degree_fahrenheit: 5.678_264_134_306_046;
            "Btu (IT)/(hr · ft² · °F)",
            "British thermal unit (IT) per hour square foot degree Fahrenheit",
            "British thermal units (IT) per hour square foot degree Fahrenheit";
        @btu_it_per_hour_square_inch_degree_fahrenheit: 8.176_700_353_400_707_E2;
            "Btu (IT)/(hr · in² · °F)",
            "British thermal unit (IT) per hour square inch degree Fahrenheit",
            "British thermal units (IT) per hour square inch degree Fahrenheit";
        @btu_it_per_minute_square_foot_degree_fahrenheit: 3.406_958_480_583_627_4_E2;
            "Btu (IT)/(min · ft² · °F)",
            "British thermal unit (IT) per minute square foot degree Fahrenheit",
            "British thermal units (IT) per minute square foot degree Fahrenheit";
        @btu_it_per_minute_square_inch_degree_fahrenheit: 4.906_020_212_040_425_E4;
            "Btu (IT)/(min · in² · °F)",
            "British thermal unit (IT) per minute square inch degree Fahrenheit",
            "British thermal units (IT) per minute square inch degree Fahrenheit";
        @btu_it_per_second_square_foot_degree_fahrenheit: 2.044_175_088_350_176_5_E4;
            "Btu (IT)/(s · ft² · °F)",
            "British thermal unit (IT) per second square foot degree Fahrenheit",
            "British thermal units (IT) per second square foot degree Fahrenheit";
        @btu_it_per_second_square_inch_degree_fahrenheit: 2.943_612_127_224_254_4_E6;
            "Btu (IT)/(s · in² · °F)",
            "British thermal unit (IT) per second square inch degree Fahrenheit",
            "British thermal units (IT) per second square inch degree Fahrenheit";
        @btu_per_hour_square_foot_degree_fahrenheit: 5.674_464_473_928_946; "Btu/(hr · ft² · °F)",
            "British thermal unit per hour square foot degree Fahrenheit",
            "British thermal units per hour square foot degree Fahrenheit";
        @btu_per_hour_square_inch_degree_fahrenheit: 8.171_228_842_457_684_E2;
            "Btu/(hr · in² · °F)", "British thermal unit per hour square inch degree Fahrenheit",
            "British thermal units per hour square inch degree Fahrenheit";
        @btu_per_minute_square_foot_degree_fahrenheit: 3.404_678_684_357_368_E2;
            "Btu/(min · ft² · °F)", "British thermal unit per minute square foot degree Fahrenheit",
            "British thermal units per minute square foot degree Fahrenheit";
        @btu_per_minute_square_inch_degree_fahrenheit: 4.902_737_305_474_611_E4;
            "Btu/(min · in² · °F)", "British thermal unit per minute square inch degree Fahrenheit",
            "British thermal units per minute square inch degree Fahrenheit";
        @btu_per_second_square_foot_degree_fahrenheit: 2.042_807_210_614_421_E4;
            "Btu/(s · ft² · °F)", "British thermal unit per second square foot degree Fahrenheit",
            "British thermal units per second square foot degree Fahrenheit";
        @btu_per_second_square_inch_degree_fahrenheit: 2.941_642_383_284_766_E6;
            "Btu/(s · in² · °F)", "British thermal unit per second square inch degree Fahrenheit",
            "British thermal units per second square inch degree Fahrenheit";
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
        use si::energy as e;
        use si::area as a;
        use si::heat_transfer as ht;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: HeatTransfer<V> = Mass::new::<m::kilogram>(V::one())
                / (Time::new::<t::second>(V::one())
                    * Time::new::<t::second>(V::one())
                    * Time::new::<t::second>(V::one())
                    * TemperatureInterval::new::<ti::kelvin>(V::one()));
        }

        #[test]
        fn check_base_units() {
            test::<m::yottagram, t::second, ti::kelvin, ht::yottagram_per_second_cubed_kelvin>();
            test::<m::zettagram, t::second, ti::kelvin, ht::zettagram_per_second_cubed_kelvin>();
            test::<m::exagram, t::second, ti::kelvin, ht::exagram_per_second_cubed_kelvin>();
            test::<m::petagram, t::second, ti::kelvin, ht::petagram_per_second_cubed_kelvin>();
            test::<m::teragram, t::second, ti::kelvin, ht::teragram_per_second_cubed_kelvin>();
            test::<m::gigagram, t::second, ti::kelvin, ht::gigagram_per_second_cubed_kelvin>();
            test::<m::megagram, t::second, ti::kelvin, ht::megagram_per_second_cubed_kelvin>();
            test::<m::kilogram, t::second, ti::kelvin, ht::kilogram_per_second_cubed_kelvin>();
            test::<m::hectogram, t::second, ti::kelvin, ht::hectogram_per_second_cubed_kelvin>();
            test::<m::decagram, t::second, ti::kelvin, ht::decagram_per_second_cubed_kelvin>();
            test::<m::gram, t::second, ti::kelvin, ht::gram_per_second_cubed_kelvin>();
            test::<m::decigram, t::second, ti::kelvin, ht::decigram_per_second_cubed_kelvin>();
            test::<m::centigram, t::second, ti::kelvin, ht::centigram_per_second_cubed_kelvin>();
            test::<m::milligram, t::second, ti::kelvin, ht::milligram_per_second_cubed_kelvin>();
            test::<m::microgram, t::second, ti::kelvin, ht::microgram_per_second_cubed_kelvin>();
            test::<m::nanogram, t::second, ti::kelvin, ht::nanogram_per_second_cubed_kelvin>();
            test::<m::picogram, t::second, ti::kelvin, ht::picogram_per_second_cubed_kelvin>();
            test::<m::femtogram, t::second, ti::kelvin, ht::femtogram_per_second_cubed_kelvin>();
            test::<m::attogram, t::second, ti::kelvin, ht::attogram_per_second_cubed_kelvin>();
            test::<m::zeptogram, t::second, ti::kelvin, ht::zeptogram_per_second_cubed_kelvin>();
            test::<m::yoctogram, t::second, ti::kelvin, ht::yoctogram_per_second_cubed_kelvin>();

            test::<m::kilogram, t::second, ti::degree_celsius,
                ht::kilogram_per_second_cubed_degree_celsius>();

            fn test<
                M: m::Conversion<V>,
                T: t::Conversion<V>,
                TI: ti::Conversion<V>,
                HT: ht::Conversion<V>>()
            {
                Test::assert_approx_eq(&HeatTransfer::new::<HT>(V::one()),
                    &(Mass::new::<M>(V::one())
                        / (Time::new::<T>(V::one())
                            * Time::new::<T>(V::one())
                            * Time::new::<T>(V::one())
                            * TemperatureInterval::new::<TI>(V::one()))));
            }
        }

        #[test]
        fn check_power_per_area_ti_units() {
            test::<p::yottawatt, a::square_meter, ti::kelvin,
                ht::yottawatt_per_square_meter_kelvin>();
            test::<p::zettawatt, a::square_meter, ti::kelvin,
                ht::zettawatt_per_square_meter_kelvin>();
            test::<p::exawatt, a::square_meter, ti::kelvin, ht::exawatt_per_square_meter_kelvin>();
            test::<p::petawatt, a::square_meter, ti::kelvin,
                ht::petawatt_per_square_meter_kelvin>();
            test::<p::terawatt, a::square_meter, ti::kelvin,
                ht::terawatt_per_square_meter_kelvin>();
            test::<p::gigawatt, a::square_meter, ti::kelvin,
                ht::gigawatt_per_square_meter_kelvin>();
            test::<p::megawatt, a::square_meter, ti::kelvin,
                ht::megawatt_per_square_meter_kelvin>();
            test::<p::kilowatt, a::square_meter, ti::kelvin,
                ht::kilowatt_per_square_meter_kelvin>();
            test::<p::hectowatt, a::square_meter, ti::kelvin,
                ht::hectowatt_per_square_meter_kelvin>();
            test::<p::decawatt, a::square_meter, ti::kelvin,
                ht::decawatt_per_square_meter_kelvin>();
            test::<p::watt, a::square_meter, ti::kelvin, ht::watt_per_square_meter_kelvin>();
            test::<p::deciwatt, a::square_meter, ti::kelvin,
                ht::deciwatt_per_square_meter_kelvin>();
            test::<p::centiwatt, a::square_meter, ti::kelvin,
                ht::centiwatt_per_square_meter_kelvin>();
            test::<p::milliwatt, a::square_meter, ti::kelvin,
                ht::milliwatt_per_square_meter_kelvin>();
            test::<p::microwatt, a::square_meter, ti::kelvin,
                ht::microwatt_per_square_meter_kelvin>();
            test::<p::nanowatt, a::square_meter, ti::kelvin,
                ht::nanowatt_per_square_meter_kelvin>();
            test::<p::picowatt, a::square_meter, ti::kelvin,
                ht::picowatt_per_square_meter_kelvin>();
            test::<p::femtowatt, a::square_meter, ti::kelvin,
                ht::femtowatt_per_square_meter_kelvin>();
            test::<p::attowatt, a::square_meter, ti::kelvin,
                ht::attowatt_per_square_meter_kelvin>();
            test::<p::zeptowatt, a::square_meter, ti::kelvin,
                ht::zeptowatt_per_square_meter_kelvin>();
            test::<p::yoctowatt, a::square_meter, ti::kelvin,
                ht::yoctowatt_per_square_meter_kelvin>();

            test::<p::watt, a::square_kilometer, ti::degree_celsius,
                ht::watt_per_square_kilometer_degree_celsius>();
            test::<p::watt, a::square_meter, ti::degree_celsius,
                ht::watt_per_square_meter_degree_celsius>();
            test::<p::watt, a::square_centimeter, ti::degree_celsius,
                ht::watt_per_square_centimeter_degree_celsius>();
            test::<p::watt, a::square_millimeter, ti::degree_celsius,
                ht::watt_per_square_millimeter_degree_celsius>();

            fn test<
                P: p::Conversion<V>,
                A: a::Conversion<V>,
                TI: ti::Conversion<V>,
                HT: ht::Conversion<V>>()
            {
                Test::assert_approx_eq(&HeatTransfer::new::<HT>(V::one()),
                    &(Power::new::<P>(V::one())
                        / (Area::new::<A>(V::one()) * TemperatureInterval::new::<TI>(V::one()))));
            }
        }

        #[test]
        fn check_energy_per_time_area_ti_units() {
            test::<e::joule, t::second, a::square_meter, ti::kelvin,
                ht::joule_per_second_square_meter_kelvin>();
            test::<e::joule, t::second, a::square_meter, ti::degree_celsius,
                ht::joule_per_second_square_meter_degree_celsius>();
            test::<e::btu_it, t::hour, a::square_foot, ti::degree_fahrenheit,
                ht::btu_it_per_hour_square_foot_degree_fahrenheit>();
            test::<e::btu_it, t::hour, a::square_inch, ti::degree_fahrenheit,
                ht::btu_it_per_hour_square_inch_degree_fahrenheit>();
            test::<e::btu_it, t::minute, a::square_foot, ti::degree_fahrenheit,
                ht::btu_it_per_minute_square_foot_degree_fahrenheit>();
            test::<e::btu_it, t::minute, a::square_inch, ti::degree_fahrenheit,
                ht::btu_it_per_minute_square_inch_degree_fahrenheit>();
            test::<e::btu_it, t::second, a::square_foot, ti::degree_fahrenheit,
                ht::btu_it_per_second_square_foot_degree_fahrenheit>();
            test::<e::btu_it, t::second, a::square_inch, ti::degree_fahrenheit,
                ht::btu_it_per_second_square_inch_degree_fahrenheit>();
            test::<e::btu, t::hour, a::square_foot, ti::degree_fahrenheit,
                ht::btu_per_hour_square_foot_degree_fahrenheit>();
            test::<e::btu, t::hour, a::square_inch, ti::degree_fahrenheit,
                ht::btu_per_hour_square_inch_degree_fahrenheit>();
            test::<e::btu, t::minute, a::square_foot, ti::degree_fahrenheit,
                ht::btu_per_minute_square_foot_degree_fahrenheit>();
            test::<e::btu, t::minute, a::square_inch, ti::degree_fahrenheit,
                ht::btu_per_minute_square_inch_degree_fahrenheit>();
            test::<e::btu, t::second, a::square_foot, ti::degree_fahrenheit,
                ht::btu_per_second_square_foot_degree_fahrenheit>();
            test::<e::btu, t::second, a::square_inch, ti::degree_fahrenheit,
                ht::btu_per_second_square_inch_degree_fahrenheit>();

            fn test<
                E: e::Conversion<V>,
                T: t::Conversion<V>,
                A: a::Conversion<V>,
                TI: ti::Conversion<V>,
                HT: ht::Conversion<V>>()
            {
                Test::assert_approx_eq(&HeatTransfer::new::<HT>(V::one()),
                    &(Energy::new::<E>(V::one())
                        / (Time::new::<T>(V::one())
                            * Area::new::<A>(V::one())
                            * TemperatureInterval::new::<TI>(V::one()))));
            }
        }
    }
}
