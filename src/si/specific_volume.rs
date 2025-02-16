//! Specific volume (base unit cubic meter per kilogram, m³ · kg⁻¹).

quantity! {
    /// Specific volume (base unit cubic meter per kilogram, m³ · kg⁻¹).
    quantity: SpecificVolume; "specific volume";
    /// Dimension of specific volume, L³M⁻¹ (base unit cubic meter per kilogram, m³ · kg⁻¹).
    dimension: ISQ<
        P3,     // length
        N1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {

        @cubic_meter_per_yottagram: prefix!(kilo) / prefix!(yotta); "m³/Yg",
            "cubic meter per yottagram", "cubic meters per yottagram";
        @cubic_meter_per_zettagram: prefix!(kilo) / prefix!(zetta); "m³/Zg",
            "cubic meter per zettagram", "cubic meters per zettagram";
        @cubic_meter_per_exagram: prefix!(kilo) / prefix!(exa); "m³/Eg",
            "cubic meter per exagram", "cubic meters per exagram";
        @cubic_meter_per_petagram: prefix!(kilo) / prefix!(peta); "m³/Pg",
            "cubic meter per petagram", "cubic meters per petagram";
        @cubic_meter_per_teragram: prefix!(kilo) / prefix!(tera); "m³/Tg",
            "cubic meter per teragram", "cubic meters per teragram";
        @cubic_meter_per_gigagram: prefix!(kilo) / prefix!(giga); "m³/Gg",
            "cubic meter per gigagram", "cubic meters per gigagram";
        @cubic_meter_per_megagram: prefix!(kilo) / prefix!(mega); "m³/Mg",
            "cubic meter per megagram", "cubic meters per megagram";
        /// Derived unit of specific volume.
        @cubic_meter_per_kilogram: prefix!(kilo) / prefix!(kilo); "m³/kg",
            "cubic meter per kilogram", "cubic meters per kilogram";
        @cubic_meter_per_hectogram: prefix!(kilo) / prefix!(hecto); "m³/hg",
            "cubic meter per hectogram", "cubic meters per hectogram";
        @cubic_meter_per_decagram: prefix!(kilo) / prefix!(deca); "m³/dag",
            "cubic meter per decagram", "cubic meters per decagram";
        @cubic_meter_per_gram: prefix!(kilo) / prefix!(none); "m³/g",
            "cubic meter per gram", "cubic meters per gram";
        @cubic_meter_per_decigram: prefix!(kilo) / prefix!(deci); "m³/dg",
            "cubic meter per decigram", "cubic meters per decigram";
        @cubic_meter_per_centigram: prefix!(kilo) / prefix!(centi); "m³/cg",
        "cubic meter per centigram", "cubic meters per centigram";
        @cubic_meter_per_milligram: prefix!(kilo) / prefix!(milli); "m³/mg",
            "cubic meter per milligram", "cubic meters per milligram";
        @cubic_meter_per_microgram: prefix!(kilo) / prefix!(micro); "m³/µg",
            "cubic meter per microgram", "cubic meters per microgram";
        @cubic_meter_per_nanogram: prefix!(kilo) / prefix!(nano); "m³/ng",
            "cubic meter per nanogram", "cubic meters per nanogram";
        @cubic_meter_per_picogram: prefix!(kilo) / prefix!(pico); "m³/pg",
            "cubic meter per picogram", "cubic meters per picogram";
        @cubic_meter_per_femtogram: prefix!(kilo) / prefix!(femto); "m³/fg",
            "cubic meter per femtogram", "cubic meters per femtogram";
        @cubic_meter_per_attogram: prefix!(kilo) / prefix!(atto); "m³/ag",
            "cubic meter per attogram", "cubic meters per attogram";
        @cubic_meter_per_zeptogram: prefix!(kilo) / prefix!(zepto); "m³/zg",
            "cubic meter per zeptogram", "cubic meters per zeptogram";
        @cubic_meter_per_yoctogram: prefix!(kilo) / prefix!(yocto); "m³/yg",
            "cubic meter per yoctogram", "cubic meters per yoctogram";

        @cubic_meter_per_carat: 5.0_E3; "m³/ct",
            "cubic meter per carat", "cubic meters per carat";
        @cubic_meter_per_grain: 1.543_235_835_294_143_2_E4; "m³/gr",
            "cubic meter per grain", "cubic meters per grain";
        @cubic_meter_per_hundredweight_long: 1.968_412_878_538_099_E-2; "m³/cwt long",
            "cubic meter per hundredweight (long)", "cubic meters per hundredweight (long)";
        @cubic_meter_per_hundredweight_short:2.204_622_476_037_958_2_E-2; "m³/cwt short",
            "cubic meter per hundredweight (short)", "cubic meters per hundredweight (short)";
        @cubic_meter_per_ounce: 3.527_396_583_786_957_E1; "m³/oz",
            "cubic meter per ounce", "cubic meters per ounce";
        @cubic_meter_per_ounce_troy: 3.215_074_326_088_270_6_E1; "m³/oz t",
            "cubic meter per troy ounce", "cubic meters per troy ounce";
        @cubic_meter_per_pennyweight: 6.430_14_865_217_654_E2; "m³/dwt",
            "cubic meter per pennyweight", "cubic meters per pennyweight";
        @cubic_meter_per_pound: 2.204_622_476_037_958_5_E0; "m³/lb",
            "cubic meter per pound", "cubic meters per pound";
        @cubic_meter_per_pound_troy: 2.679_229_035_769_583_E0; "m³/lb t",
            "cubic meter per troy pound", "cubic meters per troy pound";
        @cubic_meter_per_slug: 6.852_177_964_766_101_E-2; "m³/slug",
            "cubic meter per slug", "cubic meters per slug";
        @cubic_meter_per_ton_assay: 3.428_571_036_734_739_E1; "m³/AT",
            "cubic meter per assay ton", "cubic meters per assay ton";
        @cubic_meter_per_ton_long: 9.842_064_392_690_496_E-4; "m³/2240 lb",
            "cubic meter per long ton", "cubic meters per long ton";
        @cubic_meter_per_ton_short: 1.102_311_359_527_999_E-3; "m³/2000 lb",
            "cubic meter per short ton", "cubic meters per short ton";
        @cubic_meter_per_ton: 1.0_E-3; "m³/t",
            "cubic meter per ton", "cubic meters per ton";

        @gallon_per_grain: 5.841_783_449_752_474_E1; "gal/gr", "gallon per grain",
            "gallons per grain";
        @cubic_centimeter_per_gram:
            prefix!(centi) * prefix!(centi) * prefix!(centi) * prefix!(kilo); "cm³/g",
            "cubic centimeter per gram", "cubic centimeters per gram";
        @cubic_centimeter_per_kilogram: prefix!(centi) * prefix!(centi) * prefix!(centi); "cm³/kg",
            "cubic centimeter per kilogram", "cubic centimeters per kilogram";
        @cubic_inch_per_ounce: 5.780_365_946_231_19_E-4; "in³/oz", "cubic inch per ounce",
            "cubic inches per ounce";
        @gallon_imperial_per_ounce:1.603_586_233_558_804_4_E-1; "gal (UK)/oz",
            "Imperial gallon per ounce", "Imperial gallons per ounce";
        @gallon_per_ounce: 1.335_264_935_702_615_E-1; "gal/oz", "gallon per ounce",
            "gallons per ounce";
        @cubic_foot_per_pound: 6.24_279_639_605_954_6_E-2; "ft³/lb", "cubic foot per pound",
            "cubic feet per pound";
        @cubic_inch_per_pound: 3.612_728_079_218_259_E-5; "in³/lb", "cubic inch per pound",
            "cubic inches per pound";
        @cubic_yard_per_pound: 1.685_554_916_704_953_7_E0; "yd³/lb", "cubic yard per pound",
            "cubic yards per pound";
        @gallon_imperial_per_pound: 1.002_241_219_209_140_1_E-2; "gal (UK)/lb",
            "Imperial gallon per pound", "Imperial gallons per pound";
        @gallon_per_pound: 8.345_404_376_263_8_E-3; "gal/lb", "gallon per pound",
            "gallons per pound";
        @cubic_foot_per_slug: 1.940_320_956_015_869_7_E-3; "ft³/slug", "cubic foot per slug",
            "cubic feett per slug";
        @cubic_yard_per_ton_long: 7.524_798_557_547_043_E-4; "yd³/2240 lb",
            "cubic yard per long ton", "cubic yards per long ton";
        @cubic_yard_per_ton_short: 8.427_775_512_527_934_E-4; "yd³/2000 lb",
            "cubic yard per short ton", "cubic yards per short ton";
        @cubic_foot_per_ton_long: 2.786_962_610_981_578_5_E-5; "ft³/2240 lb",
            "cubic foot per long ton", "cubic feet per long ton";
        @cubic_foot_per_ton_short: 3.121_398_542_105_042_4E-5; "ft³/2000 lb",
            "cubic foot per short ton", "cubic feet per short ton";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::volume as a;
        use crate::si::mass as m;
        use crate::si::specific_volume as sv;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: SpecificVolume<V> = Volume::new::<a::cubic_meter>(V::one())
                / Mass::new::<m::kilogram>(V::one());
        }

        #[test]
        fn check_units() {
            test::<a::cubic_meter, m::yottagram, sv::cubic_meter_per_yottagram>();
            test::<a::cubic_meter, m::zettagram, sv::cubic_meter_per_zettagram>();
            test::<a::cubic_meter, m::exagram, sv::cubic_meter_per_exagram>();
            test::<a::cubic_meter, m::petagram, sv::cubic_meter_per_petagram>();
            test::<a::cubic_meter, m::teragram, sv::cubic_meter_per_teragram>();
            test::<a::cubic_meter, m::gigagram, sv::cubic_meter_per_gigagram>();
            test::<a::cubic_meter, m::megagram, sv::cubic_meter_per_megagram>();
            test::<a::cubic_meter, m::kilogram, sv::cubic_meter_per_kilogram>();
            test::<a::cubic_meter, m::hectogram, sv::cubic_meter_per_hectogram>();
            test::<a::cubic_meter, m::decagram, sv::cubic_meter_per_decagram>();
            test::<a::cubic_meter, m::gram, sv::cubic_meter_per_gram>();
            test::<a::cubic_meter, m::decigram, sv::cubic_meter_per_decigram>();
            test::<a::cubic_meter, m::centigram, sv::cubic_meter_per_centigram>();
            test::<a::cubic_meter, m::milligram, sv::cubic_meter_per_milligram>();
            test::<a::cubic_meter, m::microgram, sv::cubic_meter_per_microgram>();
            test::<a::cubic_meter, m::nanogram, sv::cubic_meter_per_nanogram>();
            test::<a::cubic_meter, m::picogram, sv::cubic_meter_per_picogram>();
            test::<a::cubic_meter, m::femtogram, sv::cubic_meter_per_femtogram>();
            test::<a::cubic_meter, m::attogram, sv::cubic_meter_per_attogram>();
            test::<a::cubic_meter, m::zeptogram, sv::cubic_meter_per_zeptogram>();
            test::<a::cubic_meter, m::yoctogram, sv::cubic_meter_per_yoctogram>();
            test::<a::cubic_meter, m::carat, sv::cubic_meter_per_carat>();
            test::<a::cubic_meter, m::grain, sv::cubic_meter_per_grain>();
            test::<a::cubic_meter, m::hundredweight_long,
                sv::cubic_meter_per_hundredweight_long>();
            test::<a::cubic_meter, m::hundredweight_short,
                sv::cubic_meter_per_hundredweight_short>();
            test::<a::cubic_meter, m::ounce, sv::cubic_meter_per_ounce>();
            test::<a::cubic_meter, m::ounce_troy, sv::cubic_meter_per_ounce_troy>();
            test::<a::cubic_meter, m::pennyweight, sv::cubic_meter_per_pennyweight>();
            test::<a::cubic_meter, m::pound, sv::cubic_meter_per_pound>();
            test::<a::cubic_meter, m::pound_troy, sv::cubic_meter_per_pound_troy>();
            test::<a::cubic_meter, m::slug, sv::cubic_meter_per_slug>();
            test::<a::cubic_meter, m::ton_assay, sv::cubic_meter_per_ton_assay>();
            test::<a::cubic_meter, m::ton_long, sv::cubic_meter_per_ton_long>();
            test::<a::cubic_meter, m::ton_short, sv::cubic_meter_per_ton_short>();
            test::<a::cubic_meter, m::ton, sv::cubic_meter_per_ton>();

            test::<a::gallon, m::grain, sv::gallon_per_grain>();
            test::<a::cubic_centimeter, m::gram, sv::cubic_centimeter_per_gram>();
            test::<a::cubic_centimeter, m::kilogram, sv::cubic_centimeter_per_kilogram>();
            test::<a::cubic_inch, m::ounce, sv::cubic_inch_per_ounce>();
            test::<a::gallon_imperial, m::ounce, sv::gallon_imperial_per_ounce>();
            test::<a::gallon, m::ounce, sv::gallon_per_ounce>();
            test::<a::cubic_foot, m::pound, sv::cubic_foot_per_pound>();
            test::<a::cubic_inch, m::pound, sv::cubic_inch_per_pound>();
            test::<a::cubic_yard, m::pound, sv::cubic_yard_per_pound>();
            test::<a::gallon_imperial, m::pound, sv::gallon_imperial_per_pound>();
            test::<a::gallon, m::pound, sv::gallon_per_pound>();
            test::<a::cubic_foot, m::slug, sv::cubic_foot_per_slug>();
            test::<a::cubic_yard, m::ton_long, sv::cubic_yard_per_ton_long>();
            test::<a::cubic_yard, m::ton_short, sv::cubic_yard_per_ton_short>();
            test::<a::cubic_foot, m::ton_long, sv::cubic_foot_per_ton_long>();
            test::<a::cubic_foot, m::ton_short, sv::cubic_foot_per_ton_short>();

            fn test<A: a::Conversion<V>, M: m::Conversion<V>, SV: sv::Conversion<V>>() {
                Test::assert_approx_eq(&SpecificVolume::new::<SV>(V::one()),
                    &(Volume::new::<A>(V::one()) / Mass::new::<M>(V::one())));
            }
        }
    }
}
