//! Mass density (base unit kilogram per cubic meter, kg · m⁻³).

/// Mass density (base unit kilogram per cubic meter, kg · m⁻³).
#[deprecated(since = "0.23.0", note = "Please use the more descriptive MassDensity instead.")]
pub type Density<U, V> = MassDensity<U, V>;

quantity! {
    /// Mass density (base unit kilogram per cubic meter, kg · m⁻³).
    quantity: MassDensity; "mass density";
    /// Dimension of mass density, L⁻³M (base unit kilogram per cubic meter, kg · m⁻³).
    dimension: ISQ<
        N3,     // length
        P1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottagram_per_cubic_meter: prefix!(yotta) / prefix!(kilo); "Yg/m³",
            "yottagram per cubic meter", "yottagrams per cubic meter";
        @zettagram_per_cubic_meter: prefix!(zetta) / prefix!(kilo); "Zg/m³",
            "zettagram per cubic meter", "zettagrams per cubic meter";
        @exagram_per_cubic_meter: prefix!(exa) / prefix!(kilo); "Eg/m³",
            "exagram per cubic meter", "exagrams per cubic meter";
        @petagram_per_cubic_meter: prefix!(peta) / prefix!(kilo); "Pg/m³",
            "petagram per cubic meter", "petagrams per cubic meter";
        @teragram_per_cubic_meter: prefix!(tera) / prefix!(kilo); "Tg/m³",
            "teragram per cubic meter", "teragrams per cubic meter";
        @gigagram_per_cubic_meter: prefix!(giga) / prefix!(kilo); "Gg/m³",
            "gigagram per cubic meter", "gigagrams per cubic meter";
        @megagram_per_cubic_meter: prefix!(mega) / prefix!(kilo); "Mg/m³",
            "megagram per cubic meter", "megagrams per cubic meter";
        /// Derived unit of density.
        @kilogram_per_cubic_meter: prefix!(kilo) / prefix!(kilo); "kg/m³",
            "kilogram per cubic meter", "kilograms per cubic meter";
        @hectogram_per_cubic_meter: prefix!(hecto) / prefix!(kilo); "hg/m³",
            "hectogram per cubic meter", "hectograms per cubic meter";
        @decagram_per_cubic_meter: prefix!(deca) / prefix!(kilo); "dag/m³",
            "decagram per cubic meter", "decagrams per cubic meter";
        @gram_per_cubic_meter: prefix!(none) / prefix!(kilo); "g/m³",
            "gram per cubic meter", "grams per cubic meter";
        @decigram_per_cubic_meter: prefix!(deci) / prefix!(kilo); "dg/m³",
            "decigram per cubic meter", "decigrams per cubic meter";
        @centigram_per_cubic_meter: prefix!(centi) / prefix!(kilo); "cg/m³",
            "centigram per cubic meter", "centigrams per cubic meter";
        @milligram_per_cubic_meter: prefix!(milli) / prefix!(kilo); "mg/m³",
            "milligram per cubic meter", "milligrams per cubic meter";
        @microgram_per_cubic_meter: prefix!(micro) / prefix!(kilo); "µg/m³",
            "microgram per cubic meter", "micrograms per cubic meter";
        @nanogram_per_cubic_meter: prefix!(nano) / prefix!(kilo); "ng/m³",
            "nanogram per cubic meter", "nanograms per cubic meter";
        @picogram_per_cubic_meter: prefix!(pico) / prefix!(kilo); "pg/m³",
            "picogram per cubic meter", "picograms per cubic meter";
        @femtogram_per_cubic_meter: prefix!(femto) / prefix!(kilo); "fg/m³",
            "femtogram per cubic meter", "femtograms per cubic meter";
        @attogram_per_cubic_meter: prefix!(atto) / prefix!(kilo); "ag/m³",
            "attogram per cubic meter", "attograms per cubic meter";
        @zeptogram_per_cubic_meter: prefix!(zepto) / prefix!(kilo); "zg/m³",
            "zeptogram per cubic meter", "zeptograms per cubic meter";
        @yoctogram_per_cubic_meter: prefix!(yocto) / prefix!(kilo); "yg/m³",
            "yoctogram per cubic meter", "yoctograms per cubic meter";

        @carat_per_cubic_meter: 2.0_E-4; "ct/m³", "carat per cubic meter",
            "carats per cubic meter";
        @grain_per_cubic_meter: 6.479_891_E-5; "gr/m³", "grain per cubic meter",
            "grains per cubic meter";
        @hundredweight_long_per_cubic_meter: 5.080_235_E1; "cwt long/m³",
            "hundredweight (long) per cubic meter", "hundredweight (long) per cubic meter";
        @hundredweight_short_per_cubic_meter: 4.535_924_E1; "cwt short/m³",
            "hundredweight (short) per cubic meter", "hundredweight (short) per cubic meter";
        @ounce_per_cubic_meter: 2.834_952_E-2; "oz/m³", "ounce per cubic meter",
            "ounces per cubic meter";
        @ounce_troy_per_cubic_meter: 3.110_348_E-2; "oz t/m³", "troy ounce per cubic meter",
            "troy ounces per cubic meter";
        @pennyweight_per_cubic_meter: 1.555_174_E-3; "dwt/m³", "pennyweight per cubic meter",
            "pennyweight per cubic meter";
        @pound_per_cubic_meter: 4.535_924_E-1; "lb/m³", "pound per cubic meter",
            "pounds per cubic meter";
        @pound_troy_per_cubic_meter: 3.732_417_E-1; "lb t/m³", "troy pound per cubic meter",
            "troy pounds per cubic meter";
        @slug_per_cubic_meter: 1.459_390_E1; "slug/m³", "slug per cubic meter",
            "slugs per cubic meter";
        @ton_assay_per_cubic_meter: 2.916_667_E-2; "AT/m³", "assay ton per cubic meter",
            "assay tons per cubic meter";
        @ton_long_per_cubic_meter: 1.016_047_E3; "2240 lb/m³", "long ton per cubic meter",
            "long tons per cubic meter";
        @ton_short_per_cubic_meter: 9.071_847_E2; "2000 lb/m³", "short ton per cubic meter",
            "short tons per cubic meter";
        @ton_per_cubic_meter: 1.0_E3; "t/m³", "ton per cubic meter",
            "tons per cubic meter"; // ton per cubic meter, metric

        @grain_per_gallon: 1.711_806_006_849_452_E-2; "gr/gal", "grain per gallon",
            "grains per gallon";
        @gram_per_cubic_centimeter: 1.0_E3; "g/cm³", "gram per cubic centimeter",
            "grams per cubic centimeter";
        @ounce_per_cubic_inch: 1.729_994_275_971_406_5_E3; "oz/in³", "ounce per cubic inch",
            "ounces per cubic inch";
        @ounce_per_gallon_imperial: 6.236_022_604_039_955_E0; "oz/gal (UK)",
            "ounce per Imperial gallon", "ounces per Imperial gallon";
        @ounce_per_gallon: 7.489_150_454_428_738_E0; "oz/gal", "ounce per gallon",
            "ounces per gallon";
        @pound_per_cubic_foot: 1.601_846_250_553_998_6_E1; "lb/ft³", "pound per cubic foot",
            "pounds per cubic foot";
        @pound_per_cubic_inch: 2.767_991_329_744_322_5_E4; "lb/in³", "pound per cubic inch",
            "pounds per cubic inch";
        @pound_per_cubic_yard: 5.932_764_278_928_825_E-1; "lb/yd³", "pound per cubic yard",
            "pounds per cubic yard";
        @pound_per_gallon_imperial: 9.977_637_926_217_915_E1; "lb/gal (UK)",
            "pound per Imperial gallon", "pounds per Imperial gallon";
        @pound_per_gallon: 1.198_264_284_046_228_E2; "lb/gal", "pound per gallon",
            "pounds per gallon";
        @slug_per_cubic_foot: 5.153_786_526_396_827_E2; "slug/ft³", "slug per cubic foot",
            "slugs per cubic foot";
        @ton_long_per_cubic_yard: 1.328_939_229_870_87_E3; "2240 lb/yd³", "long ton per cubic yard",
            "long tons per cubic yard";
        @ton_short_per_cubic_yard: 1.186_552_724_990_710_2_E3; "2000 lb/yd³",
            "short ton per cubic yard", "short tons per cubic yard";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::mass as m;
        use si::volume as v;
        use si::mass_density as d;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: MassDensity<V> = Mass::new::<m::kilogram>(V::one())
                / Volume::new::<v::cubic_meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<m::yottagram, v::cubic_meter, d::yottagram_per_cubic_meter>();
            test::<m::zettagram, v::cubic_meter, d::zettagram_per_cubic_meter>();
            test::<m::exagram, v::cubic_meter, d::exagram_per_cubic_meter>();
            test::<m::petagram, v::cubic_meter, d::petagram_per_cubic_meter>();
            test::<m::teragram, v::cubic_meter, d::teragram_per_cubic_meter>();
            test::<m::gigagram, v::cubic_meter, d::gigagram_per_cubic_meter>();
            test::<m::megagram, v::cubic_meter, d::megagram_per_cubic_meter>();
            test::<m::kilogram, v::cubic_meter, d::kilogram_per_cubic_meter>();
            test::<m::hectogram, v::cubic_meter, d::hectogram_per_cubic_meter>();
            test::<m::decagram, v::cubic_meter, d::decagram_per_cubic_meter>();
            test::<m::gram, v::cubic_meter, d::gram_per_cubic_meter>();
            test::<m::decigram, v::cubic_meter, d::decigram_per_cubic_meter>();
            test::<m::centigram, v::cubic_meter, d::centigram_per_cubic_meter>();
            test::<m::milligram, v::cubic_meter, d::milligram_per_cubic_meter>();
            test::<m::microgram, v::cubic_meter, d::microgram_per_cubic_meter>();
            test::<m::nanogram, v::cubic_meter, d::nanogram_per_cubic_meter>();
            test::<m::picogram, v::cubic_meter, d::picogram_per_cubic_meter>();
            test::<m::femtogram, v::cubic_meter, d::femtogram_per_cubic_meter>();
            test::<m::attogram, v::cubic_meter, d::attogram_per_cubic_meter>();
            test::<m::zeptogram, v::cubic_meter, d::zeptogram_per_cubic_meter>();
            test::<m::yoctogram, v::cubic_meter, d::yoctogram_per_cubic_meter>();

            test::<m::grain, v::gallon, d::grain_per_gallon>();
            test::<m::gram, v::cubic_centimeter, d::gram_per_cubic_centimeter>();
            test::<m::ounce, v::cubic_inch, d::ounce_per_cubic_inch>();
            test::<m::ounce, v::gallon_imperial, d::ounce_per_gallon_imperial>();
            test::<m::ounce, v::gallon, d::ounce_per_gallon>();
            test::<m::pound, v::cubic_foot, d::pound_per_cubic_foot>();
            test::<m::pound, v::cubic_inch, d::pound_per_cubic_inch>();
            test::<m::pound, v::cubic_yard, d::pound_per_cubic_yard>();
            test::<m::pound, v::gallon_imperial, d::pound_per_gallon_imperial>();
            test::<m::pound, v::gallon, d::pound_per_gallon>();
            test::<m::slug, v::cubic_foot, d::slug_per_cubic_foot>();
            test::<m::ton_long, v::cubic_yard, d::ton_long_per_cubic_yard>();
            test::<m::ton_short, v::cubic_yard, d::ton_short_per_cubic_yard>();

            fn test<M: m::Conversion<V>, U: v::Conversion<V>, D: d::Conversion<V>>() {
                Test::assert_approx_eq(&MassDensity::new::<D>(V::one()),
                    &(Mass::new::<M>(V::one()) / Volume::new::<U>(V::one())));
            }
        }
    }
}
