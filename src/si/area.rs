//! Area (base unit square meter, m²).

quantity! {
    /// Area (base unit square meter, m²).
    quantity: Area; "area";
    /// Dimension of area, L² (base unit square meter, m²).
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @square_yottameter: prefix!(yotta) * prefix!(yotta);
            "Ym²", "square yottameter", "square yottameters";
        @square_zettameter: prefix!(zetta) * prefix!(zetta);
            "Zm²", "square zettameter", "square zettameters";
        @square_exameter: prefix!(exa) * prefix!(exa);
            "Em²", "square exameter", "square exameters";
        @square_petameter: prefix!(peta) * prefix!(peta);
            "Pm²", "square petameter", "square petameters";
        @square_terameter: prefix!(tera) * prefix!(tera);
            "Tm²", "square terameter", "square terameters";
        @square_gigameter: prefix!(giga) * prefix!(giga);
            "Gm²", "square gigameter", "square gigameters";
        @square_megameter: prefix!(mega) * prefix!(mega);
            "Mm²", "square megameter", "square megameters";
        @square_kilometer: prefix!(kilo) * prefix!(kilo);
            "km²", "square kilometer", "square kilometers";
        @square_hectometer: prefix!(hecto) * prefix!(hecto);
            "hm²", "square hectometer", "square hectometers";
        @square_decameter: prefix!(deca) * prefix!(deca);
            "dam²", "square decameter", "square decameters";
        @square_meter: prefix!(none);
            "m²", "square meter", "square meters";
        @square_decimeter: prefix!(deci) * prefix!(deci);
            "dm²", "square decimeter", "square decimeters";
        @square_centimeter: prefix!(centi) * prefix!(centi);
            "cm²", "square centimeter", "square centimeters";
        @square_millimeter: prefix!(milli) * prefix!(milli);
            "mm²", "square millimeter", "square millimeters";
        @square_micrometer: prefix!(micro) * prefix!(micro);
            "µm²", "square micrometer", "square micrometers";
        @square_nanometer: prefix!(nano) * prefix!(nano);
            "nm²", "square nanometer", "square nanometers";
        @square_picometer: prefix!(pico) * prefix!(pico);
            "pm²", "square picometer", "square picometers";
        @square_femtometer: prefix!(femto) * prefix!(femto);
            "fm²", "square femtometer", "square femtometers";
        @square_attometer: prefix!(atto) * prefix!(atto);
            "am²", "square attometer", "square attometers";
        @square_zeptometer: prefix!(zepto) * prefix!(zepto);
            "zm²", "square zeptometer", "square zeptometers";
        @square_yoctometer: prefix!(yocto) * prefix!(yocto);
            "ym²", "square yoctometer", "square yoctometers";

        @acre: 4.046_873_E3; "ac", "acre", "acres";
        @are: 1.0_E2; "a", "are", "ares";
        @barn: 1.0_E-28; "b", "barn", "barns";
        @circular_mil: 5.067_075_E-10; "cmil", "circular mil", "circular mils";
        @hectare: 1.0_E4; "ha", "hectare", "hectares";
        @square_foot: 9.290_304_E-2; "ft²", "square foot", "square feet";
        @square_inch: 6.451_6_E-4; "in²", "square inch", "square inches";
        @square_mile: 2.589_988_E6; "mi²", "square mile", "square miles";
        @square_yard: 8.361_274_E-1; "yd²", "square yard", "square yards";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use lib::any::TypeId;
        use num::One;
        use si::quantities::*;
        use si::area as a;
        use si::length as l;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Area<V> = Length::new::<l::meter>(V::one()) * Length::new::<l::meter>(V::one());
        }

        #[test]
        fn check_units() {
            // Values too large for f32.
            if TypeId::of::<f64>() == TypeId::of::<V>() {
                test::<l::yottameter, a::square_yottameter>();
                test::<l::zettameter, a::square_zettameter>();
            }

            test::<l::exameter, a::square_exameter>();
            test::<l::petameter, a::square_petameter>();
            test::<l::terameter, a::square_terameter>();
            test::<l::gigameter, a::square_gigameter>();
            test::<l::megameter, a::square_megameter>();
            test::<l::kilometer, a::square_kilometer>();
            test::<l::hectometer, a::square_hectometer>();
            test::<l::decameter, a::square_decameter>();
            test::<l::meter, a::square_meter>();
            test::<l::decimeter, a::square_decimeter>();
            test::<l::centimeter, a::square_centimeter>();
            test::<l::millimeter, a::square_millimeter>();
            test::<l::micrometer, a::square_micrometer>();
            test::<l::nanometer, a::square_nanometer>();
            test::<l::picometer, a::square_picometer>();
            test::<l::femtometer, a::square_femtometer>();
            test::<l::attometer, a::square_attometer>();
            test::<l::zeptometer, a::square_zeptometer>();

            // Values too small for f32.
            if TypeId::of::<f64>() == TypeId::of::<V>() {
                test::<l::yoctometer, a::square_yoctometer>();
            }

            fn test<L: l::Conversion<V>, A: a::Conversion<V>>() {
                Test::assert_eq(&Area::new::<A>(V::one()),
                    &(Length::new::<L>(V::one()) * Length::new::<L>(V::one())));
            }
        }
    }
}
