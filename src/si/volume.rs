//! Volume (base unit cubic meter, m³).

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
        @cup: 2.365_882_E-4; "cup", "cup", "cups";
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
        @tablespoon: 1.478_676_E-5; "tbsp", "tablespoon", "tablespoons";
        @teaspoon: 4.928_922_E-6; "tsp", "teaspoon", "teaspoons";
        @register_ton: 2.831_685_E0; "RT", "register ton", "register tons";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use lib::any::TypeId;
        use num::{FromPrimitive, One};
        use si::quantities::*;
        use si::area as a;
        use si::volume as v;
        use si::length as l;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Volume<V> = Length::new::<l::meter>(V::one())
                * Length::new::<l::meter>(V::one())
                * Length::new::<l::meter>(V::one());
            let _: Volume<V> = Length::new::<l::meter>(V::one())
                * Area::new::<a::square_meter>(V::one());
        }

        #[test]
        fn check_liters() {
            // Test liter base relative to cubic meter base to verify a baseline
            test::<v::liter, v::cubic_meter>(V::from_f64(prefix!(milli)).unwrap());
            // Test relative to liter to make sure prefixes are good
            // This transitively verifies the other relations
            test::<v::yottaliter, v::liter>(V::from_f64(prefix!(yotta)).unwrap());
            test::<v::zettaliter, v::liter>(V::from_f64(prefix!(zetta)).unwrap());
            test::<v::exaliter, v::liter>(V::from_f64(prefix!(exa)).unwrap());
            test::<v::petaliter, v::liter>(V::from_f64(prefix!(peta)).unwrap());
            test::<v::teraliter, v::liter>(V::from_f64(prefix!(tera)).unwrap());
            test::<v::gigaliter, v::liter>(V::from_f64(prefix!(giga)).unwrap());
            test::<v::megaliter, v::liter>(V::from_f64(prefix!(mega)).unwrap());
            test::<v::kiloliter, v::liter>(V::from_f64(prefix!(kilo)).unwrap());
            test::<v::hectoliter, v::liter>(V::from_f64(prefix!(hecto)).unwrap());
            test::<v::decaliter, v::liter>(V::from_f64(prefix!(deca)).unwrap());
            test::<v::liter, v::liter>(V::one());
            test::<v::deciliter, v::liter>(V::from_f64(prefix!(deci)).unwrap());
            test::<v::centiliter, v::liter>(V::from_f64(prefix!(centi)).unwrap());
            test::<v::milliliter, v::liter>(V::from_f64(prefix!(milli)).unwrap());
            test::<v::microliter, v::liter>(V::from_f64(prefix!(micro)).unwrap());
            test::<v::nanoliter, v::liter>(V::from_f64(prefix!(nano)).unwrap());
            test::<v::picoliter, v::liter>(V::from_f64(prefix!(pico)).unwrap());
            test::<v::femtoliter, v::liter>(V::from_f64(prefix!(femto)).unwrap());
            test::<v::attoliter, v::liter>(V::from_f64(prefix!(atto)).unwrap());
            test::<v::zeptoliter, v::liter>(V::from_f64(prefix!(zepto)).unwrap());
            test::<v::yoctoliter, v::liter>(V::from_f64(prefix!(yocto)).unwrap());

            fn test<T: v::Conversion<V>, U: v::Conversion<V>>(ratio: V) {
                Test::assert_eq(&Volume::new::<T>(V::one()), &Volume::new::<U>(ratio))
            }
        }

        #[test]
        fn check_units() {
            // Values too large for f32.
            if TypeId::of::<f64>() == TypeId::of::<V>() {
                test::<l::yottameter, v::cubic_yottameter>();
                test::<l::zettameter, v::cubic_zettameter>();
                test::<l::exameter, v::cubic_exameter>();
                test::<l::petameter, v::cubic_petameter>();
            }

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

            // Values too small for f32.
            if TypeId::of::<f64>() == TypeId::of::<V>() {
                test::<l::attometer, v::cubic_attometer>();
                test::<l::zeptometer, v::cubic_zeptometer>();
                test::<l::yoctometer, v::cubic_yoctometer>();
            }

            fn test<L: l::Conversion<V>, O: v::Conversion<V>>() {
                Test::assert_eq(&Volume::new::<O>(V::one()),
                    &(Length::new::<L>(V::one())
                        * Length::new::<L>(V::one())
                        * Length::new::<L>(V::one())));
            }
        }
    }
}
