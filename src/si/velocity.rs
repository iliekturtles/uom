//! Velocity (base unit meter per second, m<sup>1</sup> · s<sup>-1</sup>).

quantity! {
    /// Velocity (base unit meter per second, m<sup>1</sup> · s<sup>-1</sup>).
    quantity: Velocity; "velocity";
    /// Velocity dimension, m<sup>1</sup> · s<sup>-1</sup>.
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
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        types: Float;

        use num::One;
        use si::quantities::*;
        use si::length as l;
        use si::time as t;
        use si::velocity as v;

        #[test]
        fn check_dimension() {
            let _: Velocity<V> = Length::new::<l::meter>(V::one()) / Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test(l::yottameter, v::yottameter_per_second);
            test(l::zettameter, v::zettameter_per_second);
            test(l::exameter, v::exameter_per_second);
            test(l::petameter, v::petameter_per_second);
            test(l::terameter, v::terameter_per_second);
            test(l::gigameter, v::gigameter_per_second);
            test(l::megameter, v::megameter_per_second);
            test(l::kilometer, v::kilometer_per_second);
            test(l::hectometer, v::hectometer_per_second);
            test(l::decameter, v::decameter_per_second);
            test(l::meter, v::meter_per_second);
            test(l::decimeter, v::decimeter_per_second);
            test(l::centimeter, v::centimeter_per_second);
            test(l::millimeter, v::millimeter_per_second);
            test(l::micrometer, v::micrometer_per_second);
            test(l::nanometer, v::nanometer_per_second);
            test(l::picometer, v::picometer_per_second);
            test(l::femtometer, v::femtometer_per_second);
            test(l::attometer, v::attometer_per_second);
            test(l::zeptometer, v::zeptometer_per_second);
            test(l::yoctometer, v::yoctometer_per_second);

            // TODO #17 Convert to == once PartialEq is implemented.
            fn test<L: l::Unit<V>, E: v::Unit<V>>(_l: L, v: E) {
                assert_eq!(V::one(), (Length::new::<L>(V::one()) / Time::new::<t::second>(V::one())).get(v));
            }
        }
    }
}
