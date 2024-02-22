//! Angular momentum (base unit kilogram square meter per second, kg · m² · s⁻¹).

quantity! {
    /// Angular momentum (base unit kilogram square meter per second, kg · m² · s⁻¹).
    quantity: AngularMomentum; "angular momentum";
    /// Dimension of angular momentum, L²MT⁻¹ (base unit kilogram square meter per second, kg · m² · s⁻¹).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::AngleKind);
    units {
        /// Derived unit of angular velocity.
        @kilogram_square_meter_per_second: prefix!(kilo) / prefix!(kilo); "kg · m²/s",
            "kilogram square meter per second", "kilogram square meters per second";

        @yottanewton_meter_second: prefix!(yotta); "YN · m · s", "yottanewton meter second",
            "yottanewton meter seconds";
        @zettanewton_meter_second: prefix!(zetta); "ZN · m · s", "zettanewton meter second",
            "zettanewton meter seconds";
        @exanewton_meter_second: prefix!(exa); "EN · m · s", "exanewton meter second",
            "exanewton meter seconds";
        @petanewton_meter_second: prefix!(peta); "PN · m · s", "petanewton meter second",
            "petanewton meter seconds";
        @teranewton_meter_second: prefix!(tera); "TN · m · s", "teranewton meter second",
            "teranewton meter seconds";
        @giganewton_meter_second: prefix!(giga); "GN · m · s", "giganewton meter second",
            "giganewton meter seconds";
        @meganewton_meter_second: prefix!(mega); "MN · m · s", "meganewton meter second",
            "meganewton meter seconds";
        @kilonewton_meter_second: prefix!(kilo); "kN · m · s", "kilonewton meter second",
            "kilonewton meter seconds";
        @hectonewton_meter_second: prefix!(hecto); "hN · m · s", "hectonewton meter second",
            "hectonewton meter seconds";
        @decanewton_meter_second: prefix!(deca); "daN · m · s", "decanewton meter second",
            "decanewton meter seconds";
        @newton_meter_second: prefix!(none); "N · m · s", "newton meter second",
            "newton meter seconds";
        @decinewton_meter_second: prefix!(deci); "dN · m · s", "decinewton meter second",
            "decinewton meter seconds";
        @centinewton_meter_second: prefix!(centi); "cN · m · s", "centinewton meter second",
            "centinewton meter seconds";
        @millinewton_meter_second: prefix!(milli); "mN · m · s", "millinewton meter second",
            "millinewton meter seconds";
        @micronewton_meter_second: prefix!(micro); "µN · m · s", "micronewton meter second",
            "micronewton meter seconds";
        @nanonewton_meter_second: prefix!(nano); "nN · m · s", "nanonewton meter second",
            "nanonewton meter seconds";
        @piconewton_meter_second: prefix!(pico); "pN · m · s", "piconewton meter second",
            "piconewton meter seconds";
        @femtonewton_meter_second: prefix!(femto); "fN · m · s", "femtonewton meter second",
            "femtonewton meter seconds";
        @attonewton_meter_second: prefix!(atto); "aN · m · s", "attonewton meter second",
            "attonewton meter seconds";
        @zeptonewton_meter_second: prefix!(zepto); "zN · m · s", "zeptonewton meter second",
            "zeptonewton meter seconds";
        @yoctonewton_meter_second: prefix!(yocto); "yN · m · s", "yoctonewton meter second",
            "yoctonewton meter seconds";

        @newton_yottameter_second: prefix!(yotta); "N · Ym · s", "newton yottameter second",
            "newton yottameter seconds";
        @newton_zettameter_second: prefix!(zetta); "N · Zm · s", "newton zettameter second",
            "newton zettameter seconds";
        @newton_exameter_second: prefix!(exa); "N · Em · s", "newton exameter second",
            "newton exameter seconds";
        @newton_petameter_second: prefix!(peta); "N · Pm · s", "newton petameter second",
            "newton petameter seconds";
        @newton_terameter_second: prefix!(tera); "N · Tm · s", "newton terameter second",
            "newton terameter seconds";
        @newton_gigameter_second: prefix!(giga); "N · Gm · s", "newton gigameter second",
            "newton gigameter seconds";
        @newton_megameter_second: prefix!(mega); "N · Mm · s", "newton megameter second",
            "newton megameter seconds";
        @newton_kilometer_second: prefix!(kilo); "N · km · s", "newton kilometer second",
            "newton kilometer seconds";
        @newton_hectometer_second: prefix!(hecto); "N · hm · s", "newton hectometer second",
            "newton hectometer seconds";
        @newton_decameter_second: prefix!(deca); "N · dam · s", "newton decameter second",
            "newton decameter seconds";
        @newton_decimeter_second: prefix!(deci); "N · dm · s", "newton decimeter second",
            "newton decimeter seconds";
        @newton_centimeter_second: prefix!(centi); "N · cm · s", "newton centimeter second",
            "newton centimeter seconds";
        @newton_millimeter_second: prefix!(milli); "N · mm · s", "newton millimeter second",
            "newton millimeter seconds";
        @newton_micrometer_second: prefix!(micro); "N · µm · s", "newton micrometer second",
            "newton micrometer seconds";
        @newton_nanometer_second: prefix!(nano); "N · nm · s", "newton nanometer second",
            "newton nanometer seconds";
        @newton_picometer_second: prefix!(pico); "N · pm · s", "newton picometer second",
            "newton picometer seconds";
        @newton_femtometer_second: prefix!(femto); "N · fm · s", "newton femtometer second",
            "newton femtometer seconds";
        @newton_attometer_second: prefix!(atto); "N · am · s", "newton attometer second",
            "newton attometer seconds";
        @newton_zeptometer_second: prefix!(zepto); "N · zm · s", "newton zeptometer second",
            "newton zeptometer seconds";
        @newton_yoctometer_second: prefix!(yocto); "N · ym · s", "newton yoctometer second",
            "newton yoctometer seconds";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::angular_momentum as am;
        use crate::si::area as a;
        use crate::si::force as f;
        use crate::si::length as l;
        use crate::si::mass as m;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: AngularMomentum<V> = (Mass::new::<m::kilogram>(V::one())
                * Area::new::<a::square_meter>(V::one())
                / Time::new::<t::second>(V::one()))
            .into();
            let _: AngularMomentum<V> = (Force::new::<f::newton>(V::one())
                * Length::new::<l::meter>(V::one())
                * Time::new::<t::second>(V::one()))
            .into();
        }

        #[test]
        fn check_units_mass_area() {
            test::<m::kilogram, a::square_meter, am::kilogram_square_meter_per_second>();

            fn test<M: m::Conversion<V>, A: a::Conversion<V>, AM: am::Conversion<V>>() {
                Test::assert_approx_eq(
                    &AngularMomentum::new::<AM>(V::one()),
                    &(Mass::new::<M>(V::one())
                        * Area::new::<A>(V::one())
                        / Time::new::<t::second>(V::one()))
                    .into(),
                );
            }
        }

        #[test]
        fn check_units_force_length() {
            test::<f::yottanewton, l::meter, am::yottanewton_meter_second>();
            test::<f::zettanewton, l::meter, am::zettanewton_meter_second>();
            test::<f::exanewton, l::meter, am::exanewton_meter_second>();
            test::<f::petanewton, l::meter, am::petanewton_meter_second>();
            test::<f::teranewton, l::meter, am::teranewton_meter_second>();
            test::<f::giganewton, l::meter, am::giganewton_meter_second>();
            test::<f::meganewton, l::meter, am::meganewton_meter_second>();
            test::<f::kilonewton, l::meter, am::kilonewton_meter_second>();
            test::<f::hectonewton, l::meter, am::hectonewton_meter_second>();
            test::<f::decanewton, l::meter, am::decanewton_meter_second>();
            test::<f::newton, l::meter, am::newton_meter_second>();
            test::<f::decinewton, l::meter, am::decinewton_meter_second>();
            test::<f::centinewton, l::meter, am::centinewton_meter_second>();
            test::<f::millinewton, l::meter, am::millinewton_meter_second>();
            test::<f::micronewton, l::meter, am::micronewton_meter_second>();
            test::<f::nanonewton, l::meter, am::nanonewton_meter_second>();
            test::<f::piconewton, l::meter, am::piconewton_meter_second>();
            test::<f::femtonewton, l::meter, am::femtonewton_meter_second>();
            test::<f::attonewton, l::meter, am::attonewton_meter_second>();
            test::<f::zeptonewton, l::meter, am::zeptonewton_meter_second>();
            test::<f::yoctonewton, l::meter, am::yoctonewton_meter_second>();

            test::<f::newton, l::yottameter, am::newton_yottameter_second>();
            test::<f::newton, l::zettameter, am::newton_zettameter_second>();
            test::<f::newton, l::exameter, am::newton_exameter_second>();
            test::<f::newton, l::petameter, am::newton_petameter_second>();
            test::<f::newton, l::terameter, am::newton_terameter_second>();
            test::<f::newton, l::gigameter, am::newton_gigameter_second>();
            test::<f::newton, l::megameter, am::newton_megameter_second>();
            test::<f::newton, l::kilometer, am::newton_kilometer_second>();
            test::<f::newton, l::hectometer, am::newton_hectometer_second>();
            test::<f::newton, l::decameter, am::newton_decameter_second>();
            test::<f::newton, l::decimeter, am::newton_decimeter_second>();
            test::<f::newton, l::centimeter, am::newton_centimeter_second>();
            test::<f::newton, l::millimeter, am::newton_millimeter_second>();
            test::<f::newton, l::micrometer, am::newton_micrometer_second>();
            test::<f::newton, l::nanometer, am::newton_nanometer_second>();
            test::<f::newton, l::picometer, am::newton_picometer_second>();
            test::<f::newton, l::femtometer, am::newton_femtometer_second>();
            test::<f::newton, l::attometer, am::newton_attometer_second>();
            test::<f::newton, l::zeptometer, am::newton_zeptometer_second>();
            test::<f::newton, l::yoctometer, am::newton_yoctometer_second>();

            fn test<F: f::Conversion<V>, L: l::Conversion<V>, AM: am::Conversion<V>>() {
                Test::assert_approx_eq(
                    &AngularMomentum::new::<AM>(V::one()),
                    &(Force::new::<F>(V::one())
                        * Length::new::<L>(V::one())
                        * Time::new::<t::second>(V::one()))
                    .into(),
                );
            }
        }
    }
}
