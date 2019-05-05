//! Jerk (base unit meter per second cubed, m · s⁻³).

quantity! {
    /// Jerk (base unit meter per second cubed, m · s⁻³).
    quantity: Jerk; "jerk";
    /// Dimension of jerk, LT⁻³ (base unit meter per second cubed, m · s⁻³).
    dimension: ISQ<
        P1,     // length
        Z0,     // mass
        N3,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottameter_per_second_cubed: prefix!(yotta); "Ym/s³", "yottameter per second cubed",
            "yottameters per second cubed";
        @zettameter_per_second_cubed: prefix!(zetta); "Zm/s³", "zettameter per second cubed",
            "zettameters per second cubed";
        @exameter_per_second_cubed: prefix!(exa); "Em/s³", "exameter per second cubed",
            "exameters per second cubed";
        @petameter_per_second_cubed: prefix!(peta); "Pm/s³", "petameter per second cubed",
            "petameters per second cubed";
        @terameter_per_second_cubed: prefix!(tera); "Tm/s³", "terameter per second cubed",
            "terameters per second cubed";
        @gigameter_per_second_cubed: prefix!(giga); "Gm/s³", "gigameter per second cubed",
            "gigameters per second cubed";
        @megameter_per_second_cubed: prefix!(mega); "Mm/s³", "megameter per second cubed",
            "megameters per second cubed";
        @kilometer_per_second_cubed: prefix!(kilo); "km/s³", "kilometer per second cubed",
            "kilometers per second cubed";
        @hectometer_per_second_cubed: prefix!(hecto); "hm/s³", "hectometer per second cubed",
            "hectometers per second cubed";
        @decameter_per_second_cubed: prefix!(deca); "dam/s³", "decameter per second cubed",
            "decameters per second cubed";
        @meter_per_second_cubed: prefix!(none); "m/s³", "meter per second cubed",
            "meters per second cubed";
        @decimeter_per_second_cubed: prefix!(deci); "dm/s³", "decimeter per second cubed",
            "decimeters per second cubed";
        @centimeter_per_second_cubed: prefix!(centi); "cm/s³", "centimeter per second cubed",
            "centimeters per second cubed";
        @millimeter_per_second_cubed: prefix!(milli); "mm/s³", "millimeter per second cubed",
            "millimeters per second cubed";
        @micrometer_per_second_cubed: prefix!(micro); "µm/s³", "micrometer per second cubed",
            "micrometers per second cubed";
        @nanometer_per_second_cubed: prefix!(nano); "nm/s³", "nanometer per second cubed",
            "nanometers per second cubed";
        @picometer_per_second_cubed: prefix!(pico); "pm/s³", "picometer per second cubed",
            "picometers per second cubed";
        @femtometer_per_second_cubed: prefix!(femto); "fm/s³", "femtometer per second cubed",
            "femtometers per second cubed";
        @attometer_per_second_cubed: prefix!(atto); "am/s³", "attometer per second cubed",
            "attometers per second cubed";
        @zeptometer_per_second_cubed: prefix!(zepto); "zm/s³", "zeptometer per second cubed",
            "zeptometers per second cubed";
        @yoctometer_per_second_cubed: prefix!(yocto); "ym/s³", "yoctometer per second cubed",
            "yoctometers per second cubed";

        @foot_per_second_cubed: 3.048_E-1; "ft/s³", "foot per second cubed",
            "feet per second cubed";
        @inch_per_second_cubed: 2.54_E-2; "in/s³", "inch per second cubed",
            "inches per second cubed";
        @kilometer_per_minute_cubed: 4.629_629_629_629_629_E-3; "km/min³",
            "kilometer per minute cubed", "kilometers per minute cubed";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::jerk as j;
        use si::length as l;
        use si::time as t;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Jerk<V> = Length::new::<l::meter>(V::one())
                / (Time::new::<t::second>(V::one())
                    * Time::new::<t::second>(V::one())
                    * Time::new::<t::second>(V::one()));
        }
        
        #[test]
        fn check_units() {
            test::<l::yottameter, t::second, j::yottameter_per_second_cubed>();
            test::<l::zettameter, t::second, j::zettameter_per_second_cubed>();
            test::<l::exameter, t::second, j::exameter_per_second_cubed>();
            test::<l::petameter, t::second, j::petameter_per_second_cubed>();
            test::<l::terameter, t::second, j::terameter_per_second_cubed>();
            test::<l::gigameter, t::second, j::gigameter_per_second_cubed>();
            test::<l::megameter, t::second, j::megameter_per_second_cubed>();
            test::<l::kilometer, t::second, j::kilometer_per_second_cubed>();
            test::<l::hectometer, t::second, j::hectometer_per_second_cubed>();
            test::<l::decameter, t::second, j::decameter_per_second_cubed>();
            test::<l::meter, t::second, j::meter_per_second_cubed>();
            test::<l::decimeter, t::second, j::decimeter_per_second_cubed>();
            test::<l::centimeter, t::second, j::centimeter_per_second_cubed>();
            test::<l::millimeter, t::second, j::millimeter_per_second_cubed>();
            test::<l::micrometer, t::second, j::micrometer_per_second_cubed>();
            test::<l::nanometer, t::second, j::nanometer_per_second_cubed>();
            test::<l::picometer, t::second, j::picometer_per_second_cubed>();
            test::<l::femtometer, t::second, j::femtometer_per_second_cubed>();
            test::<l::attometer, t::second, j::attometer_per_second_cubed>();
            test::<l::zeptometer, t::second, j::zeptometer_per_second_cubed>();
            test::<l::yoctometer, t::second, j::yoctometer_per_second_cubed>();

            test::<l::foot, t::second, j::foot_per_second_cubed>();
            test::<l::inch, t::second, j::inch_per_second_cubed>();
            test::<l::kilometer, t::minute, j::kilometer_per_minute_cubed>();

            fn test<L: l::Conversion<V>, T: t::Conversion<V>, J: j::Conversion<V>>() {
                Test::assert_eq(
                    &Jerk::new::<J>(V::one()),
                    &(Length::new::<L>(V::one()) /
                        (Time::new::<T>(V::one()) * 
                        Time::new::<T>(V::one()) * 
                        Time::new::<T>(V::one()))));
            }
        }
    }
}
