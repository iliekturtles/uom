//! Torque (aka moment of force) (base unit newton meter, kg · m² · s⁻²).

quantity! {
    /// Torque (aka moment of force) (base unit newton meter, kg · m² · s⁻²).
    ///
    /// Torque is a moment, the product of distance and force. Moments are inherently dependent on
    /// the distance from a fixed reference point. This library does not capture this dependency.
    /// As a consequence, there are *no compile time guarantees that only moments of the same frame
    /// can be combined*.
    quantity: Torque; "torque";
    /// Dimension of torque, L²MT⁻² (base unit newton meter, kg · m² · s⁻²).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (::si::marker::AngleKind);
    units {
        @yottanewton_meter: prefix!(yotta); "YN · m", "yottanewton meter", "yottanewton meters";
        @zettanewton_meter: prefix!(zetta); "ZN · m", "zettanewton meter", "zettanewton meters";
        @exanewton_meter: prefix!(exa); "EN · m", "exanewton meter", "exanewton meters";
        @petanewton_meter: prefix!(peta); "PN · m", "petanewton meter", "petanewton meters";
        @teranewton_meter: prefix!(tera); "TN · m", "teranewton meter", "teranewton meters";
        @giganewton_meter: prefix!(giga); "GN · m", "giganewton meter", "giganewton meters";
        @meganewton_meter: prefix!(mega); "MN · m", "meganewton meter", "meganewton meters";
        @kilonewton_meter: prefix!(kilo); "kN · m", "kilonewton meter", "kilonewton meters";
        @hectonewton_meter: prefix!(hecto); "hN · m", "hectonewton meter", "hectonewton meters";
        @decanewton_meter: prefix!(deca); "daN · m", "decanewton meter", "decanewton meters";
        /// Derived unit of torque.
        @newton_meter: prefix!(none); "N · m", "newton meter", "newton meters";
        @decinewton_meter: prefix!(deci); "dN · m", "decinewton meter", "decinewton meters";
        @centinewton_meter: prefix!(centi); "cN · m", "centinewton meter", "centinewton meters";
        @millinewton_meter: prefix!(milli); "mN · m", "millinewton meter", "millinewton meters";
        @micronewton_meter: prefix!(micro); "µN · m", "micronewton meter", "micronewton meters";
        @nanonewton_meter: prefix!(nano); "nN · m", "nanonewton meter", "nanonewton meters";
        @piconewton_meter: prefix!(pico); "pN · m", "piconewton meter", "piconewton meters";
        @femtonewton_meter: prefix!(femto); "fN · m", "femtonewton meter", "femtonewton meters";
        @attonewton_meter: prefix!(atto); "aN · m", "attonewton meter", "attonewton meters";
        @zeptonewton_meter: prefix!(zepto); "zN · m", "zeptonewton meter", "zeptonewton meters";
        @yoctonewton_meter: prefix!(yocto); "yN · m", "yoctonewton meter", "yoctonewton meters";

        @newton_yottameter: prefix!(yotta); "N · Ym", "newton yottameter", "newton yottameters";
        @newton_zettameter: prefix!(zetta); "N · Zm", "newton zettameter", "newton zettameters";
        @newton_exameter: prefix!(exa); "N · Em", "newton exameter", "newton exameters";
        @newton_petameter: prefix!(peta); "N · Pm", "newton petameter", "newton petameters";
        @newton_terameter: prefix!(tera); "N · Tm", "newton terameter", "newton terameters";
        @newton_gigameter: prefix!(giga); "N · Gm", "newton gigameter", "newton gigameters";
        @newton_megameter: prefix!(mega); "N · Mm", "newton megameter", "newton megameters";
        @newton_kilometer: prefix!(kilo); "N · km", "newton kilometer", "newton kilometers";
        @newton_hectometer: prefix!(hecto); "N · hm", "newton hectometer", "newton hectometers";
        @newton_decameter: prefix!(deca); "N · dam", "newton decameter", "newton decameters";
        @newton_decimeter: prefix!(deci); "N · dm", "newton decimeter", "newton decimeters";
        @newton_centimeter: prefix!(centi); "N · cm", "newton centimeter", "newton centimeters";
        @newton_millimeter: prefix!(milli); "N · mm", "newton millimeter", "newton millimeters";
        @newton_micrometer: prefix!(micro); "N · µm", "newton micrometer", "newton micrometers";
        @newton_nanometer: prefix!(nano); "N · nm", "newton nanometer", "newton nanometers";
        @newton_picometer: prefix!(pico); "N · pm", "newton picometer", "newton picometers";
        @newton_femtometer: prefix!(femto); "N · fm", "newton femtometer", "newton femtometers";
        @newton_attometer: prefix!(atto); "N · am", "newton attometer", "newton attometers";
        @newton_zeptometer: prefix!(zepto); "N · zm", "newton zeptometer", "newton zeptometers";
        @newton_yoctometer: prefix!(yocto); "N · ym", "newton yoctometer", "newton yoctometers";

        @dyne_meter: 1.0_E-5; "dyn · m", "dyne meter", "dyne meters";
        @dyne_centimeter: 1.0_E-7; "dyn · cm", "dyne centimeter", "dyne centimeters";
        @kilogram_force_meter: 9.806_65_E0; "kgf · m", "kilogram-force meter",
            "kilogram-force meters";
        @ounce_force_inch: 7.061_553_06_E-3; "ozf · in", "ounce-force inch", "ounces-force inches";
        @pound_force_foot: 1.355_818_065_6_E0; "lbf · ft", "pound-force foot", "pounds-force feet";
        @pound_force_inch: 1.129_848_388_E-1; "lbf · in", "pound-force inch", "pounds-force inches";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::force as f;
        use si::length as l;
        use si::torque as t;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Torque<V> = (Force::new::<f::newton>(V::one())
                * Length::new::<l::meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<f::yottanewton, l::meter, t::yottanewton_meter>();
            test::<f::zettanewton, l::meter, t::zettanewton_meter>();
            test::<f::exanewton, l::meter, t::exanewton_meter>();
            test::<f::petanewton, l::meter, t::petanewton_meter>();
            test::<f::teranewton, l::meter, t::teranewton_meter>();
            test::<f::giganewton, l::meter, t::giganewton_meter>();
            test::<f::meganewton, l::meter, t::meganewton_meter>();
            test::<f::kilonewton, l::meter, t::kilonewton_meter>();
            test::<f::hectonewton, l::meter, t::hectonewton_meter>();
            test::<f::decanewton, l::meter, t::decanewton_meter>();
            test::<f::newton, l::meter, t::newton_meter>();
            test::<f::decinewton, l::meter, t::decinewton_meter>();
            test::<f::centinewton, l::meter, t::centinewton_meter>();
            test::<f::millinewton, l::meter, t::millinewton_meter>();
            test::<f::micronewton, l::meter, t::micronewton_meter>();
            test::<f::nanonewton, l::meter, t::nanonewton_meter>();
            test::<f::piconewton, l::meter, t::piconewton_meter>();
            test::<f::femtonewton, l::meter, t::femtonewton_meter>();
            test::<f::attonewton, l::meter, t::attonewton_meter>();
            test::<f::zeptonewton, l::meter, t::zeptonewton_meter>();
            test::<f::yoctonewton, l::meter, t::yoctonewton_meter>();

            test::<f::newton, l::yottameter, t::newton_yottameter>();
            test::<f::newton, l::zettameter, t::newton_zettameter>();
            test::<f::newton, l::exameter, t::newton_exameter>();
            test::<f::newton, l::petameter, t::newton_petameter>();
            test::<f::newton, l::terameter, t::newton_terameter>();
            test::<f::newton, l::gigameter, t::newton_gigameter>();
            test::<f::newton, l::megameter, t::newton_megameter>();
            test::<f::newton, l::kilometer, t::newton_kilometer>();
            test::<f::newton, l::hectometer, t::newton_hectometer>();
            test::<f::newton, l::decameter, t::newton_decameter>();
            test::<f::newton, l::decimeter, t::newton_decimeter>();
            test::<f::newton, l::centimeter, t::newton_centimeter>();
            test::<f::newton, l::millimeter, t::newton_millimeter>();
            test::<f::newton, l::micrometer, t::newton_micrometer>();
            test::<f::newton, l::nanometer, t::newton_nanometer>();
            test::<f::newton, l::picometer, t::newton_picometer>();
            test::<f::newton, l::femtometer, t::newton_femtometer>();
            test::<f::newton, l::attometer, t::newton_attometer>();
            test::<f::newton, l::zeptometer, t::newton_zeptometer>();
            test::<f::newton, l::yoctometer, t::newton_yoctometer>();

            test::<f::dyne, l::meter, t::dyne_meter>();
            test::<f::dyne, l::centimeter, t::dyne_centimeter>();
            test::<f::kilogram_force, l::meter, t::kilogram_force_meter>();
            test::<f::ounce_force, l::inch, t::ounce_force_inch>();
            test::<f::pound_force, l::foot, t::pound_force_foot>();
            test::<f::pound_force, l::inch, t::pound_force_inch>();

            fn test<F: f::Conversion<V>, L: l::Conversion<V>, T: t::Conversion<V>>() {
                Test::assert_approx_eq(&Torque::new::<T>(V::one()),
                    &(Force::new::<F>(V::one()) * Length::new::<L>(V::one())).into());
            }
        }
    }
}
