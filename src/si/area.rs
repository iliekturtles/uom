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

        /// Roman units
        @pes_quadratus: 0.0876; "pes quadratus", "pes quadratus", "pes quadratus";
        @decempeda_quadrata: 8.76; "decempeda quadrata", "decempeda quadrata", "decempeda quadrata";
        @actus_simplex: 42.1; "actus simplex", "actus simplex", "actus simplex";
        @uncia: 210.0; "uncia", "uncia", "uncia";
        @clima: 315.0; "clima", "clima", "clima";
        @actus_quadratus: 1262.0; "actus quadratus", "actus quadratus", "actus quadratus";
        @jugerum: 2523.0; "jugerem", "jugerem", "jugerem";
        @heredium: 5047.0; "heredium", "heredium", "heredium";
        @centuria: 505000.0; "centuria", "centuria", "centuria";
        @saltus: 2019000.0; "saltus", "saltus", "saltus";
        @modius: 160000.0; "modius", "modius", "modius";

        @dimidium_scupula: 4.38; "dimidium scupula", "dimidium scupula", "dimidium scupula";
        @scrupulum: 8.76; "scrupulum", "scrupulum", "scrupulum";
        @dua_scrupula: 17.5; "dua scrupula", "dua scrupula", "dua scrupula";
        @sextula: 35.0; "sextula", "sextula", "sextula";
        @sicilicus: 52.6; "sicilicus", "sicilicus", "sicilicus";
        @semiuncia: 105.0; "semiuncia", "semiuncia", "semiuncia";
        @sextans: 421.0; "sextans", "sextans", "sextans";
        @quadrans: 631.0; "quadrans", "quadrans", "quadrans";
        @triens: 841.0; "triens", "triens", "triens";
        @quincunx: 1051.0; "quincunx", "quincunx", "quincunx";
        @semis: 1262.0; "semis", "semis", "semis";
        @septunx: 1472.0; "septunx", "septunx", "septunx";
        @bes: 1682.0; "bes", "bes", "bes";
        @dodrans: 1893.0; "dodrans", "dodrans", "dodrans";
        @dextans: 2313.0; "dextans", "dextans", "dextans";
        @denux: 2523.0; "denux", "denux", "denux";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::area as a;
        use crate::si::length as l;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: Area<V> = Length::new::<l::meter>(V::one()) * Length::new::<l::meter>(V::one());
        }

        #[test]
        // #392: Disable tests on ARM until issues with floating point behavior can be resolved.
        #[cfg(not(target_arch = "arm"))]
        fn check_units() {
            test::<l::yottameter, a::square_yottameter>();
            test::<l::zettameter, a::square_zettameter>();
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
            test::<l::yoctometer, a::square_yoctometer>();

            fn test<L: l::Conversion<V>, A: a::Conversion<V>>() {
                if A::is_valid() {
                    Test::assert_eq(&Area::new::<A>(V::one()),
                        &(Length::new::<L>(V::one()) * Length::new::<L>(V::one())));
                }            }
        }
    }
}
