//! Acceleration (base unit meter per second<sup>2</sup>, m<sup>1</sup> · s<sup>-2</sup>).

quantity! {
    /// Acceleration (base unit meter per second<sup>2</sup>, m<sup>1</sup> · s<sup>-2</sup>).
    quantity: Acceleration; "acceleration";
    /// Acceleration dimension, m<sup>1</sup> · s<sup>-2</sup>.
    dimension: ISQ<
        P1,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottameter_per_second_squared: prefix!(yotta); "Ym/s²", "yottameter per second squared",
            "yottameters per second squared";
        @zettameter_per_second_squared: prefix!(zetta); "Zm/s²", "zettameter per second squared",
            "zettameters per second squared";
        @exameter_per_second_squared: prefix!(exa); "Em/s²", "exameter per second squared",
            "exameters per second squared";
        @petameter_per_second_squared: prefix!(peta); "Pm/s²", "petameter per second squared",
            "petameters per second squared";
        @terameter_per_second_squared: prefix!(tera); "Tm/s²", "terameter per second squared",
            "terameters per second squared";
        @gigameter_per_second_squared: prefix!(giga); "Gm/s²", "gigameter per second squared",
            "gigameters per second squared";
        @megameter_per_second_squared: prefix!(mega); "Mm/s²", "megameter per second squared",
            "megameters per second squared";
        @kilometer_per_second_squared: prefix!(kilo); "km/s²", "kilometer per second squared",
            "kilometers per second squared";
        @hectometer_per_second_squared: prefix!(hecto); "hm/s²", "hectometer per second squared",
            "hectometers per second squared";
        @decameter_per_second_squared: prefix!(deca); "dam/s²", "decameter per second squared",
            "decameters per second squared";
        @meter_per_second_squared: prefix!(none); "m/s²", "meter per second squared",
            "meters per second squared";
        @decimeter_per_second_squared: prefix!(deci); "dm/s²", "decimeter per second squared",
            "decimeters per second squared";
        @centimeter_per_second_squared: prefix!(centi); "cm/s²", "centimeter per second squared",
            "centimeters per second squared";
        @millimeter_per_second_squared: prefix!(milli); "mm/s²", "millimeter per second squared",
            "millimeters per second squared";
        @micrometer_per_second_squared: prefix!(micro); "µm/s²", "micrometer per second squared",
            "micrometers per second squared";
        @nanometer_per_second_squared: prefix!(nano); "nm/s²", "nanometer per second squared",
            "nanometers per second squared";
        @picometer_per_second_squared: prefix!(pico); "pm/s²", "picometer per second squared",
            "picometers per second squared";
        @femtometer_per_second_squared: prefix!(femto); "fm/s²", "femtometer per second squared",
            "femtometers per second squared";
        @attometer_per_second_squared: prefix!(atto); "am/s²", "attometer per second squared",
            "attometers per second squared";
        @zeptometer_per_second_squared: prefix!(zepto); "zm/s²", "zeptometer per second squared",
            "zeptometers per second squared";
        @yoctometer_per_second_squared: prefix!(yocto); "ym/s²", "yoctometer per second squared",
            "yoctometers per second squared";

        @foot_per_second_squared: 3.048_E-1; "ft/s²", "foot per second squared", "feet per second squared";
        @galileo: 1.0_E-2; "Gal", "galileo", "galileos";
        @inch_per_second_squared: 2.54_E-2; "in/s²", "inch per second squared", "inches per second squared";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::acceleration as a;
        use si::length as l;
        use si::time as t;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Acceleration<V> = Length::new::<l::meter>(V::one()) /
                (Time::new::<t::second>(V::one()) * Time::new::<t::second>(V::one()));
        }

        #[test]
        fn check_units() {
            test::<l::yottameter, a::yottameter_per_second_squared>();
            test::<l::zettameter, a::zettameter_per_second_squared>();
            test::<l::exameter, a::exameter_per_second_squared>();
            test::<l::petameter, a::petameter_per_second_squared>();
            test::<l::terameter, a::terameter_per_second_squared>();
            test::<l::gigameter, a::gigameter_per_second_squared>();
            test::<l::megameter, a::megameter_per_second_squared>();
            test::<l::kilometer, a::kilometer_per_second_squared>();
            test::<l::hectometer, a::hectometer_per_second_squared>();
            test::<l::decameter, a::decameter_per_second_squared>();
            test::<l::meter, a::meter_per_second_squared>();
            test::<l::decimeter, a::decimeter_per_second_squared>();
            test::<l::centimeter, a::centimeter_per_second_squared>();
            test::<l::millimeter, a::millimeter_per_second_squared>();
            test::<l::micrometer, a::micrometer_per_second_squared>();
            test::<l::nanometer, a::nanometer_per_second_squared>();
            test::<l::picometer, a::picometer_per_second_squared>();
            test::<l::femtometer, a::femtometer_per_second_squared>();
            test::<l::attometer, a::attometer_per_second_squared>();
            test::<l::zeptometer, a::zeptometer_per_second_squared>();
            test::<l::yoctometer, a::yoctometer_per_second_squared>();

            fn test<L: l::Conversion<V>, A: a::Conversion<V>>() {
                Test::assert_eq(&Acceleration::new::<A>(V::one()),
                    &(Length::new::<L>(V::one()) /
                        (Time::new::<t::second>(V::one()) * Time::new::<t::second>(V::one()))));
            }
        }
    }
}
