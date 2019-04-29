//! Pressure (base unit pascal, kg · m⁻¹ · s⁻²).

quantity! {
    /// Pressure (base unit pascal, kg · m⁻¹ · s⁻²).
    quantity: Pressure; "pressure";
    /// Dimension of pressure, L⁻¹MT⁻² (base unit pascal, kg · m⁻¹ · s⁻²).
    dimension: ISQ<
        N1,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottapascal: prefix!(yotta); "YPa", "yottapascal", "yottapascals";
        @zettapascal: prefix!(zetta); "ZPa", "zettapascal", "zettapascals";
        @exapascal: prefix!(exa); "EPa", "exapascal", "exapascals";
        @petapascal: prefix!(peta); "PPa", "petapascal", "petapascals";
        @terapascal: prefix!(tera); "TPa", "terapascal", "terapascals";
        @gigapascal: prefix!(giga); "GPa", "gigapascal", "gigapascals";
        @megapascal: prefix!(mega); "MPa", "megapascal", "megapascals";
        @kilopascal: prefix!(kilo); "kPa", "kilopascal", "kilopascals";
        @hectopascal: prefix!(hecto); "hPa", "hectopascal", "hectopascals";
        @decapascal: prefix!(deca); "daPa", "decapascal", "decapascals";
        /// Derived unit of pressure.
        @pascal: prefix!(none); "Pa", "pascal", "pascals";
        @decipascal: prefix!(deci); "dPa", "decipascal", "decipascals";
        @centipascal: prefix!(centi); "cPa", "centipascal", "centipascals";
        @millipascal: prefix!(milli); "mPa", "millipascal", "millipascals";
        @micropascal: prefix!(micro); "µPa", "micropascal", "micropascals";
        @nanopascal: prefix!(nano); "nPa", "nanopascal", "nanopascals";
        @picopascal: prefix!(pico); "pPa", "picopascal", "picopascals";
        @femtopascal: prefix!(femto); "fPa", "femtopascal", "femtopascals";
        @attopascal: prefix!(atto); "aPa", "attopascal", "attopascals";
        @zeptopascal: prefix!(zepto); "zPa", "zeptopascal", "zeptopascals";
        @yoctopascal: prefix!(yocto); "yPa", "yoctopascal", "yoctopascals";

        @atmosphere: 1.013_25_E5; "atm", "atmosphere", "atmospheres";
        @atmosphere_technical: 9.806_65_E4; "at", "atmosphere (technical)",
            "atmospheres (technical)";
        @bar: 1.0_E5; "bar", "bar", "bar";
        @centimeter_of_mercury_0: 1.333_22_E3; "cm Hg (0 °C)", "centimeter of mercury (0 °C)",
            "centimeters of mercury (0 °C)";
        @centimeter_of_mercury: 1.333_224_E3; "cm Hg", "centimeter of mercury",
            "centimeters of mercury";
        @centimeter_of_water_4: 9.806_38_E1; "cm H₂O (4 °C)", "centimeter of water (4 °C)",
            "centimeters of water (4 °C)";
        @centimeter_of_water: 9.806_65_E1; "cm H₂O", "centimeter of water", "centimeters of water";
        @dyne_per_square_centimeter: 1.0_E-1; "dyn/cm²", "dyne per square centimeter",
            "dynes per square centimeter";
        @foot_of_mercury: 4.063_666_E4; "ft Hg", "foot of mercury", "feet of mercury";
        @foot_of_water_39_2: 2.988_98_E3; "ft H₂O (39.2 °F)", "foot of water (39.2 °F)",
            "feet of water (39.2 °F)";
        @foot_of_water: 2.989_067_E3; "ft H₂O", "foot of water", "feet of water";
        @gram_force_per_square_centimeter: 9.806_65_E1; "gf/cm²",
            "gram-force per square centimeter", "grams-force per square centimeter";
        @inch_of_mercury_32: 3.386_38_E3; "in Hg (32 °F)", "inch of mercury (32 °F)",
            "inches of mercury (32 °F)";
        @inch_of_mercury_60: 3.376_85_E3; "in Hg (60 °F)", "inch of mercury (60 °F)",
            "inches of mercury (60 °F)";
        @inch_of_mercury: 3.386_389_E3; "in Hg", "inch of mercury", "inches of mercury";
        @inch_of_water_39_2: 2.490_82_E2; "in H₂O (39.2 °F)", "inch of water (39.2 °F)",
            "inches of water (39.2 °F)";
        @inch_of_water_60: 2.488_4_E2; "in H₂O (60 °F)", "inch of water (60 °F)",
            "inches of water (60 °F)";
        @inch_of_water: 2.490_889_E2; "in H₂O", "inch of water", "inches of water";
        @kilogram_force_per_square_centimeter: 9.806_65_E4; "kgf/cm²",
            "kilogram-force per square centimeter", "kilograms-force per square centimeter";
        @kilogram_force_per_square_meter: 9.806_65_E0; "kgf/m²",
            "kilogram-force per square meter", "kilograms-force per square meter";
        @kilogram_force_per_square_millimeter: 9.806_65_E6; "kgf/mm²",
            "kilogram-force per square millimeter", "kilograms-force per square millimeter";
        @kip_per_square_inch: 6.894_757_889_515_779_E6; "kip/in²", "kip per square inch",
            "kips per square inch";
        @millibar: 1.0_E2; "mbar", "millibar", "millibar";
        @millimeter_of_mercury: 1.333_224_E2; "mm Hg", "millimeter of mercury",
            "millimeters of mercury";
        @millimeter_of_water: 9.806_65_E0; "mm H₂O", "millimeter of water",
            "millimeters of water";
        @millitorr: 1.333_224_E-1; "mTorr", "millitorr", "millitorr";
        @poundal_per_square_foot: 1.488_164_434_662_202_5_E0; "pdl/ft²", "poundal per square foot",
            "poundals per square foot";
        @pound_force_per_square_foot: 4.788_026_312_163_735_6_E1; "lbf/ft²", "pound-force per square foot",
            "pounds-force per square foot";
        @pound_force_per_square_inch: 6.894_757_889_515_779_E3; "lbf/in²", "pound-force per square inch",
            "pounds-force per square inch";
        @psi: 6.894_757_E3; "psi", "pound-force per square inch",
            "pounds-force per square inch";
        @torr: 1.333_224_E2; "Torr", "torr", "torr";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::force as f;
        use si::area as a;
        use si::pressure as p;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Pressure<V> = Force::new::<f::newton>(V::one())
                / Area::new::<a::square_meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<f::yottanewton, a::square_meter, p::yottapascal>();
            test::<f::zettanewton, a::square_meter, p::zettapascal>();
            test::<f::exanewton, a::square_meter, p::exapascal>();
            test::<f::petanewton, a::square_meter, p::petapascal>();
            test::<f::teranewton, a::square_meter, p::terapascal>();
            test::<f::giganewton, a::square_meter, p::gigapascal>();
            test::<f::meganewton, a::square_meter, p::megapascal>();
            test::<f::kilonewton, a::square_meter, p::kilopascal>();
            test::<f::hectonewton, a::square_meter, p::hectopascal>();
            test::<f::decanewton, a::square_meter, p::decapascal>();
            test::<f::newton, a::square_meter, p::pascal>();
            test::<f::decinewton, a::square_meter, p::decipascal>();
            test::<f::centinewton, a::square_meter, p::centipascal>();
            test::<f::millinewton, a::square_meter, p::millipascal>();
            test::<f::micronewton, a::square_meter, p::micropascal>();
            test::<f::nanonewton, a::square_meter, p::nanopascal>();
            test::<f::piconewton, a::square_meter, p::picopascal>();
            test::<f::femtonewton, a::square_meter, p::femtopascal>();
            test::<f::attonewton, a::square_meter, p::attopascal>();
            test::<f::zeptonewton, a::square_meter, p::zeptopascal>();
            test::<f::yoctonewton, a::square_meter, p::yoctopascal>();

            test::<f::dyne, a::square_centimeter, p::dyne_per_square_centimeter>();
            test::<f::kilogram_force, a::square_millimeter, p::kilogram_force_per_square_millimeter>();
            test::<f::kilogram_force, a::square_centimeter, p::kilogram_force_per_square_centimeter>();
            test::<f::kilogram_force, a::square_meter, p::kilogram_force_per_square_meter>();
            test::<f::kip, a::square_inch, p::kip_per_square_inch>();
            test::<f::poundal, a::square_foot, p::poundal_per_square_foot>();
            test::<f::pound_force, a::square_inch, p::pound_force_per_square_inch>();
            test::<f::pound_force, a::square_foot, p::pound_force_per_square_foot>();

            fn test<F: f::Conversion<V>, A: a::Conversion<V>, P: p::Conversion<V>>() {
                Test::assert_approx_eq(&Pressure::new::<P>(V::one()),
                    &(Force::new::<F>(V::one()) / Area::new::<A>(V::one())));
            }
        }
    }
}
