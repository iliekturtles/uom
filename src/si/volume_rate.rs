//! Volume rate (base unit cubic meter per second, m³ · s⁻¹).

quantity! {
    /// Volume rate (base unit cubic meter per second, m³ · s⁻¹).
    quantity: VolumeRate; "volume rate";
    /// Dimension of volume rate, L³T⁻¹ (base unit cubic meter per second, m³ · s⁻¹).
    dimension: ISQ<
        P3,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @cubic_yottameter_per_second: prefix!(yotta) * prefix!(yotta) * prefix!(yotta);
            "Ym³/s", "cubic yottameter per second", "cubic yottameters per second";
        @cubic_zettameter_per_second: prefix!(zetta) * prefix!(zetta) * prefix!(zetta);
            "Zm³/s", "cubic zettameter per second", "cubic zettameters per second";
        @cubic_exameter_per_second: prefix!(exa) * prefix!(exa) * prefix!(exa);
            "Em³/s", "cubic exameter per second", "cubic exameters per second";
        @cubic_petameter_per_second: prefix!(peta) * prefix!(peta) * prefix!(peta);
            "Pm³/s", "cubic petameter per second", "cubic petameters per second";
        @cubic_terameter_per_second: prefix!(tera) * prefix!(tera) * prefix!(tera);
            "Tm³/s", "cubic terameter per second", "cubic terameters per second";
        @cubic_gigameter_per_second: prefix!(giga) * prefix!(giga) * prefix!(giga);
            "Gm³/s", "cubic gigameter per second", "cubic gigameters per second";
        @cubic_megameter_per_second: prefix!(mega) * prefix!(mega) * prefix!(mega);
            "Mm³/s", "cubic megameter per second", "cubic megameters per second";
        @cubic_kilometer_per_second: prefix!(kilo) * prefix!(kilo) * prefix!(kilo);
            "km³/s", "cubic kilometer per second", "cubic kilometers per second";
        @cubic_hectometer_per_second: prefix!(hecto) * prefix!(hecto) * prefix!(hecto);
            "hm³/s", "cubic hectometer per second", "cubic hectometers per second";
        @cubic_decameter_per_second: prefix!(deca) * prefix!(deca) * prefix!(deca);
            "dam³/s", "cubic decameter per second", "cubic decameters per second";
        @cubic_meter_per_second: prefix!(none); "m³/s", "cubic meter per second",
            "cubic meters per second";
        @cubic_decimeter_per_second: prefix!(deci) * prefix!(deci) * prefix!(deci);
            "dm³/s", "cubic decimeter per second", "cubic decimeters per second";
        @cubic_centimeter_per_second: prefix!(centi) * prefix!(centi) * prefix!(centi);
            "cm³/s", "cubic centimeter per second", "cubic centimeters per second";
        @cubic_millimeter_per_second: prefix!(milli) * prefix!(milli) * prefix!(milli);
            "mm³/s", "cubic millimeter per second", "cubic millimeters per second";
        @cubic_micrometer_per_second: prefix!(micro) * prefix!(micro) * prefix!(micro);
            "µm³/s", "cubic micrometer per second", "cubic micrometers per second";
        @cubic_nanometer_per_second: prefix!(nano) * prefix!(nano) * prefix!(nano);
            "nm³/s", "cubic nanometer per second", "cubic nanometers per second";
        @cubic_picometer_per_second: prefix!(pico) * prefix!(pico) * prefix!(pico);
            "pm³/s", "cubic picometer per second", "cubic picometers per second";
        @cubic_femtometer_per_second: prefix!(femto) * prefix!(femto) * prefix!(femto);
            "fm³/s", "cubic femtometer per second", "cubic femtometers per second";
        @cubic_attometer_per_second: prefix!(atto) * prefix!(atto) * prefix!(atto);
            "am³/s", "cubic attometer per second", "cubic attometers per second";
        @cubic_zeptometer_per_second: prefix!(zepto) * prefix!(zepto) * prefix!(zepto);
            "zm³/s", "cubic zeptometer per second", "cubic zeptometers per second";
        @cubic_yoctometer_per_second: prefix!(yocto) * prefix!(yocto) * prefix!(yocto);
            "ym³/s", "cubic yoctometer per second", "cubic yoctometers per second";

        @yottaliter_per_second: prefix!(milli) * prefix!(yotta); "YL/s", "yottaliter per second",
            "yottaliters per second";
        @zettaliter_per_second: prefix!(milli) * prefix!(zetta); "ZL/s", "zettaliter per second",
            "zettaliters per second";
        @exaliter_per_second: prefix!(milli) * prefix!(exa); "EL/s", "exaliter per second",
            "exaliters per second";
        @petaliter_per_second: prefix!(milli) * prefix!(peta); "PL/s", "petaliter per second",
            "petaliters per second";
        @teraliter_per_second: prefix!(milli) * prefix!(tera); "TL/s", "teraliter per second",
            "teraliters per second";
        @gigaliter_per_second: prefix!(milli) * prefix!(giga); "GL/s", "gigaliter per second",
            "gigaliters per second";
        @megaliter_per_second: prefix!(milli) * prefix!(mega); "ML/s", "megaliter per second",
            "megaliters per second";
        @kiloliter_per_second: prefix!(milli) * prefix!(kilo); "kL/s", "kiloliter per second",
            "kiloliters per second";
        @hectoliter_per_second: prefix!(milli) * prefix!(hecto); "hL/s", "hectoliter per second",
            "hectoliters per second";
        @decaliter_per_second: prefix!(milli) * prefix!(deca); "daL/s", "decaliter per second",
            "decaliters per second";
        @liter_per_second: prefix!(milli); "L/s", "liter per second", "liters per second";
        @deciliter_per_second: prefix!(milli) * prefix!(deci); "dL/s", "deciliter per second",
            "deciliters per second";
        @centiliter_per_second: prefix!(milli) * prefix!(centi); "cL/s", "centiliter per second",
            "centiliters per second";
        @milliliter_per_second: prefix!(milli) * prefix!(milli); "mL/s", "milliliter per second",
            "milliliters per second";
        @microliter_per_second: prefix!(milli) * prefix!(micro); "µL/s", "microliter per second",
            "microliters per second";
        @nanoliter_per_second: prefix!(milli) * prefix!(nano); "nL/s", "nanoliter per second",
            "nanoliters per second";
        @picoliter_per_second: prefix!(milli) * prefix!(pico); "pL/s", "picoliter per second",
            "picoliters per second";
        @femtoliter_per_second: prefix!(milli) * prefix!(femto); "fL/s", "femtoliter per second",
            "femtoliters per second";
        @attoliter_per_second: prefix!(milli) * prefix!(atto); "aL/s", "attoliter per second",
            "attoliters per second";
        @zeptoliter_per_second: prefix!(milli) * prefix!(zepto); "zL/s", "zeptoliter per second",
            "zeptoliters per second";
        @yoctoliter_per_second: prefix!(milli) * prefix!(yocto); "yL/s", "yoctoliter per second",
            "yoctoliters per second";

        @acre_foot_per_second: 1.233_489_E3; "ac · ft/s", "acre-foot per second",
            "acre-feet per second";
        @barrel_per_second: 1.589_873_E-1; "bbl/s", "barrel per second", "barrels per second";
        @bushel_per_second: 3.523_907_E-2; "bu/s", "bushel per second", "bushels per second";
        @cord_per_second: 3.624_556_E0; "cords/s", "cord per second", "cords per second";
        @cubic_foot_per_second: 2.831_685_E-2; "ft³/s", "cubic foot per second",
            "cubic feet per second";
        @cubic_foot_per_minute: 4.719_474_E-4; "ft³/min", "cubic foot per minute",
            "cubic feet per minute";
        @cubic_inch_per_second: 1.638_706_E-5; "in³/s", "cubic inch per second",
            "cubic inches per second";
        @cubic_inch_per_minute: 2.731_177_E-7; "in³/min", "cubic inch per minute",
            "cubic inches per minute";
        @cubic_mile_per_second: 4.168_182_E9; "mi³/s", "cubic mile per second",
            "cubic miles per second";
        @cubic_yard_per_second: 7.645_549_E-1; "yd³/s", "cubic yard per second",
            "cubic yards per second";
        @cubic_yard_per_minute: 1.274_258E-2; "yd³/min", "cubic yard per minute",
            "cubic yards per minute";
        @cup_per_second: 2.365_882_E-4; "cup/s", "cup per second", "cups per second";
        @fluid_ounce_per_second: 2.957_353_E-5; "fl oz/s", "fluid ounce per second",
            "fluid ounces per second";
        @fluid_ounce_imperial_per_second: 2.841_306_E-5; "fl oz (UK)/s",
            "Imperial fluid ounce per second", "Imperial fluid ounces per second";
        @gallon_imperial_per_second: 4.546_09_E-3; "gal (UK)/s", "Imperial gallon per second",
            "Imperial gallons per second";
        @gallon_per_second: 3.785_412_E-3; "gal/s", "gallon per second", "gallons per second";
        @gallon_per_minute: 6.309_020_E-5; "gal/min", "gallon per minute", "gallons per minute";
        @gallon_per_day: 4.381_264_E-8; "gal/d", "gallon per day", "gallons per day";
        @gill_imperial_per_second: 1.420_653_E-4; "gi (UK)/s", "Imperial gill per second",
            "Imperial gills per second";
        @gill_per_second: 1.182_941_E-4; "gi/s", "gill per second", "gills per second";
        @peck_per_second: 8.809_768_E-3; "pk/s", "peck per second", "pecks per second";
        @pint_dry_per_second: 5.506_105_E-4; "dry pt/s", "dry pint per second",
            "dry pints per second";
        @pint_liquid_per_second: 4.731_765_E-4; "liq pt/s", "liquid pint per second",
            "liquid pints per second";
        @quart_dry_per_second: 1.101_221_E-3; "dry qt/s", "dry quart per second",
            "dry quarts per second";
        @quart_liquid_per_second: 9.463_529_E-4; "liq qt/s", "liquid quart per second",
            "liquid quarts per second";
        @stere_per_second: 1.0_E0; "st/s", "stere per second", "steres per second";
        @tablespoon_per_second: 1.478_676_E-5; "tbsp/s", "tablespoon per second",
            "tablespoons per second";
        @teaspoon_per_second: 4.928_922_E-6; "tsp/s", "teaspoon per second",
            "teaspoons per second";
        @register_ton_per_second: 2.831_685_E0; "RT/s", "register ton per second",
            "register tons per second";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use lib::any::TypeId;
        use num::One;
        use si::quantities::*;
        use si::time as t;
        use si::volume as v;
        use si::volume_rate as r;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: VolumeRate<V> = Volume::new::<v::cubic_meter>(V::one())
                / Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            // Values too large for f32.
            if TypeId::of::<f64>() == TypeId::of::<V>() {
                test::<v::cubic_yottameter, r::cubic_yottameter_per_second>();
                test::<v::cubic_zettameter, r::cubic_zettameter_per_second>();
                test::<v::cubic_exameter, r::cubic_exameter_per_second>();
                test::<v::cubic_petameter, r::cubic_petameter_per_second>();
            }

            test::<v::cubic_terameter, r::cubic_terameter_per_second>();
            test::<v::cubic_gigameter, r::cubic_gigameter_per_second>();
            test::<v::cubic_megameter, r::cubic_megameter_per_second>();
            test::<v::cubic_kilometer, r::cubic_kilometer_per_second>();
            test::<v::cubic_hectometer, r::cubic_hectometer_per_second>();
            test::<v::cubic_decameter, r::cubic_decameter_per_second>();
            test::<v::cubic_meter, r::cubic_meter_per_second>();
            test::<v::cubic_decimeter, r::cubic_decimeter_per_second>();
            test::<v::cubic_centimeter, r::cubic_centimeter_per_second>();
            test::<v::cubic_millimeter, r::cubic_millimeter_per_second>();
            test::<v::cubic_micrometer, r::cubic_micrometer_per_second>();
            test::<v::cubic_nanometer, r::cubic_nanometer_per_second>();
            test::<v::cubic_picometer, r::cubic_picometer_per_second>();
            test::<v::cubic_femtometer, r::cubic_femtometer_per_second>();

            // Values too small for f32.
            if TypeId::of::<f64>() == TypeId::of::<V>() {
                test::<v::cubic_attometer, r::cubic_attometer_per_second>();
                test::<v::cubic_zeptometer, r::cubic_zeptometer_per_second>();
                test::<v::cubic_yoctometer, r::cubic_yoctometer_per_second>();
            }

            test::<v::acre_foot, r::acre_foot_per_second>();
            test::<v::barrel, r::barrel_per_second>();
            test::<v::bushel, r::bushel_per_second>();
            test::<v::cord, r::cord_per_second>();
            test::<v::cubic_foot, r::cubic_foot_per_second>();
            test::<v::cubic_inch, r::cubic_inch_per_second>();
            test::<v::cubic_mile, r::cubic_mile_per_second>();
            test::<v::cubic_yard, r::cubic_yard_per_second>();
            test::<v::cup, r::cup_per_second>();
            test::<v::fluid_ounce, r::fluid_ounce_per_second>();
            test::<v::fluid_ounce_imperial, r::fluid_ounce_imperial_per_second>();
            test::<v::gallon_imperial, r::gallon_imperial_per_second>();
            test::<v::gallon, r::gallon_per_second>();
            test::<v::gill_imperial, r::gill_imperial_per_second>();
            test::<v::gill, r::gill_per_second>();
            test::<v::peck, r::peck_per_second>();
            test::<v::pint_dry, r::pint_dry_per_second>();
            test::<v::pint_liquid, r::pint_liquid_per_second>();
            test::<v::quart_dry, r::quart_dry_per_second>();
            test::<v::quart_liquid, r::quart_liquid_per_second>();
            test::<v::stere, r::stere_per_second>();
            test::<v::tablespoon, r::tablespoon_per_second>();
            test::<v::teaspoon, r::teaspoon_per_second>();
            test::<v::register_ton, r::register_ton_per_second>();

            test::<v::yottaliter, r::yottaliter_per_second>();
            test::<v::zettaliter, r::zettaliter_per_second>();
            test::<v::exaliter, r::exaliter_per_second>();
            test::<v::petaliter, r::petaliter_per_second>();
            test::<v::teraliter, r::teraliter_per_second>();
            test::<v::gigaliter, r::gigaliter_per_second>();
            test::<v::megaliter, r::megaliter_per_second>();
            test::<v::kiloliter, r::kiloliter_per_second>();
            test::<v::hectoliter, r::hectoliter_per_second>();
            test::<v::decaliter, r::decaliter_per_second>();
            test::<v::liter, r::liter_per_second>();
            test::<v::deciliter, r::deciliter_per_second>();
            test::<v::centiliter, r::centiliter_per_second>();
            test::<v::milliliter, r::milliliter_per_second>();
            test::<v::microliter, r::microliter_per_second>();
            test::<v::nanoliter, r::nanoliter_per_second>();
            test::<v::picoliter, r::picoliter_per_second>();
            test::<v::femtoliter, r::femtoliter_per_second>();
            test::<v::attoliter, r::attoliter_per_second>();
            test::<v::zeptoliter, r::zeptoliter_per_second>();
            test::<v::yoctoliter, r::yoctoliter_per_second>();

            fn test<O: v::Conversion<V>, R: r::Conversion<V>>() {
                Test::assert_eq(&VolumeRate::new::<R>(V::one()),
                    &(Volume::new::<O>(V::one()) / Time::new::<t::second>(V::one())));
            }
        }
    }
}
