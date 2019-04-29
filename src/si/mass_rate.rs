//! Mass rate (base unit kilogram per second, kg · s⁻¹).

quantity! {
    /// Mass rate (base unit kilogram per second, kg · s⁻¹).
    quantity: MassRate; "mass rate";
    /// Dimension of mass rate, MT⁻¹ (base unit kilogram per second, kg · s⁻¹).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottagram_per_second: prefix!(yotta) / prefix!(kilo); "Yg/s", "yottagram per second",
            "yottagrams per second";
        @zettagram_per_second: prefix!(zetta) / prefix!(kilo); "Zg/s", "zettagram per second",
            "zettagrams per second";
        @exagram_per_second: prefix!(exa) / prefix!(kilo); "Eg/s", "exagram per second",
            "exagrams per second";
        @petagram_per_second: prefix!(peta) / prefix!(kilo); "Pg/s", "petagram per second",
            "petagrams per second";
        @teragram_per_second: prefix!(tera) / prefix!(kilo); "Tg/s", "teragram per second",
            "teragrams per second";
        @gigagram_per_second: prefix!(giga) / prefix!(kilo); "Gg/s", "gigagram per second",
            "gigagrams per second";
        @megagram_per_second: prefix!(mega) / prefix!(kilo); "Mg/s", "megagram per second",
            "megagrams per second";
        /// Derived unit of mass rate.
        @kilogram_per_second: prefix!(kilo) / prefix!(kilo); "kg/s", "kilogram per second",
            "kilograms per second";
        @hectogram_per_second: prefix!(hecto) / prefix!(kilo); "hg/s", "hectogram per second",
            "hectograms per second";
        @decagram_per_second: prefix!(deca) / prefix!(kilo); "dag/s", "decagram per second",
            "decagrams per second";
        @gram_per_second: prefix!(none) / prefix!(kilo); "g/s", "gram per second",
            "grams per second";
        @decigram_per_second: prefix!(deci) / prefix!(kilo); "dg/s", "decigram per second",
            "decigrams per second";
        @centigram_per_second: prefix!(centi) / prefix!(kilo); "cg/s", "centigram per second",
            "centigrams per second";
        @milligram_per_second: prefix!(milli) / prefix!(kilo); "mg/s", "milligram per second",
            "milligrams per second";
        @microgram_per_second: prefix!(micro) / prefix!(kilo); "µg/s", "microgram per second",
            "micrograms per second";
        @nanogram_per_second: prefix!(nano) / prefix!(kilo); "ng/s", "nanogram per second",
            "nanograms per second";
        @picogram_per_second: prefix!(pico) / prefix!(kilo); "pg/s", "picogram per second",
            "picograms per second";
        @femtogram_per_second: prefix!(femto) / prefix!(kilo); "fg/s", "femtogram per second",
            "femtograms per second";
        @attogram_per_second: prefix!(atto) / prefix!(kilo); "ag/s", "attogram per second",
            "attograms per second";
        @zeptogram_per_second: prefix!(zepto) / prefix!(kilo); "zg/s", "zeptogram per second",
            "zeptograms per second";
        @yoctogram_per_second: prefix!(yocto) / prefix!(kilo); "yg/s", "yoctogram per second",
            "yoctograms per second";

        @kilogram_per_minute: 1.666_666_666_666_666_6_E-2; "kg/min", "kilogram per minute",
            "kilograms per minute";
        @kilogram_per_hour: 2.777_777_777_777_777_7_E-4; "kg/h", "kilogram per hour",
            "kilograms per hour";
        @kilogram_per_day: 1.157_407_407_407_407_4_E-5; "kg/d", "kilogram per day",
            "kilograms per day";
        @gram_per_minute: 1.666_666_666_666_666_6_E-5; "g/min", "gram per minute",
            "grams per minute";
        @gram_per_hour: 2.777_777_777_777_777_7_E-7; "g/h", "gram per hour", "grams per hour";
        @gram_per_day: 1.157_407_407_407_407_4_E-8; "g/d", "gram per day", "grams per day";

        @carat_per_second: 2.0_E-4; "ct/s", "carat per second", "carats per second";
        @grain_per_second: 6.479_891_E-5; "gr/s", "grain per second", "grains per second";
        @hundredweight_long_per_second: 5.080_235_E1; "cwt long/s",
            "hundredweight (long) per second", "hundredweight (long) per second";
        @hundredweight_short_per_second: 4.535_924_E1; "cwt short/s",
            "hundredweight (short) per second", "hundredweight (short) per second";
        @ounce_per_second: 2.834_952_E-2; "oz/s", "ounce per second", "ounces per second";
        @ounce_troy_per_second: 3.110_348_E-2; "oz t/s", "troy ounce per second",
            "troy ounces per second";
        @pennyweight_per_second: 1.555_174_E-3; "dwt/s", "pennyweight per second",
            "pennyweight per second";
        @pound_per_second: 4.535_924_E-1; "lb/s", "pound per second", "pounds per second";
        @pound_per_minute: 7.559_873_333_333_333_E-3; "lb/min", "pound per minute",
            "pounds per minute";
        @pound_per_hour: 1.259_978_888_888_888_8_E-4; "lb/h", "pound per hour", "pounds per hour";
        @pound_per_day:  5.249_912_037_037_037_0_E-6; "lb/d", "pound per day", "pounds per day";
        @pound_troy_per_second: 3.732_417_E-1; "lb t/s", "troy pound per second",
            "troy pounds per second";
        @slug_per_second: 1.459_390_E1; "slug/s", "slug per second", "slugs per second";
        @ton_assay_per_second: 2.916_667_E-2; "AT/s", "assay ton per second",
            "assay tons per second";
        @ton_long_per_second: 1.016_047_E3; "2240 lb/s", "long ton per second",
            "long tons per second";
        @ton_short_per_second: 9.071_847_E2; "2000 lb/s", "short ton per second",
            "short tons per second";
        @ton_short_per_hour: 2.519_957_5_E-1; "2000 lb/h", "short ton per hour",
            "short tons per hour";
        @ton_per_second: 1.0_E3; "t/s", "ton per second",
            "tons per second"; // ton per second, metric
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::mass as m;
        use si::time as t;
        use si::mass_rate as r;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: MassRate<V> = Mass::new::<m::kilogram>(V::one())
                / Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<m::yottagram, t::second, r::yottagram_per_second>();
            test::<m::zettagram, t::second, r::zettagram_per_second>();
            test::<m::exagram, t::second, r::exagram_per_second>();
            test::<m::petagram, t::second, r::petagram_per_second>();
            test::<m::teragram, t::second, r::teragram_per_second>();
            test::<m::gigagram, t::second, r::gigagram_per_second>();
            test::<m::megagram, t::second, r::megagram_per_second>();
            test::<m::kilogram, t::second, r::kilogram_per_second>();
            test::<m::hectogram, t::second, r::hectogram_per_second>();
            test::<m::decagram, t::second, r::decagram_per_second>();
            test::<m::gram, t::second, r::gram_per_second>();
            test::<m::decigram, t::second, r::decigram_per_second>();
            test::<m::centigram, t::second, r::centigram_per_second>();
            test::<m::milligram, t::second, r::milligram_per_second>();
            test::<m::microgram, t::second, r::microgram_per_second>();
            test::<m::nanogram, t::second, r::nanogram_per_second>();
            test::<m::picogram, t::second, r::picogram_per_second>();
            test::<m::femtogram, t::second, r::femtogram_per_second>();
            test::<m::attogram, t::second, r::attogram_per_second>();
            test::<m::zeptogram, t::second, r::zeptogram_per_second>();
            test::<m::yoctogram, t::second, r::yoctogram_per_second>();

            test::<m::kilogram, t::minute, r::kilogram_per_minute>();
            test::<m::kilogram, t::hour, r::kilogram_per_hour>();
            test::<m::kilogram, t::day, r::kilogram_per_day>();
            test::<m::gram, t::minute, r::gram_per_minute>();
            test::<m::gram, t::hour, r::gram_per_hour>();
            test::<m::gram, t::day, r::gram_per_day>();

            test::<m::carat, t::second, r::carat_per_second>();
            test::<m::grain, t::second, r::grain_per_second>();
            test::<m::hundredweight_long, t::second, r::hundredweight_long_per_second>();
            test::<m::hundredweight_short, t::second, r::hundredweight_short_per_second>();
            test::<m::ounce, t::second, r::ounce_per_second>();
            test::<m::ounce_troy, t::second, r::ounce_troy_per_second>();
            test::<m::pennyweight, t::second, r::pennyweight_per_second>();
            test::<m::pound, t::second, r::pound_per_second>();
            test::<m::pound, t::minute, r::pound_per_minute>();
            test::<m::pound, t::hour, r::pound_per_hour>();
            test::<m::pound, t::day, r::pound_per_day>();
            test::<m::pound_troy, t::second, r::pound_troy_per_second>();
            test::<m::slug, t::second, r::slug_per_second>();
            test::<m::ton_assay, t::second, r::ton_assay_per_second>();
            test::<m::ton_long, t::second, r::ton_long_per_second>();
            test::<m::ton_short, t::second, r::ton_short_per_second>();
            test::<m::ton_short, t::hour, r::ton_short_per_hour>();
            test::<m::ton, t::second, r::ton_per_second>();

            fn test<M: m::Conversion<V>, T: t::Conversion<V>, R: r::Conversion<V>>() {
                Test::assert_approx_eq(&MassRate::new::<R>(V::one()),
                    &(Mass::new::<M>(V::one()) / Time::new::<T>(V::one())));
            }
        }
    }
}
