//! Heat capacity (base unit joule per kelvin, kg · m² · s⁻² · K⁻¹).
//!
//! Heat capacity has the same kind as [temperature interval][ti], as this quantity relates to
//! change of temperature. Not of kind `TemperatureKind`, used by [thermodynamic temperature][tt].
//! See [thermodynamic temperature][tt] for a full explanation.
//!
//! This quantity might be used to define the heat capacity of an object. To define heat capacity
//! of a material, use [specific heat capacity][shc].
//!
//! [ti]: ../temperature_interval/index.html
//! [tt]: ../thermodynamic_temperature/index.html
//! [shc]: ../specific_heat_capacity/index.html

quantity! {
    /// Heat capacity (base unit joule per kelvin, kg · m² · s⁻² · K⁻¹).
    quantity: HeatCapacity; "heat capacity";
    /// Dimension of heat capacity, L²MT⁻²Th⁻¹ (base unit joule per kelvin, kg · m² · s⁻² · K⁻¹).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        N1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottagram_square_meter_per_second_squared_kelvin: prefix!(yotta) / prefix!(kilo);
            "Yg · m²/(s² · K)",
            "yottagram square meter per second squared kelvin",
            "yottagram square meters per second squared kelvin";
        @zettagram_square_meter_per_second_squared_kelvin: prefix!(zetta) / prefix!(kilo);
            "Zg · m²/(s² · K)",
            "zettagram square meter per second squared kelvin",
            "zettagram square meters per second squared kelvin";
        @exagram_square_meter_per_second_squared_kelvin: prefix!(exa) / prefix!(kilo);
            "Eg · m²/(s² · K)",
            "exagram square meter per second squared kelvin",
            "exagram square meters per second squared kelvin";
        @petagram_square_meter_per_second_squared_kelvin: prefix!(peta) / prefix!(kilo);
            "Pg · m²/(s² · K)",
            "petagram square meter per second squared kelvin",
            "petagram square meters per second squared kelvin";
        @teragram_square_meter_per_second_squared_kelvin: prefix!(tera) / prefix!(kilo);
            "Tg · m²/(s² · K)",
            "teragram square meter per second squared kelvin",
            "teragram square meters per second squared kelvin";
        @gigagram_square_meter_per_second_squared_kelvin: prefix!(giga) / prefix!(kilo);
            "Gg · m²/(s² · K)",
            "gigagram square meter per second squared kelvin",
            "gigagram square meters per second squared kelvin";
        @megagram_square_meter_per_second_squared_kelvin: prefix!(mega) / prefix!(kilo);
            "Mg · m²/(s² · K)",
            "megagram square meter per second squared kelvin",
            "megagram square meters per second squared kelvin";
        /// The derived unit of heat capacity expressed in base units. Equivalent to J/K.
        @kilogram_square_meter_per_second_squared_kelvin: prefix!(kilo) / prefix!(kilo);
            "kg · m²/(s² · K)",
            "kilogram square meter per second squared kelvin",
            "kilogram square meters per second squared kelvin";
        @hectogram_square_meter_per_second_squared_kelvin: prefix!(hecto) / prefix!(kilo);
            "hg · m²/(s² · K)",
            "hectogram square meter per second squared kelvin",
            "hectogram square meters per second squared kelvin";
        @decagram_square_meter_per_second_squared_kelvin: prefix!(deca) / prefix!(kilo);
            "dag · m²/(s² · K)",
            "decagram square meter per second squared kelvin",
            "decagram square meters per second squared kelvin";
        @gram_square_meter_per_second_squared_kelvin: prefix!(none) / prefix!(kilo);
            "g · m²/(s² · K)",
            "gram square meter per second squared kelvin",
            "gram square meters per second squared kelvin";
        @decigram_square_meter_per_second_squared_kelvin: prefix!(deci) / prefix!(kilo);
            "dg · m²/(s² · K)",
            "decigram square meter per second squared kelvin",
            "decigram square meters per second squared kelvin";
        @centigram_square_meter_per_second_squared_kelvin: prefix!(centi) / prefix!(kilo);
            "cg · m²/(s² · K)",
            "centigram square meter per second squared kelvin",
            "centigram square meters per second squared kelvin";
        @milligram_square_meter_per_second_squared_kelvin: prefix!(milli) / prefix!(kilo);
            "mg · m²/(s² · K)",
            "milligram square meter per second squared kelvin",
            "milligram square meters per second squared kelvin";
        @microgram_square_meter_per_second_squared_kelvin: prefix!(micro) / prefix!(kilo);
            "µg · m²/(s² · K)",
            "microgram square meter per second squared kelvin",
            "microgram square meters per second squared kelvin";
        @nanogram_square_meter_per_second_squared_kelvin: prefix!(nano) / prefix!(kilo);
            "ng · m²/(s² · K)",
            "nanogram square meter per second squared kelvin",
            "nanogram square meters per second squared kelvin";
        @picogram_square_meter_per_second_squared_kelvin: prefix!(pico) / prefix!(kilo);
            "pg · m²/(s² · K)",
            "picogram square meter per second squared kelvin",
            "picogram square meters per second squared kelvin";
        @femtogram_square_meter_per_second_squared_kelvin: prefix!(femto) / prefix!(kilo);
            "fg · m²/(s² · K)",
            "femtogram square meter per second squared kelvin",
            "femtogram square meters per second squared kelvin";
        @attogram_square_meter_per_second_squared_kelvin: prefix!(atto) / prefix!(kilo);
            "ag · m²/(s² · K)",
            "attogram square meter per second squared kelvin",
            "attogram square meters per second squared kelvin";
        @zeptogram_square_meter_per_second_squared_kelvin: prefix!(zepto) / prefix!(kilo);
            "zg · m²/(s² · K)",
            "zeptogram square meter per second squared kelvin",
            "zeptogram square meters per second squared kelvin";
        @yoctogram_square_meter_per_second_squared_kelvin: prefix!(yocto) / prefix!(kilo);
            "yg · m²/(s² · K)",
            "yoctogram square meter per second squared kelvin",
            "yoctogram square meters per second squared kelvin";

        // Heat capacity is much more commonly expressed in terms of energy / temperature.
        @yottajoule_per_kelvin: prefix!(yotta); "YJ/K", "yottajoule per kelvin",
            "yottajoules per kelvin";
        @zettajoule_per_kelvin: prefix!(zetta); "ZJ/K", "zettajoule per kelvin",
            "zettajoules per kelvin";
        @exajoule_per_kelvin: prefix!(exa); "EJ/K", "exajoule per kelvin", "exajoules per kelvin";
        @petajoule_per_kelvin: prefix!(peta); "PJ/K", "petajoule per kelvin",
            "petajoules per kelvin";
        @terajoule_per_kelvin: prefix!(tera); "TJ/K", "terajoule per kelvin",
            "terajoules per kelvin";
        @gigajoule_per_kelvin: prefix!(giga); "GJ/K", "gigajoule per kelvin",
            "gigajoules per kelvin";
        @megajoule_per_kelvin: prefix!(mega); "MJ/K", "megajoule per kelvin",
            "megajoules per kelvin";
        @kilojoule_per_kelvin: prefix!(kilo); "kJ/K", "kilojoule per kelvin",
            "kilojoules per kelvin";
        @hectojoule_per_kelvin: prefix!(hecto); "hJ/K", "hectojoule per kelvin",
            "hectojoules per kelvin";
        @decajoule_per_kelvin: prefix!(deca); "daJ/K", "decajoule per kelvin",
            "decajoules per kelvin";
        /// Derived unit of heat capacity expressed in terms of derived unit Joule. Equivalent to
        /// kg · m²/(s² · K).
        @joule_per_kelvin: prefix!(none); "J/K", "joule per kelvin", "joules per kelvin";
        @decijoule_per_kelvin: prefix!(deci); "dJ/K", "decijoule per kelvin",
            "decijoules per kelvin";
        @centijoule_per_kelvin: prefix!(centi); "cJ/K", "centijoule per kelvin",
            "centijoules per kelvin";
        @millijoule_per_kelvin: prefix!(milli); "mJ/K", "millijoule per kelvin",
            "millijoules per kelvin";
        @microjoule_per_kelvin: prefix!(micro); "µJ/K", "microjoule per kelvin",
            "microjoules per kelvin";
        @nanojoule_per_kelvin: prefix!(nano); "nJ/K", "nanojoule per kelvin",
            "nanojoules per kelvin";
        @picojoule_per_kelvin: prefix!(pico); "pJ/K", "picojoule per kelvin",
            "picojoules per kelvin";
        @femtojoule_per_kelvin: prefix!(femto); "fJ/K", "femtojoule per kelvin",
            "femtojoules per kelvin";
        @attojoule_per_kelvin: prefix!(atto); "aJ/K", "attojoule per kelvin",
            "attojoules per kelvin";
        @zeptojoule_per_kelvin: prefix!(zepto); "zJ/K", "zeptojoule per kelvin",
            "zeptojoules per kelvin";
        @yoctojoule_per_kelvin: prefix!(yocto); "yJ/K", "yoctojoule per kelvin",
            "yoctojoules per kelvin";

        @kilojoule_per_degree_celsius: 1.0_E3; "kJ/°C", "kilojoule per degree celsius",
            "kilojoules per degree celsius";
        @joule_per_degree_celsius: 1.0_E0; "J/°C", "joule per degree celsius",
            "joules per degree celsius";
        @millijoule_per_degree_celsius: 1.0_E-3; "mJ/°C", "millijoule per degree celsius",
            "millijoules per degree celsius";

        @btu_per_degree_fahrenheit: 1.897_830_E3; "Btu/°F",
            "British thermal unit per degree Fahrenheit",
            "British thermal units per degree Fahrenheit";
        @btu_it_per_degree_fahrenheit: 1.899_100_8_E3; "Btu (IT)/°F",
            "British thermal unit (IT) per degree Fahrenheit",
            "British thermal units (IT) per degree Fahrenheit";
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
        use si::heat_capacity as hc;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: HeatCapacity<V> = Mass::new::<m::kilogram>(V::one())
                * Length::new::<l::meter>(V::one())
                * Length::new::<l::meter>(V::one())
                / (Time::new::<t::second>(V::one())
                    * Time::new::<t::second>(V::one())
                    * TemperatureInterval::new::<ti::kelvin>(V::one()));
        }

        #[test]
        fn check_base_units() {
            test::<m::yottagram, hc::yottagram_square_meter_per_second_squared_kelvin>();
            test::<m::zettagram, hc::zettagram_square_meter_per_second_squared_kelvin>();
            test::<m::exagram, hc::exagram_square_meter_per_second_squared_kelvin>();
            test::<m::petagram, hc::petagram_square_meter_per_second_squared_kelvin>();
            test::<m::teragram, hc::teragram_square_meter_per_second_squared_kelvin>();
            test::<m::gigagram, hc::gigagram_square_meter_per_second_squared_kelvin>();
            test::<m::megagram, hc::megagram_square_meter_per_second_squared_kelvin>();
            test::<m::kilogram, hc::kilogram_square_meter_per_second_squared_kelvin>();
            test::<m::hectogram, hc::hectogram_square_meter_per_second_squared_kelvin>();
            test::<m::decagram, hc::decagram_square_meter_per_second_squared_kelvin>();
            test::<m::gram, hc::gram_square_meter_per_second_squared_kelvin>();
            test::<m::decigram, hc::decigram_square_meter_per_second_squared_kelvin>();
            test::<m::centigram, hc::centigram_square_meter_per_second_squared_kelvin>();
            test::<m::milligram, hc::milligram_square_meter_per_second_squared_kelvin>();
            test::<m::microgram, hc::microgram_square_meter_per_second_squared_kelvin>();
            test::<m::nanogram, hc::nanogram_square_meter_per_second_squared_kelvin>();
            test::<m::picogram, hc::picogram_square_meter_per_second_squared_kelvin>();
            test::<m::femtogram, hc::femtogram_square_meter_per_second_squared_kelvin>();
            test::<m::attogram, hc::attogram_square_meter_per_second_squared_kelvin>();
            test::<m::zeptogram, hc::zeptogram_square_meter_per_second_squared_kelvin>();
            test::<m::yoctogram, hc::yoctogram_square_meter_per_second_squared_kelvin>();

            fn test<M: m::Conversion<V>, HC: hc::Conversion<V>>() {
                Test::assert_approx_eq(&HeatCapacity::new::<HC>(V::one()),
                    &(Mass::new::<M>(V::one())
                        * Length::new::<l::meter>(V::one())
                        * Length::new::<l::meter>(V::one())
                        / (Time::new::<t::second>(V::one())
                            * Time::new::<t::second>(V::one())
                            * TemperatureInterval::new::<ti::kelvin>(V::one()))));
            }
        }

        #[test]
        fn check_energy_per_ti_hc_units() {
            test::<e::yottajoule, ti::kelvin, hc::yottajoule_per_kelvin>();
            test::<e::zettajoule, ti::kelvin, hc::zettajoule_per_kelvin>();
            test::<e::exajoule, ti::kelvin, hc::exajoule_per_kelvin>();
            test::<e::petajoule, ti::kelvin, hc::petajoule_per_kelvin>();
            test::<e::terajoule, ti::kelvin, hc::terajoule_per_kelvin>();
            test::<e::gigajoule, ti::kelvin, hc::gigajoule_per_kelvin>();
            test::<e::megajoule, ti::kelvin, hc::megajoule_per_kelvin>();
            test::<e::kilojoule, ti::kelvin, hc::kilojoule_per_kelvin>();
            test::<e::hectojoule, ti::kelvin, hc::hectojoule_per_kelvin>();
            test::<e::decajoule, ti::kelvin, hc::decajoule_per_kelvin>();
            test::<e::joule, ti::kelvin, hc::joule_per_kelvin>();
            test::<e::decijoule, ti::kelvin, hc::decijoule_per_kelvin>();
            test::<e::centijoule, ti::kelvin, hc::centijoule_per_kelvin>();
            test::<e::millijoule, ti::kelvin, hc::millijoule_per_kelvin>();
            test::<e::microjoule, ti::kelvin, hc::microjoule_per_kelvin>();
            test::<e::nanojoule, ti::kelvin, hc::nanojoule_per_kelvin>();
            test::<e::picojoule, ti::kelvin, hc::picojoule_per_kelvin>();
            test::<e::femtojoule, ti::kelvin, hc::femtojoule_per_kelvin>();
            test::<e::attojoule, ti::kelvin, hc::attojoule_per_kelvin>();
            test::<e::zeptojoule, ti::kelvin, hc::zeptojoule_per_kelvin>();
            test::<e::yoctojoule, ti::kelvin, hc::yoctojoule_per_kelvin>();

            test::<e::kilojoule, ti::degree_celsius, hc::kilojoule_per_degree_celsius>();
            test::<e::joule, ti::degree_celsius, hc::joule_per_degree_celsius>();
            test::<e::millijoule, ti::degree_celsius, hc::millijoule_per_degree_celsius>();

            test::<e::btu, ti::degree_fahrenheit, hc::btu_per_degree_fahrenheit>();
            test::<e::btu_it, ti::degree_fahrenheit, hc::btu_it_per_degree_fahrenheit>();

            fn test<E: e::Conversion<V>, TI: ti::Conversion<V>, HC: hc::Conversion<V>>() {
                Test::assert_approx_eq(&HeatCapacity::new::<HC>(V::one()),
                    &(Energy::new::<E>(V::one())
                        / TemperatureInterval::new::<TI>(V::one())));
            }
        }
    }
}
