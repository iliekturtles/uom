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
        @square_mile: 2.589_988_110_336_E6; "mi²", "square mile", "square miles";
        @square_yard: 8.361_273_6_E-1; "yd²", "square yard", "square yards";

        // Ancient Roman units.
        @saltus: 2.018_672_64_E6; "saltus", "saltus", "saltus";
        @centuria: 5.046_681_6_E5; "centuria", "centuria", "centuriae";
        @heredium: 5.046_681_6_E3; "heredium", "heredium", "heredia";
        @jugerum: 2.523_340_8_E3; "jugerum", "jugerum", "jugera";
        @deunx: 2.313_062_4_E3; "deunx", "deunx", "deunces";
        @dextans: 2.102_784_E3; "dextans", "dextans", "dextantes";
        @dodrans: 1.892_505_6_E3; "dodrans", "dodrans", "dodrantes";
        @bes: 1.682_227_2_E3; "bes", "bes", "besses";
        @septunx: 1.471_948_8_E3; "septunx", "septunx", "septunces";
        @semis: 1.261_670_4_E3; "semis", "semis", "semisses";
        @actus_quadratus: 1.261_670_4_E3; "actus quadratus", "actus quadratus", "acta quadratus";
        @quincunx: 1.051_392_E3; "quincunx", "quincunx", "quincunces";
        @triens: 8.411_136_E2; "triens", "triens", "trientes";
        @quadrans: 6.308_352_E2; "quadrans", "quadrans", "quadrantes";
        @sextans: 4.205_568_E2; "sextans", "sextans", "sextantes";
        @clima: 3.154_176_E2; "clima", "clima", "climata";
        @uncia: 2.102_784_E2; "uncia", "uncia", "unciae";
        @semiuncia: 1.051_392_E2; "semiuncia", "semiuncia", "semiunciae";
        @sicilicus: 5.256_96_E1; "sicilicus", "sicilicus", "siculi";
        @actus_simplex: 4.205_568_E1; "actus simplex", "actus simplex", "acta simplex";
        @sextula: 3.504_64_E1; "sextula", "sextula", "sextulae";
        @duo_scrupulum: 1.752_32_E1; "duo scrupulum", "duo scrupulum", "duo scrupula";
        @scrupulum: 8.761_6_E0; "scrupulum", "scrupulum", "scrupula";
        @dimidium_scrupulum: 4.380_8_E0; "dimidium scrupulum", "dimidium scrupulum",
            "dimidium scupula";
        @pes_quadratus: 8.761_6_E-2; "pes quadratus", "pes quadratus", "pedes quadratus";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::{FromPrimitive, One};
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

            test::<l::decameter, a::are>();
            test::<l::foot, a::square_foot>();
            test::<l::inch, a::square_inch>();
            test::<l::mile, a::square_mile>();
            test::<l::yard, a::square_yard>();

            test::<l::pes, a::pes_quadratus>();

            fn test<L: l::Conversion<V>, A: a::Conversion<V>>() {
                if A::is_valid() {
                    Test::assert_approx_eq(&Area::new::<A>(V::one()),
                        &(Length::new::<L>(V::one()) * Length::new::<L>(V::one())));
                }
            }
        }

        #[test]
        fn check_roman() {
            test::<a::scrupulum>(1.0_E2);
            test::<a::duo_scrupulum>(2.0_E2);
            test::<a::sextula>(4.0_E2);
            test::<a::actus_simplex>(4.8_E2);
            test::<a::sicilicus>(6.0_E2);
            test::<a::semiuncia>(1.2_E3);
            test::<a::uncia>(2.4_E3);
            test::<a::clima>(3.6_E3);
            test::<a::sextans>(4.8_E3);
            test::<a::quadrans>(7.2_E3);
            test::<a::triens>(9.6_E3);
            test::<a::quincunx>(1.2_E4);
            test::<a::actus_quadratus>(1.44_E4);
            test::<a::semis>(1.44_E4);
            test::<a::septunx>(1.68_E4);
            test::<a::bes>(1.92_E4);
            test::<a::dodrans>(2.16_E4);
            test::<a::dextans>(2.4_E4);
            test::<a::deunx>(2.64_E4);
            test::<a::jugerum>(2.88_E4);
            test::<a::heredium>(5.76_E4);
            test::<a::centuria>(5.76_E6);
            test::<a::saltus>(2.304_E7);

            fn test<A: a::Conversion<V>>(p: f64) {
                Test::assert_approx_eq(&Area::new::<A>(V::one()),
                    &Area::new::<a::pes_quadratus>(V::from_f64(p).unwrap()));
            }
        }
    }
}
