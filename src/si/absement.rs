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
        @yottameter_seconds: prefix!(yotta); "Ym · s", "yottameter second",
            "yottameter seconds";
        @zettameter_seconds: prefix!(zetta); "Zm · s", "zettameter second",
            "zettameter seconds";
        @exameter_seconds: prefix!(exa); "Em · s", "exameter second",
            "exameter seconds";
        @petameter_seconds: prefix!(peta); "Pm · s", "petameter second",
            "petameter seconds";
        @terameter_seconds: prefix!(tera); "Tm · s", "terameter second",
            "terameter seconds";
        @gigameter_seconds: prefix!(giga); "Gm · s", "gigameter second",
            "gigameter seconds";
        @megameter_seconds: prefix!(mega); "Mm · s", "megameter second",
            "megameter seconds";
        @kilometer_seconds: prefix!(kilo); "km · s", "kilometer second",
            "kilometer seconds";
        @hectometer_seconds: prefix!(hecto); "hm · s", "hectometer second",
            "hectometer seconds";
        @decameter_seconds: prefix!(deca); "dam · s", "decameter second",
            "decameter seconds";
        @meter_seconds: prefix!(none); "m · s", "meter second",
            "meter seconds";
        @decimeter_seconds: prefix!(deci); "dm · s", "decimeter second",
            "decimeter seconds";
        @centimeter_seconds: prefix!(centi); "cm · s", "centimeter second",
            "centimeter seconds";
        @millimeter_seconds: prefix!(milli); "mm · s", "millimeter second",
            "millimeter seconds";
        @micrometer_seconds: prefix!(micro); "µm · s", "micrometer second",
            "micrometer seconds";
        @nanometer_seconds: prefix!(nano); "nm · s", "nanometer second",
            "nanometer seconds";
        @picometer_seconds: prefix!(pico); "pm · s", "picometer second",
            "picometer seconds";
        @femtometer_seconds: prefix!(femto); "fm · s", "femtometer second",
            "femtometer seconds";
        @attometer_seconds: prefix!(atto); "am · s", "attometer second",
            "attometer seconds";
        @zeptometer_seconds: prefix!(zepto); "zm · s", "zeptometer second",
            "zeptometer seconds";
        @yoctometer_seconds: prefix!(yocto); "ym · s", "yoctometer second",
            "yoctometer seconds";

        @foot_seconds: 3.048_E-1; "ft · s", "foot second", "foot seconds";
        @inch_seconds: 2.54_E-2; "in · s", "inch second", "inch seconds";
        @kilometer_hours: 3.6_E6; "km · h", "kilometer hour", "kilometer hours";
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
            test::<l::yottameter, t::second, a::yottameter_seconds>();
            test::<l::zettameter, t::second, a::zettameter_seconds>();
            test::<l::exameter, t::second, a::exameter_seconds>();
            test::<l::petameter, t::second, a::petameter_seconds>();
            test::<l::terameter, t::second, a::terameter_seconds>();
            test::<l::gigameter, t::second, a::gigameter_seconds>();
            test::<l::megameter, t::second, a::megameter_seconds>();
            test::<l::kilometer, t::second, a::kilometer_seconds>();
            test::<l::hectometer, t::second, a::hectometer_seconds>();
            test::<l::decameter, t::second, a::decameter_seconds>();
            test::<l::meter, t::second, a::meter_seconds>();
            test::<l::decimeter, t::second, a::decimeter_seconds>();
            test::<l::centimeter, t::second, a::centimeter_seconds>();
            test::<l::millimeter, t::second, a::millimeter_seconds>();
            test::<l::micrometer, t::second, a::micrometer_seconds>();
            test::<l::nanometer, t::second, a::nanometer_seconds>();
            test::<l::picometer, t::second, a::picometer_seconds>();
            test::<l::femtometer, t::second, a::femtometer_seconds>();
            test::<l::attometer, t::second, a::attometer_seconds>();
            test::<l::zeptometer, t::second, a::zeptometer_seconds>();
            test::<l::yoctometer, t::second, a::yoctometer_seconds>();

            test::<l::foot, t::second, a::foot_seconds>();
            test::<l::inch, t::second, a::inch_seconds>();
            test::<l::kilometer, t::hour, a::kilometer_hours>();

            fn test<L: l::Conversion<V>, T: t::Conversion<V>, A: a::Conversion<V>>() {
                Test::assert_eq(&Absement::new::<A>(V::one()),
                    &(Length::new::<L>(V::one()) * Time::new::<T>(V::one())));
            }
        }
    }
}
