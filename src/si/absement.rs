//! Absement (base unit meter second, m · s).

quantity! {
    /// Absement (base unit meter second, m · s).
    quantity: Absement; "absement";
    /// Dimension of absement, LT (base unit meter second, m · s).
    dimension: ISQ<
        P1,     // length
        Z0,     // mass
        P1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottameter_second: prefix!(yotta); "Ym · s", "yottameter second",
            "yottameter seconds";
        @zettameter_second: prefix!(zetta); "Zm · s", "zettameter second",
            "zettameter seconds";
        @exameter_second: prefix!(exa); "Em · s", "exameter second",
            "exameter seconds";
        @petameter_second: prefix!(peta); "Pm · s", "petameter second",
            "petameter seconds";
        @terameter_second: prefix!(tera); "Tm · s", "terameter second",
            "terameter seconds";
        @gigameter_second: prefix!(giga); "Gm · s", "gigameter second",
            "gigameter seconds";
        @megameter_second: prefix!(mega); "Mm · s", "megameter second",
            "megameter seconds";
        @kilometer_second: prefix!(kilo); "km · s", "kilometer second",
            "kilometer seconds";
        @hectometer_second: prefix!(hecto); "hm · s", "hectometer second",
            "hectometer seconds";
        @decameter_second: prefix!(deca); "dam · s", "decameter second",
            "decameter seconds";
        @meter_second: prefix!(none); "m · s", "meter second",
            "meter seconds";
        @decimeter_second: prefix!(deci); "dm · s", "decimeter second",
            "decimeter seconds";
        @centimeter_second: prefix!(centi); "cm · s", "centimeter second",
            "centimeter seconds";
        @millimeter_second: prefix!(milli); "mm · s", "millimeter second",
            "millimeter seconds";
        @micrometer_second: prefix!(micro); "µm · s", "micrometer second",
            "micrometer seconds";
        @nanometer_second: prefix!(nano); "nm · s", "nanometer second",
            "nanometer seconds";
        @picometer_second: prefix!(pico); "pm · s", "picometer second",
            "picometer seconds";
        @femtometer_second: prefix!(femto); "fm · s", "femtometer second",
            "femtometer seconds";
        @attometer_second: prefix!(atto); "am · s", "attometer second",
            "attometer seconds";
        @zeptometer_second: prefix!(zepto); "zm · s", "zeptometer second",
            "zeptometer seconds";
        @yoctometer_second: prefix!(yocto); "ym · s", "yoctometer second",
            "yoctometer seconds";

        @foot_second: 3.048_E-1; "ft · s", "foot second", "foot seconds";
        @inch_second: 2.54_E-2; "in · s", "inch second", "inch seconds";
        @kilometer_hour: 3.6_E6; "km · h", "kilometer hour", "kilometer hours";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::si::absement as a;
        use crate::si::length as l;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::tests::Test;
        use crate::num::One;

        #[test]
        fn check_dimension() {
            let _: Absement<V> = Length::new::<l::meter>(V::one())
                * Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<l::yottameter, t::second, a::yottameter_second>();
            test::<l::zettameter, t::second, a::zettameter_second>();
            test::<l::exameter, t::second, a::exameter_second>();
            test::<l::petameter, t::second, a::petameter_second>();
            test::<l::terameter, t::second, a::terameter_second>();
            test::<l::gigameter, t::second, a::gigameter_second>();
            test::<l::megameter, t::second, a::megameter_second>();
            test::<l::kilometer, t::second, a::kilometer_second>();
            test::<l::hectometer, t::second, a::hectometer_second>();
            test::<l::decameter, t::second, a::decameter_second>();
            test::<l::meter, t::second, a::meter_second>();
            test::<l::decimeter, t::second, a::decimeter_second>();
            test::<l::centimeter, t::second, a::centimeter_second>();
            test::<l::millimeter, t::second, a::millimeter_second>();
            test::<l::micrometer, t::second, a::micrometer_second>();
            test::<l::nanometer, t::second, a::nanometer_second>();
            test::<l::picometer, t::second, a::picometer_second>();
            test::<l::femtometer, t::second, a::femtometer_second>();
            test::<l::attometer, t::second, a::attometer_second>();
            test::<l::zeptometer, t::second, a::zeptometer_second>();
            test::<l::yoctometer, t::second, a::yoctometer_second>();

            test::<l::foot, t::second, a::foot_second>();
            test::<l::inch, t::second, a::inch_second>();
            test::<l::kilometer, t::hour, a::kilometer_hour>();

            fn test<L: l::Conversion<V>, T: t::Conversion<V>, A: a::Conversion<V>>() {
                Test::assert_eq(&Absement::new::<A>(V::one()),
                    &(Length::new::<L>(V::one()) * Time::new::<T>(V::one())));
            }
        }
    }
}
