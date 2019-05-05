//! Velocity (base unit meter per second, m · s⁻¹).

quantity! {
    /// Velocity (base unit meter per second, m · s⁻¹).
    quantity: Velocity; "velocity";
    /// Dimension of velocity, LT⁻¹ (base unit meter per second, m · s⁻¹).
    dimension: ISQ<
        P1,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottameter_per_second: prefix!(yotta); "Ym/s", "yottameter per second",
            "yottameters per second";
        @zettameter_per_second: prefix!(zetta); "Zm/s", "zettameter per second",
            "zettameters per second";
        @exameter_per_second: prefix!(exa); "Em/s", "exameter per second",
            "exameters per second";
        @petameter_per_second: prefix!(peta); "Pm/s", "petameter per second",
            "petameters per second";
        @terameter_per_second: prefix!(tera); "Tm/s", "terameter per second",
            "terameters per second";
        @gigameter_per_second: prefix!(giga); "Gm/s", "gigameter per second",
            "gigameters per second";
        @megameter_per_second: prefix!(mega); "Mm/s", "megameter per second",
            "megameters per second";
        @kilometer_per_second: prefix!(kilo); "km/s", "kilometer per second",
            "kilometers per second";
        @hectometer_per_second: prefix!(hecto); "hm/s", "hectometer per second",
            "hectometers per second";
        @decameter_per_second: prefix!(deca); "dam/s", "decameter per second",
            "decameters per second";
        @meter_per_second: prefix!(none); "m/s", "meter per second", "meters per second";
        @decimeter_per_second: prefix!(deci); "dm/s", "decimeter per second",
            "decimeters per second";
        @centimeter_per_second: prefix!(centi); "cm/s", "centimeter per second",
            "centimeters per second";
        @millimeter_per_second: prefix!(milli); "mm/s", "millimeter per second",
            "millimeters per second";
        @micrometer_per_second: prefix!(micro); "µm/s", "micrometer per second",
            "micrometers per second";
        @nanometer_per_second: prefix!(nano); "nm/s", "nanometer per second",
            "nanometers per second";
        @picometer_per_second: prefix!(pico); "pm/s", "picometer per second",
            "picometers per second";
        @femtometer_per_second: prefix!(femto); "fm/s", "femtometer per second",
            "femtometers per second";
        @attometer_per_second: prefix!(atto); "am/s", "attometer per second",
            "attometers per second";
        @zeptometer_per_second: prefix!(zepto); "zm/s", "zeptometer per second",
            "zeptometers per second";
        @yoctometer_per_second: prefix!(yocto); "ym/s", "yoctometer per second",
            "yoctometers per second";

        @foot_per_hour: 8.466_666_666_666_667_E-5; "ft/h", "foot per hour", "feet per hour";
        @foot_per_minute: 5.08_E-3; "ft/min", "foot per minute", "feet per minute";
        @foot_per_second: 3.048_E-1; "ft/s", "foot per second", "feet per second";
        @inch_per_second: 2.54_E-2; "in/s", "inch per second", "inches per second";
        @kilometer_per_hour: 2.777_777_777_777_778_E-1; "km/h", "kilometer per hour",
            "kilometers per hour";
        @knot: 5.144_444_444_444_445_E-1; "kn", "knot", "knots";
        @mile_per_hour: 4.470_4_E-1; "mi/h", "mile per hour", "miles per hour";
        @mile_per_minute: 2.682_24_E1; "mi/min", "mile per minute", "miles per minute";
        @mile_per_second: 1.609_344_E3; "mi/s", "mile per second", "miles per second";
        @millimeter_per_minute: 1.666_666_666_666_666_667_E-5; "mm/min", "millimeter per minute",
            "millimeters per minute";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::length as l;
        use si::time as t;
        use si::velocity as v;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Velocity<V> = Length::new::<l::meter>(V::one())
                / Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<l::yottameter, t::second, v::yottameter_per_second>();
            test::<l::zettameter, t::second, v::zettameter_per_second>();
            test::<l::exameter, t::second, v::exameter_per_second>();
            test::<l::petameter, t::second, v::petameter_per_second>();
            test::<l::terameter, t::second, v::terameter_per_second>();
            test::<l::gigameter, t::second, v::gigameter_per_second>();
            test::<l::megameter, t::second, v::megameter_per_second>();
            test::<l::kilometer, t::second, v::kilometer_per_second>();
            test::<l::hectometer, t::second, v::hectometer_per_second>();
            test::<l::decameter, t::second, v::decameter_per_second>();
            test::<l::meter, t::second, v::meter_per_second>();
            test::<l::decimeter, t::second, v::decimeter_per_second>();
            test::<l::centimeter, t::second, v::centimeter_per_second>();
            test::<l::millimeter, t::second, v::millimeter_per_second>();
            test::<l::micrometer, t::second, v::micrometer_per_second>();
            test::<l::nanometer, t::second, v::nanometer_per_second>();
            test::<l::picometer, t::second, v::picometer_per_second>();
            test::<l::femtometer, t::second, v::femtometer_per_second>();
            test::<l::attometer, t::second, v::attometer_per_second>();
            test::<l::zeptometer, t::second, v::zeptometer_per_second>();
            test::<l::yoctometer, t::second, v::yoctometer_per_second>();

            test::<l::foot, t::hour, v::foot_per_hour>();
            test::<l::foot, t::minute, v::foot_per_minute>();
            test::<l::foot, t::second, v::foot_per_second>();
            test::<l::inch, t::second, v::inch_per_second>();
            test::<l::kilometer, t::hour, v::kilometer_per_hour>();
            test::<l::nautical_mile, t::hour, v::knot>();
            test::<l::mile, t::hour, v::mile_per_hour>();
            test::<l::mile, t::second, v::mile_per_second>();
            test::<l::millimeter, t::minute, v::millimeter_per_minute>();

            fn test<L: l::Conversion<V>, T: t::Conversion<V>, E: v::Conversion<V>>() {
                Test::assert_eq(&Velocity::new::<E>(V::one()),
                    &(Length::new::<L>(V::one()) / Time::new::<T>(V::one())));
            }
        }
    }
}
