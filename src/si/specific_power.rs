//! Specific power (base unit watt per kilogram, m² · s⁻³).

quantity! {
    /// Specific power (base unit watt per kilogram, m² · s⁻³).
    quantity: SpecificPower; "specific power";
    /// Dimension of specific power, L²T⁻³ (base unit watt per kilogram, m² · s⁻³).
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        N3,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottawatt_per_kilogram: prefix!(yotta); "YW/kg", "yottawatt per kilogram",
            "yottawatts per kilogram";
        @zettawatt_per_kilogram: prefix!(zetta); "ZW/kg", "zettawatt per kilogram",
            "zettawatts per kilogram";
        @exawatt_per_kilogram: prefix!(exa); "EW/kg", "exawatt per kilogram",
            "exawatts per kilogram";
        @petawatt_per_kilogram: prefix!(peta); "PW/kg", "petawatt per kilogram",
            "petawatts per kilogram";
        @terawatt_per_kilogram: prefix!(tera); "TW/kg", "terawatt per kilogram",
            "terawatts per kilogram";
        @gigawatt_per_kilogram: prefix!(giga); "GW/kg", "gigawatt per kilogram",
            "gigawatts per kilogram";
        @megawatt_per_kilogram: prefix!(mega); "MW/kg", "megawatt per kilogram",
            "megawatts per kilogram";
        @kilowatt_per_kilogram: prefix!(kilo); "kW/kg", "kilowatt per kilogram",
            "kilowatts per kilogram";
        @hectowatt_per_kilogram: prefix!(hecto); "hW/kg", "hectowatt per kilogram",
            "hectowatts per kilogram";
        @decawatt_per_kilogram: prefix!(deca); "daW/kg", "decawatt per kilogram",
            "decawatts per kilogram";
        /// Derived unit of specific power.
        @watt_per_kilogram: prefix!(none); "W/kg", "watt per kilogram", "watts per kilogram";
        @deciwatt_per_kilogram: prefix!(deci); "dW/kg", "deciwatt per kilogram",
            "deciwatts per kilogram";
        @centiwatt_per_kilogram: prefix!(centi); "cW/kg", "centiwatt per kilogram",
            "centiwatts per kilogram";
        @milliwatt_per_kilogram: prefix!(milli); "mW/kg", "milliwatt per kilogram",
            "milliwatts per kilogram";
        @microwatt_per_kilogram: prefix!(micro); "µW/kg", "microwatt per kilogram",
            "microwatts per kilogram";
        @nanowatt_per_kilogram: prefix!(nano); "nW/kg", "nanowatt per kilogram",
            "nanowatts per kilogram";
        @picowatt_per_kilogram: prefix!(pico); "pW/kg", "picowatt per kilogram",
            "picowatts per kilogram";
        @femtowatt_per_kilogram: prefix!(femto); "fW/kg", "femtowatt per kilogram",
            "femtowatts per kilogram";
        @attowatt_per_kilogram: prefix!(atto); "aW/kg", "attowatt per kilogram",
            "attowatts per kilogram";
        @zeptowatt_per_kilogram: prefix!(zepto); "zW/kg", "zeptowatt per kilogram",
            "zeptowatts per kilogram";
        @yoctowatt_per_kilogram: prefix!(yocto); "yW/kg", "yoctowatt per kilogram",
            "yoctowatts per kilogram";

        /// Less commonly used, but more accurate (reduced) derived unit of specific power.
        @square_meter_per_cubic_second: 1.0_E0; "m² · s⁻³", "square meter per cubic second",
            "square meters per cubic second";

        @erg_per_second_per_kilogram: 1.0_E-7; "(erg/s)/kg", "erg per second per kilogram",
            "ergs per second per kilogram";
        @foot_pound_per_hour_per_pound: 3.766_161_111_111_111_E-4 / 4.535_924_E-1;
            "(ft · lbf/h)/lb", "foot pound-force per hour per pound",
            "foot pounds-force per hour per pound";
        @foot_pound_per_minute_per_pound: 2.259_696_666_666_666_6_E-2 / 4.535_924_E-1;
            "(ft · lbf/min)/lb", "foot pound-force per minute per pound",
            "foot pounds-force per minute per pound";
        @foot_pound_per_second_per_pound: 1.355_818 / 4.535_924_E-1; "(ft · lbf/s)/lb",
            "foot pound-force per second per pound", "foot pounds-force per second per pound";

        @horsepower_per_pound: 7.456_999_E2 / 4.535_924_E-1; "hp/lb",
            "horsepower per pound", "horsepower per pound";
        @horsepower_boiler_per_pound: 9.809_50_E3 / 4.535_924_E-1; "(hp (S))/lb",
            "horsepower (boiler) per pound", "horsepower (boiler) per pound";
        @horsepower_electric_per_pound: 7.46_E2 / 4.535_924_E-1; "(hp (E))/lb",
            "horsepower (electric) per pound", "horsepower (electric) per pound";
        @horsepower_metric_per_pound: 7.354_988_E2 / 4.535_924_E-1; "(hp (M))/lb",
            "metric horsepower per pound", "metric horsepower per pound";
        @horsepower_imperial_per_pound: 7.457_0_E2 / 4.535_924_E-1; "(hp (I))/lb",
            "horsepower (Imperial) per pound", "horsepower (Imperial) per pound";
        @hydraulic_horsepower_per_pound: 7.460_43_E2 / 4.535_924_E-1; "(hp (hydraulic))/lb",
            "hydraulic horsepower per pound", "hydraulic horsepower per pound";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::energy as e;
        use crate::si::mass as m;
        use crate::si::power as p;
        use crate::si::quantities::*;
        use crate::si::specific_power as sp;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: SpecificPower<V> = Power::new::<p::watt>(V::one())
                / Mass::new::<m::kilogram>(V::one());
        }

        #[test]
        fn check_units_power() {
            test::<p::zettawatt, m::kilogram, sp::zettawatt_per_kilogram>();
            test::<p::exawatt, m::kilogram, sp::exawatt_per_kilogram>();
            test::<p::petawatt, m::kilogram, sp::petawatt_per_kilogram>();
            test::<p::terawatt, m::kilogram, sp::terawatt_per_kilogram>();
            test::<p::gigawatt, m::kilogram, sp::gigawatt_per_kilogram>();
            test::<p::megawatt, m::kilogram, sp::megawatt_per_kilogram>();
            test::<p::kilowatt, m::kilogram, sp::kilowatt_per_kilogram>();
            test::<p::watt, m::kilogram, sp::watt_per_kilogram>();
            test::<p::centiwatt, m::kilogram, sp::centiwatt_per_kilogram>();
            test::<p::milliwatt, m::kilogram, sp::milliwatt_per_kilogram>();
            test::<p::microwatt, m::kilogram, sp::microwatt_per_kilogram>();
            test::<p::nanowatt, m::kilogram, sp::nanowatt_per_kilogram>();
            test::<p::picowatt, m::kilogram, sp::picowatt_per_kilogram>();
            test::<p::femtowatt, m::kilogram, sp::femtowatt_per_kilogram>();
            test::<p::attowatt, m::kilogram, sp::attowatt_per_kilogram>();
            test::<p::zeptowatt, m::kilogram, sp::zeptowatt_per_kilogram>();
            test::<p::yoctowatt, m::kilogram, sp::yoctowatt_per_kilogram>();

            test::<p::watt, m::kilogram, sp::square_meter_per_cubic_second>();

            test::<p::horsepower, m::pound, sp::horsepower_per_pound>();
            test::<p::horsepower_boiler, m::pound, sp::horsepower_boiler_per_pound>();
            test::<p::horsepower_electric, m::pound, sp::horsepower_electric_per_pound>();
            test::<p::horsepower_metric, m::pound, sp::horsepower_metric_per_pound>();
            test::<p::horsepower_imperial, m::pound, sp::horsepower_imperial_per_pound>();
            test::<p::hydraulic_horsepower, m::pound, sp::hydraulic_horsepower_per_pound>();

            fn test<P: p::Conversion<V>, M: m::Conversion<V>, SP: sp::Conversion<V>>() {
                Test::assert_approx_eq(&SpecificPower::new::<SP>(V::one()),
                    &(Power::new::<P>(V::one()) / Mass::new::<M>(V::one())));
            }
        }

        #[test]
        fn check_units_energy() {
            test::<e::erg, t::second, m::kilogram, sp::erg_per_second_per_kilogram>();
            test::<e::foot_pound, t::hour, m::pound, sp::foot_pound_per_hour_per_pound>();
            test::<e::foot_pound, t::minute, m::pound, sp::foot_pound_per_minute_per_pound>();
            test::<e::foot_pound, t::second, m::pound, sp::foot_pound_per_second_per_pound>();

            fn test<E: e::Conversion<V>, T: t::Conversion<V>, M: m::Conversion<V>,
                SP: sp::Conversion<V>>()
            {
                Test::assert_approx_eq(&SpecificPower::new::<SP>(V::one()),
                    &((Energy::new::<E>(V::one())  / Time::new::<T>(V::one()))
                        / Mass::new::<M>(V::one())));
            }
        }
    }
}
