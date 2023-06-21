//! Power rate (base unit watt per second, m² · kg · s⁻⁴).

quantity! {
    /// Power rate (base unit watt per second, m² · kg · s⁻⁴).
    quantity: PowerRate; "power rate";
    /// Dimension of power rate, L²MT⁻⁴ (base unit watt per second, m² · kg · s⁻⁴).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N4,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottawatt_per_second: prefix!(yotta); "YW/s", "yottawatt per second",
            "yottawatts per second";
        @zettawatt_per_second: prefix!(zetta); "ZW/s", "zettawatt per second",
            "zettawatts per second";
        @exawatt_per_second: prefix!(exa); "EW/s", "exawatt per second", "exawatts per second";
        @petawatt_per_second: prefix!(peta); "PW/s", "petawatt per second", "petawatts per second";
        @terawatt_per_second: prefix!(tera); "TW/s", "terawatt per second", "terawatts per second";
        @gigawatt_per_second: prefix!(giga); "GW/s", "gigawatt per second", "gigawatts per second";
        @megawatt_per_second: prefix!(mega); "MW/s", "megawatt per second", "megawatts per second";
        @kilowatt_per_second: prefix!(kilo); "kW/s", "kilowatt per second", "kilowatts per second";
        @hectowatt_per_second: prefix!(hecto); "hW/s", "hectowatt per second",
            "hectowatts per second";
        @decawatt_per_second: prefix!(deca); "daW/s", "decawatt per second", "decawatts per second";
        /// Derived unit of power rate.
        @watt_per_second: prefix!(none); "W/s", "watt per second", "watts per second";
        @deciwatt_per_second: prefix!(deci); "dW/s", "deciwatt per second", "deciwatts per second";
        @centiwatt_per_second: prefix!(centi); "cW/s", "centiwatt per second",
            "centiwatts per second";
        @milliwatt_per_second: prefix!(milli); "mW/s", "milliwatt per second",
            "milliwatts per second";
        @microwatt_per_second: prefix!(micro); "µW/s", "microwatt per second",
            "microwatts per second";
        @nanowatt_per_second: prefix!(nano); "nW/s", "nanowatt per second", "nanowatts per second";
        @picowatt_per_second: prefix!(pico); "pW/s", "picowatt per second", "picowatts per second";
        @femtowatt_per_second: prefix!(femto); "fW/s", "femtowatt per second",
            "femtowatts per second";
        @attowatt_per_second: prefix!(atto); "aW/s", "attowatt per second", "attowatts per second";
        @zeptowatt_per_second: prefix!(zepto); "zW/s", "zeptowatt per second",
            "zeptowatts per second";
        @yoctowatt_per_second: prefix!(yocto); "yW/s", "yoctowatt per second",
            "yoctowatts per second";

        @erg_per_second_squared: 1.0_E-7; "erg/s²", "erg per second squared",
            "ergs per second squared";
        @foot_pound_per_second_squared: 1.355_818; "ft · lbf/s²",
            "foot pound-force per second squared", "foot pounds-force per second squared";
        @horsepower_per_second: 7.456_999_E2; "hp/s", "horsepower per second",
            "horsepower per second";
        @horsepower_per_second_boiler: 9.809_50_E3; "hp/s (S)", "horsepower per second (boiler)",
            "horsepower per second (boiler)";
        @horsepower_per_second_electric: 7.46_E2; "hp/s (E)", "horsepower per second (electric)",
            "horsepower per second (electric)";
        @horsepower_per_second_metric: 7.354_988_E2; "hp/s (M)", "metric horsepower per second",
            "metric horsepower per second";
        @horsepower_per_second_imperial: 7.457_0_E2; "hp/s (I)", "horsepower per second (Imperial)",
            "horsepower per second (Imperial)";
        @hydraulic_horsepower_per_second: 7.460_43_E2; "hp/s (hydraulic)",
            "hydraulic horsepower per second", "hydraulic horsepower per second";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::power as p;
        use crate::si::power_rate as r;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: PowerRate<V> = Power::new::<p::watt>(V::one())
                / Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<p::yottawatt, t::second, r::yottawatt_per_second>();
            test::<p::zettawatt, t::second, r::zettawatt_per_second>();
            test::<p::exawatt, t::second, r::exawatt_per_second>();
            test::<p::petawatt, t::second, r::petawatt_per_second>();
            test::<p::terawatt, t::second, r::terawatt_per_second>();
            test::<p::gigawatt, t::second, r::gigawatt_per_second>();
            test::<p::megawatt, t::second, r::megawatt_per_second>();
            test::<p::kilowatt, t::second, r::kilowatt_per_second>();
            test::<p::hectowatt, t::second, r::hectowatt_per_second>();
            test::<p::decawatt, t::second, r::decawatt_per_second>();
            test::<p::watt, t::second, r::watt_per_second>();
            test::<p::deciwatt, t::second, r::deciwatt_per_second>();
            test::<p::centiwatt, t::second, r::centiwatt_per_second>();
            test::<p::milliwatt, t::second, r::milliwatt_per_second>();
            test::<p::microwatt, t::second, r::microwatt_per_second>();
            test::<p::nanowatt, t::second, r::nanowatt_per_second>();
            test::<p::picowatt, t::second, r::picowatt_per_second>();
            test::<p::femtowatt, t::second, r::femtowatt_per_second>();
            test::<p::attowatt, t::second, r::attowatt_per_second>();
            test::<p::zeptowatt, t::second, r::zeptowatt_per_second>();
            test::<p::yoctowatt, t::second, r::yoctowatt_per_second>();

            test::<p::erg_per_second, t::second, r::erg_per_second_squared>();
            test::<p::foot_pound_per_second, t::second, r::foot_pound_per_second_squared>();
            test::<p::horsepower, t::second, r::horsepower_per_second>();
            test::<p::horsepower_boiler, t::second, r::horsepower_per_second_boiler>();
            test::<p::horsepower_electric, t::second, r::horsepower_per_second_electric>();
            test::<p::horsepower_metric, t::second, r::horsepower_per_second_metric>();
            test::<p::horsepower_imperial, t::second, r::horsepower_per_second_imperial>();
            test::<p::hydraulic_horsepower, t::second, r::hydraulic_horsepower_per_second>();

            fn test<P: p::Conversion<V>, T: t::Conversion<V>, R: r::Conversion<V>>() {
                Test::assert_approx_eq(&PowerRate::new::<R>(V::one()),
                    &(Power::new::<P>(V::one()) / Time::new::<T>(V::one())));
            }
        }
    }
}
