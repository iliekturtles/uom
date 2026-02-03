//! Volume (base unit cubic meter, m³).
#![allow(deprecated)] // rust warns of our own deprecations

quantity! {
    /// Volume (base unit cubic meter, m³).
    quantity: Volume; "volume";
    /// Dimension of volume, L³ (base unit cubic meter, m³).
    dimension: ISQ<
        P3,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @cubic_yottameter: prefix!(yotta) * prefix!(yotta) * prefix!(yotta);
            "Ym³", "cubic yottameter", "cubic yottameters";
        @cubic_zettameter: prefix!(zetta) * prefix!(zetta) * prefix!(zetta);
            "Zm³", "cubic zettameter", "cubic zettameters";
        @cubic_exameter: prefix!(exa) * prefix!(exa) * prefix!(exa);
            "Em³", "cubic exameter", "cubic exameters";
        @cubic_petameter: prefix!(peta) * prefix!(peta) * prefix!(peta);
            "Pm³", "cubic petameter", "cubic petameters";
        @cubic_terameter: prefix!(tera) * prefix!(tera) * prefix!(tera);
            "Tm³", "cubic terameter", "cubic terameters";
        @cubic_gigameter: prefix!(giga) * prefix!(giga) * prefix!(giga);
            "Gm³", "cubic gigameter", "cubic gigameters";
        @cubic_megameter: prefix!(mega) * prefix!(mega) * prefix!(mega);
            "Mm³", "cubic megameter", "cubic megameters";
        @cubic_kilometer: prefix!(kilo) * prefix!(kilo) * prefix!(kilo);
            "km³", "cubic kilometer", "cubic kilometers";
        @cubic_hectometer: prefix!(hecto) * prefix!(hecto) * prefix!(hecto);
            "hm³", "cubic hectometer", "cubic hectometers";
        @cubic_decameter: prefix!(deca) * prefix!(deca) * prefix!(deca);
            "dam³", "cubic decameter", "cubic decameters";
        @cubic_meter: prefix!(none); "m³", "cubic meter", "cubic meters";
        @cubic_decimeter: prefix!(deci) * prefix!(deci) * prefix!(deci);
            "dm³", "cubic decimeter", "cubic decimeters";
        @cubic_centimeter: prefix!(centi) * prefix!(centi) * prefix!(centi);
            "cm³", "cubic centimeter", "cubic centimeters";
        @cubic_millimeter: prefix!(milli) * prefix!(milli) * prefix!(milli);
            "mm³", "cubic millimeter", "cubic millimeters";
        @cubic_micrometer: prefix!(micro) * prefix!(micro) * prefix!(micro);
            "µm³", "cubic micrometer", "cubic micrometers";
        @cubic_nanometer: prefix!(nano) * prefix!(nano) * prefix!(nano);
            "nm³", "cubic nanometer", "cubic nanometers";
        @cubic_picometer: prefix!(pico) * prefix!(pico) * prefix!(pico);
            "pm³", "cubic picometer", "cubic picometers";
        @cubic_femtometer: prefix!(femto) * prefix!(femto) * prefix!(femto);
            "fm³", "cubic femtometer", "cubic femtometers";
        @cubic_attometer: prefix!(atto) * prefix!(atto) * prefix!(atto);
            "am³", "cubic attometer", "cubic attometers";
        @cubic_zeptometer: prefix!(zepto) * prefix!(zepto) * prefix!(zepto);
            "zm³", "cubic zeptometer", "cubic zeptometers";
        @cubic_yoctometer: prefix!(yocto) * prefix!(yocto) * prefix!(yocto);
            "ym³", "cubic yoctometer", "cubic yoctometers";

        @acre_foot: 1.233_489_E3; "ac · ft", "acre-foot", "acre-feet";
        @barrel: 1.589_873_E-1; "bbl", "barrel", "barrels";
        @bushel: 3.523_907_E-2; "bu", "bushel", "bushels";
        @cord: 3.624_556_E0; "cords", "cord", "cords";
        @cubic_foot: 2.831_685_E-2; "ft³", "cubic foot", "cubic feet";
        @cubic_inch: 1.638_706_E-5; "in³", "cubic inch", "cubic inches";
        @cubic_mile: 4.168_182_E9; "mi³", "cubic mile", "cubic miles";
        @cubic_yard: 7.645_549_E-1; "yd³", "cubic yard", "cubic yards";
        #[doc = "US customary cups. Deprecated."]
        #[deprecated(note = "please use suffixed unit e.g. _us, _metric")]
        @cup: 2.365_882_E-4; "cup (US customary)", "cup (US customary)", "cups (US customary)";
        @cup_us_customary: 2.365_882_E-4; "cup (US customary)", "cup (US customary)", "cups (US customary)";
        @cup_us_legal: prefix!(milli) * prefix!(milli) * 240.0; "cup (US legal)", "cup (US legal)", "cups (US legal)";
        @cup_metric: prefix!(milli) * prefix!(milli) * 250.0; "cup", "cup", "cups";
        @fluid_ounce: 2.957_353_E-5; "fl oz", "fluid ounce", "fluid ounces";
        @fluid_ounce_imperial: 2.841_306_E-5; "fl oz (UK)", "Imperial fluid ounce", "Imperial fluid ounces";
        @gallon_imperial: 4.546_09_E-3; "gal (UK)", "Imperial gallon", "Imperial gallons";
        @gallon: 3.785_412_E-3; "gal", "gallon", "gallons";
        @gill_imperial: 1.420_653_E-4; "gi (UK)", "Imperial gill", "Imperial gills";
        @gill: 1.182_941_E-4; "gi", "gill", "gills";
        @yottaliter: prefix!(milli) * prefix!(yotta); "YL", "yottaliter", "yottaliters";
        @zettaliter: prefix!(milli) * prefix!(zetta); "ZL", "zettaliter", "zettaliters";
        @exaliter: prefix!(milli) * prefix!(exa); "EL", "exaliter", "exaliters";
        @petaliter: prefix!(milli) * prefix!(peta); "PL", "petaliter", "petaliters";
        @teraliter: prefix!(milli) * prefix!(tera); "TL", "teraliter", "teraliters";
        @gigaliter: prefix!(milli) * prefix!(giga); "GL", "gigaliter", "gigaliters";
        @megaliter: prefix!(milli) * prefix!(mega); "ML", "megaliter", "megaliters";
        @kiloliter: prefix!(milli) * prefix!(kilo); "kL", "kiloliter", "kiloliters";
        @hectoliter: prefix!(milli) * prefix!(hecto); "hL", "hectoliter", "hectoliters";
        @decaliter: prefix!(milli) * prefix!(deca); "daL", "decaliter", "decaliters";
        @liter: prefix!(milli); "L", "liter", "liters";
        @deciliter: prefix!(milli) * prefix!(deci); "dL", "deciliter", "deciliters";
        @centiliter: prefix!(milli) * prefix!(centi); "cL", "centiliter", "centiliters";
        @milliliter: prefix!(milli) * prefix!(milli); "mL", "milliliter", "milliliters";
        @microliter: prefix!(milli) * prefix!(micro); "µL", "microliter", "microliters";
        @nanoliter: prefix!(milli) * prefix!(nano); "nL", "nanoliter", "nanoliters";
        @picoliter: prefix!(milli) * prefix!(pico); "pL", "picoliter", "picoliters";
        @femtoliter: prefix!(milli) * prefix!(femto); "fL", "femtoliter", "femtoliters";
        @attoliter: prefix!(milli) * prefix!(atto); "aL", "attoliter", "attoliters";
        @zeptoliter: prefix!(milli) * prefix!(zepto); "zL", "zeptoliter", "zeptoliters";
        @yoctoliter: prefix!(milli) * prefix!(yocto); "yL", "yoctoliter", "yoctoliters";
        @peck: 8.809_768_E-3; "pk", "peck", "pecks";
        @pint_dry: 5.506_105_E-4; "dry pt", "dry pint", "dry pints";
        @pint_liquid: 4.731_765_E-4; "liq pt", "liquid pint", "liquid pints";
        @quart_dry: 1.101_221_E-3; "dry qt", "dry quart", "dry quarts";
        @quart_liquid: 9.463_529_E-4; "liq qt", "liquid quart", "liquid quarts";
        @stere: 1.0_E0; "st", "stere", "steres";
        #[deprecated(note = "please use suffixed unit e.g. _us, _aus, _metric")]
        #[doc = "US tablespoons. Deprecated."] // doc is required here, see macro arms in unit.rs L368 vs L374
        @tablespoon: 1.478_676_E-5; "tbsp (US)", "tablespoon (US)", "tablespoons (US)";
        #[deprecated(note = "please use suffixed unit e.g. _us, _metric")]
        #[doc = "US teaspoons. Deprecated."]
        @teaspoon: 4.928_922_E-6; "tsp (US)", "teaspoon (US)", "teaspoons (US)";
        @tablespoon_us: 1.478_676_E-5; "tbsp (US)", "US tablespoon", "US tablespoons";
        @teaspoon_us: 4.928_922_E-6; "tsp (US)", "US teaspoon", "US teaspoons";
        @tablespoon_metric: prefix!(milli) * prefix!(milli) * 15.0; "tbsp", "tablespoon", "tablespoons";
        @teaspoon_metric: prefix!(milli) * prefix!(milli) * 5.0; "tsp", "teaspoon", "teaspoons";
        // note: Australian tablespoons are 20mL (so one teaspoon is 1/4 of a tbsp) instead of the
        // typical 15mL. There is no difference in the Australian teaspoon.
        @tablespoon_aus: prefix!(milli) * prefix!(milli) * 20.0; "tbsp (AU)", "tablespoon (AU)", "tablespoons (AU)";
        @register_ton: 2.831_685_E0; "RT", "register ton", "register tons";

        // Ancient Roman units.
        @culeus: 5.186_867_2_E-1; "culeus", "culeus", "culei";
        @amphora_quadrantal: 2.593_433_6_E-2; "amphora quadrantal", "amphorae quadrantal",
            "amphora quadrantal";
        @urna: 1.296_716_8_E-2; "urna", "urna", "urnae";
        @modius_castrensis: 1.296_716_8_E-2; "modius castrensis", "modius castrensis",
            "modii castrensis";
        @modius: 8.644_778_666_666_66_E-3; "modius", "modius", "modii";
        @semimodius: 4.322_389_333_333_33_E-3; "semimodius", "semimodius", "semimodii";
        @congius: 3.241_792_E-3; "congius", "congius", "congii";
        @sextarius: 5.402_986_666_666_67_E-4; "sextarius", "sextarius", "sextarii";
        @hemina: 2.701_493_333_333_33_E-4; "hemina", "hemina", "heminae";
        @quartarius: 1.350_746_666_666_67_E-4; "quartarius", "quartarius", "quartarii";
        @acetabulum: 6.753_733_333_333_33_E-5; "acetabulum", "acetabulum", "acetabula";
        @cyathus: 4.502_488_888_888_89_E-5; "cyathus", "cyathus", "cyathi";
        @ligula: 1.125_622_222_222_222_2_E-5; "ligula", "ligula", "ligula";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::{FromPrimitive, One};
        use crate::si::area as a;
        use crate::si::length as l;
        use crate::si::quantities::*;
        use crate::si::volume as v;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: Volume<V> = Length::new::<l::meter>(V::one())
                * Length::new::<l::meter>(V::one())
                * Length::new::<l::meter>(V::one());
            let _: Volume<V> = Length::new::<l::meter>(V::one())
                * Area::new::<a::square_meter>(V::one());
        }

        #[test]
        // #392: Disable tests on ARM until issues with floating point behavior can be resolved.
        #[cfg(not(target_arch = "arm"))]
        fn check_units() {
            test::<l::yottameter, v::cubic_yottameter>();
            test::<l::zettameter, v::cubic_zettameter>();
            test::<l::exameter, v::cubic_exameter>();
            test::<l::petameter, v::cubic_petameter>();
            test::<l::terameter, v::cubic_terameter>();
            test::<l::gigameter, v::cubic_gigameter>();
            test::<l::megameter, v::cubic_megameter>();
            test::<l::kilometer, v::cubic_kilometer>();
            test::<l::hectometer, v::cubic_hectometer>();
            test::<l::decameter, v::cubic_decameter>();
            test::<l::meter, v::cubic_meter>();
            test::<l::decimeter, v::cubic_decimeter>();
            test::<l::centimeter, v::cubic_centimeter>();
            test::<l::millimeter, v::cubic_millimeter>();
            test::<l::micrometer, v::cubic_micrometer>();
            test::<l::nanometer, v::cubic_nanometer>();
            test::<l::picometer, v::cubic_picometer>();
            test::<l::femtometer, v::cubic_femtometer>();
            test::<l::attometer, v::cubic_attometer>();
            test::<l::zeptometer, v::cubic_zeptometer>();
            test::<l::yoctometer, v::cubic_yoctometer>();

            test::<l::pes, v::amphora_quadrantal>();

            fn test<L: l::Conversion<V>, O: v::Conversion<V>>() {
                if O::is_valid() {
                    Test::assert_approx_eq(&Volume::new::<O>(V::one()),
                        &(Length::new::<L>(V::one())
                            * Length::new::<L>(V::one())
                            * Length::new::<L>(V::one())));
                }
            }
        }

        #[test]
        fn check_liters() {
            // Test liter base relative to cubic meter base to verify a baseline
            test::<v::liter, v::cubic_meter>(prefix!(milli));
            // Test relative to liter to make sure prefixes are good
            // This transitively verifies the other relations
            test::<v::yottaliter, v::liter>(prefix!(yotta));
            test::<v::zettaliter, v::liter>(prefix!(zetta));
            test::<v::exaliter, v::liter>(prefix!(exa));
            test::<v::petaliter, v::liter>(prefix!(peta));
            test::<v::teraliter, v::liter>(prefix!(tera));
            test::<v::gigaliter, v::liter>(prefix!(giga));
            test::<v::megaliter, v::liter>(prefix!(mega));
            test::<v::kiloliter, v::liter>(prefix!(kilo));
            test::<v::hectoliter, v::liter>(prefix!(hecto));
            test::<v::decaliter, v::liter>(prefix!(deca));
            test::<v::liter, v::liter>(1.0_E0);
            test::<v::deciliter, v::liter>(prefix!(deci));
            test::<v::centiliter, v::liter>(prefix!(centi));
            test::<v::milliliter, v::liter>(prefix!(milli));
            test::<v::microliter, v::liter>(prefix!(micro));
            test::<v::nanoliter, v::liter>(prefix!(nano));
            test::<v::picoliter, v::liter>(prefix!(pico));
            test::<v::femtoliter, v::liter>(prefix!(femto));
            test::<v::attoliter, v::liter>(prefix!(atto));
            test::<v::zeptoliter, v::liter>(prefix!(zepto));
            test::<v::yoctoliter, v::liter>(prefix!(yocto));

            fn test<T: v::Conversion<V>, U: v::Conversion<V>>(ratio: f64) {
                Test::assert_eq(&Volume::new::<T>(V::one()),
                    &Volume::new::<U>(V::from_f64(ratio).unwrap()));
            }
        }

        #[test]
        fn check_roman() {
            test::<v::ligula>(1.0_E0 / 2.88_E2);
            test::<v::cyathus>(1.0_E0 / 7.2_E1);
            test::<v::acetabulum>(1.0_E0 / 4.8_E1);
            test::<v::quartarius>(1.0_E0 / 2.4_E1);
            test::<v::hemina>(1.0_E0 / 1.2_E1);
            test::<v::sextarius>(1.0_E0 / 6.0_E0);
            test::<v::congius>(1.0_E0);
            test::<v::semimodius>(1.0_E0 + 1.0_E0 / 3.0_E0);
            test::<v::modius>(2.0_E0 + 2.0_E0 / 3.0_E0);
            test::<v::modius_castrensis>(4.0_E0);
            test::<v::urna>(4.0_E0);
            test::<v::amphora_quadrantal>(8.0_E0);
            test::<v::culeus>(160.0_E0);

            fn test<T: v::Conversion<V>>(c: f64) {
                Test::assert_approx_eq(&Volume::new::<T>(V::one()),
                    &Volume::new::<v::congius>(V::from_f64(c).unwrap()));
            }
        }
    }
}
