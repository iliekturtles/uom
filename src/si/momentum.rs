//! Momentum (base unit kilogram meter per second, kg · m · s⁻¹).

quantity! {
    /// Momentum (base unit kilogram meter per second, kg · m · s⁻¹).
    quantity: Momentum; "momentum";
    /// Dimension of momentum, LMT⁻¹ (base unit kilogram meter per second, kg · m · s⁻¹).
    dimension: ISQ<
        P1,     // length
        P1,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottagram_meter_per_second: prefix!(yotta) / prefix!(kilo); "Yg · m/s",
            "yottagram meter per second", "yottagram meters per second";
        @zettagram_meter_per_second: prefix!(zetta) / prefix!(kilo); "Zg · m/s",
            "zettagram meter per second", "zettagram meters per second";
        @exagram_meter_per_second: prefix!(exa) / prefix!(kilo); "Eg · m/s",
            "exagram meter per second", "exagram meters per second";
        @petagram_meter_per_second: prefix!(peta) / prefix!(kilo); "Pg · m/s",
            "petagram meter per second", "petagram meters per second";
        @teragram_meter_per_second: prefix!(tera) / prefix!(kilo); "Tg · m/s",
            "teragram meter per second", "teragram meters per second";
        @gigagram_meter_per_second: prefix!(giga) / prefix!(kilo); "Gg · m/s",
            "gigagram meter per second", "gigagram meters per second";
        @megagram_meter_per_second: prefix!(mega) / prefix!(kilo); "Mg · m/s",
            "megagram meter per second", "megagram meters per second";
        /// Derived unit of momentum.
        @kilogram_meter_per_second: prefix!(kilo) / prefix!(kilo); "kg · m/s",
            "kilogram meter per second", "kilogram meters per second";
        @hectogram_meter_per_second: prefix!(hecto) / prefix!(kilo); "hg · m/s",
            "hectogram meter per second", "hectogram meters per second";
        @decagram_meter_per_second: prefix!(deca) / prefix!(kilo); "dag · m/s",
            "decagram meter per second", "decagram meters per second";
        @gram_meter_per_second: prefix!(none) / prefix!(kilo); "g · m/s", "gram meter per second",
            "gram meters per second";
        @decigram_meter_per_second: prefix!(deci) / prefix!(kilo); "dg · m/s",
            "decigram meter per second", "decigram meters per second";
        @centigram_meter_per_second: prefix!(centi) / prefix!(kilo); "cg · m/s",
            "centigram meter per second", "centigram meters per second";
        @milligram_meter_per_second: prefix!(milli) / prefix!(kilo); "mg · m/s",
            "milligram meter per second", "milligram meters per second";
        @microgram_meter_per_second: prefix!(micro) / prefix!(kilo); "µg · m/s",
            "microgram meter per second", "microgram meters per second";
        @nanogram_meter_per_second: prefix!(nano) / prefix!(kilo); "ng · m/s",
            "nanogram meter per second", "nanogram meters per second";
        @picogram_meter_per_second: prefix!(pico) / prefix!(kilo); "pg · m/s",
            "picogram meter per second", "picogram meters per second";
        @femtogram_meter_per_second: prefix!(femto) / prefix!(kilo); "fg · m/s",
            "femtogram meter per second", "femtogram meters per second";
        @attogram_meter_per_second: prefix!(atto) / prefix!(kilo); "ag · m/s",
            "attogram meter per second", "attogram meters per second";
        @zeptogram_meter_per_second: prefix!(zepto) / prefix!(kilo); "zg · m/s",
            "zeptogram meter per second", "zeptogram meters per second";
        @yoctogram_meter_per_second: prefix!(yocto) / prefix!(kilo); "yg · m/s",
            "yoctogram meter per second", "yoctogram meters per second";

        @kilogram_yottameter_per_second: prefix!(yotta); "kg · Ym/s",
            "kilogram yottameter per second", "kilogram yottameters per second";
        @kilogram_zettameter_per_second: prefix!(zetta); "kg · Zm/s",
            "kilogram zettameter per second", "kilogram zettameters per second";
        @kilogram_exameter_per_second: prefix!(exa); "kg · Em/s", "kilogram exameter per second",
            "kilogram exameters per second";
        @kilogram_petameter_per_second: prefix!(peta); "kg · Pm/s", "kilogram petameter per second",
            "kilogram petameters per second";
        @kilogram_terameter_per_second: prefix!(tera); "kg · Tm/s", "kilogram terameter per second",
            "kilogram terameters per second";
        @kilogram_gigameter_per_second: prefix!(giga); "kg · Gm/s", "kilogram gigameter per second",
            "kilogram gigameters per second";
        @kilogram_megameter_per_second: prefix!(mega); "kg · Mm/s", "kilogram megameter per second",
            "kilogram megameters per second";
        @kilogram_kilometer_per_second: prefix!(kilo); "kg · km/s", "kilogram kilometer per second",
            "kilogram kilometers per second";
        @kilogram_hectometer_per_second: prefix!(hecto); "kg · hm/s",
            "kilogram hectometer per second", "kilogram hectometers per second";
        @kilogram_decameter_per_second: prefix!(deca); "kg · dam/s",
            "kilogram decameter per second", "kilogram decameters per second";
        @kilogram_decimeter_per_second: prefix!(deci); "kg · dm/s", "kilogram decimeter per second",
            "kilogram decimeters per second";
        @kilogram_centimeter_per_second: prefix!(centi); "kg · cm/s",
            "kilogram centimeter per second", "kilogram centimeters per second";
        @kilogram_millimeter_per_second: prefix!(milli); "kg · mm/s",
            "kilogram millimeter per second", "kilogram millimeters per second";
        @kilogram_micrometer_per_second: prefix!(micro); "kg · µm/s",
            "kilogram micrometer per second", "kilogram micrometers per second";
        @kilogram_nanometer_per_second: prefix!(nano); "kg · nm/s", "kilogram nanometer per second",
            "kilogram nanometers per second";
        @kilogram_picometer_per_second: prefix!(pico); "kg · pm/s", "kilogram picometer per second",
            "kilogram picometers per second";
        @kilogram_femtometer_per_second: prefix!(femto); "kg · fm/s",
            "kilogram femtometer per second", "kilogram femtometers per second";
        @kilogram_attometer_per_second: prefix!(atto); "kg · am/s", "kilogram attometer per second",
            "kilogram attometers per second";
        @kilogram_zeptometer_per_second: prefix!(zepto); "kg · zm/s",
            "kilogram zeptometer per second", "kilogram zeptometers per second";
        @kilogram_yoctometer_per_second: prefix!(yocto); "kg · ym/s",
            "kilogram yoctometer per second", "kilogram yoctometers per second";

        @ton_meter_per_second: prefix!(mega) / prefix!(kilo); "t · m/s", "ton meter per second",
            "ton meters per second";

        @kilogram_meter_per_minute: 1.666_666_666_666_666_6_E-2; "kg · m/min",
            "kilogram meter per minute", "kilogram meters per minute";
        @kilogram_meter_per_hour: 2.777_777_777_777_777_7_E-4; "kg · m/h",
            "kilogram meter per hour", "kilogram meters per hour";
        @kilogram_meter_per_day: 1.157_407_407_407_407_4_E-5; "kg · m/d", "kilogram meter per day",
            "kilogram meters per day";

        @slug_foot_per_second: 4.448_220_72_E0; "slug · ft/s", "slug foot per second",
            "slug feet per second";
        @slug_inch_per_second: 3.706_850_6_E-1; "slug · in/s", "slug inch per second",
            "slug inches per second";
        @pound_foot_per_second: 1.382_549_635_2_E-1; "lb · ft/s", "pound foot per second",
            "pound feet per second";
        @pound_inch_per_second: 1.152_124_696_E-2; "lb · in/s", "pound inch per second",
            "pound inches per second";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::length as l;
        use si::mass as m;
        use si::time as t;
        use si::momentum as mmm;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Momentum<V> = Mass::new::<m::kilogram>(V::one())
                * Length::new::<l::meter>(V::one())
                / Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<m::yottagram, l::meter, t::second, mmm::yottagram_meter_per_second>();
            test::<m::zettagram, l::meter, t::second, mmm::zettagram_meter_per_second>();
            test::<m::exagram, l::meter, t::second, mmm::exagram_meter_per_second>();
            test::<m::petagram, l::meter, t::second, mmm::petagram_meter_per_second>();
            test::<m::teragram, l::meter, t::second, mmm::teragram_meter_per_second>();
            test::<m::gigagram, l::meter, t::second, mmm::gigagram_meter_per_second>();
            test::<m::megagram, l::meter, t::second, mmm::megagram_meter_per_second>();
            test::<m::kilogram, l::meter, t::second, mmm::kilogram_meter_per_second>();
            test::<m::hectogram, l::meter, t::second, mmm::hectogram_meter_per_second>();
            test::<m::decagram, l::meter, t::second, mmm::decagram_meter_per_second>();
            test::<m::gram, l::meter, t::second, mmm::gram_meter_per_second>();
            test::<m::decigram, l::meter, t::second, mmm::decigram_meter_per_second>();
            test::<m::centigram, l::meter, t::second, mmm::centigram_meter_per_second>();
            test::<m::milligram, l::meter, t::second, mmm::milligram_meter_per_second>();
            test::<m::microgram, l::meter, t::second, mmm::microgram_meter_per_second>();
            test::<m::nanogram, l::meter, t::second, mmm::nanogram_meter_per_second>();
            test::<m::picogram, l::meter, t::second, mmm::picogram_meter_per_second>();
            test::<m::femtogram, l::meter, t::second, mmm::femtogram_meter_per_second>();
            test::<m::attogram, l::meter, t::second, mmm::attogram_meter_per_second>();
            test::<m::zeptogram, l::meter, t::second, mmm::zeptogram_meter_per_second>();
            test::<m::yoctogram, l::meter, t::second, mmm::yoctogram_meter_per_second>();

            test::<m::kilogram, l::yottameter, t::second, mmm::kilogram_yottameter_per_second>();
            test::<m::kilogram, l::zettameter, t::second, mmm::kilogram_zettameter_per_second>();
            test::<m::kilogram, l::exameter, t::second, mmm::kilogram_exameter_per_second>();
            test::<m::kilogram, l::petameter, t::second, mmm::kilogram_petameter_per_second>();
            test::<m::kilogram, l::terameter, t::second, mmm::kilogram_terameter_per_second>();
            test::<m::kilogram, l::gigameter, t::second, mmm::kilogram_gigameter_per_second>();
            test::<m::kilogram, l::megameter, t::second, mmm::kilogram_megameter_per_second>();
            test::<m::kilogram, l::kilometer, t::second, mmm::kilogram_kilometer_per_second>();
            test::<m::kilogram, l::hectometer, t::second, mmm::kilogram_hectometer_per_second>();
            test::<m::kilogram, l::decameter, t::second, mmm::kilogram_decameter_per_second>();
            test::<m::kilogram, l::decimeter, t::second, mmm::kilogram_decimeter_per_second>();
            test::<m::kilogram, l::centimeter, t::second, mmm::kilogram_centimeter_per_second>();
            test::<m::kilogram, l::millimeter, t::second, mmm::kilogram_millimeter_per_second>();
            test::<m::kilogram, l::micrometer, t::second, mmm::kilogram_micrometer_per_second>();
            test::<m::kilogram, l::nanometer, t::second, mmm::kilogram_nanometer_per_second>();
            test::<m::kilogram, l::picometer, t::second, mmm::kilogram_picometer_per_second>();
            test::<m::kilogram, l::femtometer, t::second, mmm::kilogram_femtometer_per_second>();
            test::<m::kilogram, l::attometer, t::second, mmm::kilogram_attometer_per_second>();
            test::<m::kilogram, l::zeptometer, t::second, mmm::kilogram_zeptometer_per_second>();
            test::<m::kilogram, l::yoctometer, t::second, mmm::kilogram_yoctometer_per_second>();

            test::<m::ton, l::meter, t::second, mmm::ton_meter_per_second>();
            test::<m::kilogram, l::meter, t::minute, mmm::kilogram_meter_per_minute>();
            test::<m::kilogram, l::meter, t::hour, mmm::kilogram_meter_per_hour>();
            test::<m::kilogram, l::meter, t::day, mmm::kilogram_meter_per_day>();

            test::<m::slug, l::foot, t::second, mmm::slug_foot_per_second>();
            test::<m::slug, l::inch, t::second, mmm::slug_inch_per_second>();
            test::<m::pound, l::foot, t::second, mmm::pound_foot_per_second>();
            test::<m::pound, l::inch, t::second, mmm::pound_inch_per_second>();

            fn test<M, L, T, R>()
            where
                M: m::Conversion<V>,
                L: l::Conversion<V>,
                T: t::Conversion<V>,
                R: mmm::Conversion<V>
            {
                Test::assert_approx_eq(&Momentum::new::<R>(V::one()),
                    &(Mass::new::<M>(V::one()) * Length::new::<L>(V::one())
                        / Time::new::<T>(V::one())));
            }
        }
    }
}
