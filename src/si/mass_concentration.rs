//! Mass concentration (base unit kilogram per cubic meter, kg · m⁻³).

quantity! {
    /// Mass concentration (base unit kilogram per cubic meter, kg · m⁻³).
    quantity: MassConcentration; "mass concentration";
    /// Dimension of mass concentration, L⁻³M (base unit kilogram per cubic meter, kg · m⁻³).
    dimension: ISQ<
        N3,     // length
        P1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn(::si::marker::ConstituentConcentrationKind);
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

        @kilogram_per_cubic_decimeter:
            (prefix!(kilo) / prefix!(kilo)) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "kg/dm³", "kilogram per cubic decimeter", "kilograms per cubic decimeter";
        @kilogram_per_liter:
            (prefix!(kilo) / prefix!(kilo)) / prefix!(milli);
            "kg/L", "kilogram per liter", "kilograms per liter";
        @gram_per_cubic_decimeter:
            (prefix!(none) / prefix!(kilo)) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "g/dm³", "gram per cubic decimeter", "grams per cubic decimeter";
        @gram_per_liter:
            (prefix!(none) / prefix!(kilo)) / prefix!(milli);
            "g/L", "gram per liter", "grams per liter";
        @milligram_per_cubic_decimeter:
            (prefix!(milli) / prefix!(kilo)) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "mg/dm³", "milligram per cubic decimeter", "milligrams per cubic decimeter";
        @milligram_per_liter:
            (prefix!(milli) / prefix!(kilo)) / prefix!(milli);
            "mg/L", "milligram per liter", "milligrams per liter";
        @microgram_per_cubic_decimeter:
            (prefix!(micro) / prefix!(kilo)) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "μg/dm³", "microgram per cubic decimeter", "micrograms per cubic decimeter";
        @microgram_per_liter:
            (prefix!(micro) / prefix!(kilo)) / prefix!(milli);
            "μg/L", "microgram per liter", "micrograms per liter";
        @nanogram_per_cubic_decimeter:
            (prefix!(nano) / prefix!(kilo)) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "ng/dm³", "nanogram per cubic decimeter", "nanograms per cubic decimeter";
        @nanogram_per_liter:
            (prefix!(nano) / prefix!(kilo)) / prefix!(milli);
            "ng/L", "nanogram per liter", "nanograms per liter";
        @picogram_per_cubic_decimeter:
            (prefix!(pico) / prefix!(kilo)) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "pg/dm³", "picogram per cubic decimeter", "picograms per cubic decimeter";
        @picogram_per_liter:
            (prefix!(pico) / prefix!(kilo)) / prefix!(milli);
            "pg/L", "picogram per liter", "picograms per liter";
        @femtogram_per_cubic_decimeter:
            (prefix!(femto) / prefix!(kilo)) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "fg/dm³", "femtogram per cubic decimeter", "femtograms per cubic decimeter";
        @femtogram_per_liter:
            (prefix!(femto) / prefix!(kilo)) / prefix!(milli);
            "fg/L", "femtogram per liter", "femtograms per liter";

        @kilogram_per_deciliter:
            (prefix!(kilo) / prefix!(kilo)) / prefix!(deci) / prefix!(milli);
            "kg/dL", "kilogram per deciliter", "kilograms per deciliter";
        @gram_per_deciliter:
            (prefix!(none) / prefix!(kilo)) / prefix!(deci) / prefix!(milli);
            "g/dL", "gram per deciliter", "grams per deciliter";
        @milligram_per_deciliter:
            (prefix!(milli) / prefix!(kilo)) / prefix!(deci) / prefix!(milli);
            "mg/dL", "milligram per deciliter", "milligrams per deciliter";
        @microgram_per_deciliter:
            (prefix!(micro) / prefix!(kilo)) / prefix!(deci) / prefix!(milli);
            "μg/dL", "microgram per deciliter", "micrograms per deciliter";
        @nanogram_per_deciliter:
            (prefix!(nano) / prefix!(kilo)) / prefix!(deci) / prefix!(milli);
            "ng/dL", "nanogram per deciliter", "nanograms per deciliter";
        @picogram_per_deciliter:
            (prefix!(pico) / prefix!(kilo)) / prefix!(deci) / prefix!(milli);
            "pg/dL", "picogram per deciliter", "picograms per deciliter";
        @femtogram_per_deciliter:
            (prefix!(femto) / prefix!(kilo)) / prefix!(deci) / prefix!(milli);
            "fg/dL", "femtogram per deciliter", "femtograms per deciliter";

        @kilogram_per_milliliter:
            (prefix!(kilo) / prefix!(kilo)) / prefix!(milli) / prefix!(milli);
            "kg/mL", "kilogram per milliliter", "kilograms per milliliter";
        @gram_per_milliliter:
            (prefix!(none) / prefix!(kilo)) / prefix!(milli) / prefix!(milli);
            "g/mL", "gram per milliliter", "grams per milliliter";
        @milligram_per_milliliter:
            (prefix!(milli) / prefix!(kilo)) / prefix!(milli) / prefix!(milli);
            "mg/mL", "milligram per milliliter", "milligrams per milliliter";
        @microgram_per_milliliter:
            (prefix!(micro) / prefix!(kilo)) / prefix!(milli) / prefix!(milli);
            "μg/mL", "microgram per milliliter", "micrograms per milliliter";
        @nanogram_per_milliliter:
            (prefix!(nano) / prefix!(kilo)) / prefix!(milli) / prefix!(milli);
            "ng/mL", "nanogram per milliliter", "nanograms per milliliter";
        @picogram_per_milliliter:
            (prefix!(pico) / prefix!(kilo)) / prefix!(milli) / prefix!(milli);
            "pg/mL", "picogram per milliliter", "picograms per milliliter";
        @femtogram_per_milliliter:
            (prefix!(femto) / prefix!(kilo)) / prefix!(milli) / prefix!(milli);
            "fg/mL", "femtogram per milliliter", "femtograms per milliliter";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::mass as m;
        use si::volume as v;
        use si::mass_concentration as r;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: MassConcentration<V> = (Mass::new::<m::kilogram>(V::one())
                / Volume::new::<v::cubic_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<m::yottagram, v::cubic_meter, r::yottagram_per_cubic_meter>();
            test::<m::zettagram, v::cubic_meter, r::zettagram_per_cubic_meter>();
            test::<m::exagram, v::cubic_meter, r::exagram_per_cubic_meter>();
            test::<m::petagram, v::cubic_meter, r::petagram_per_cubic_meter>();
            test::<m::teragram, v::cubic_meter, r::teragram_per_cubic_meter>();
            test::<m::gigagram, v::cubic_meter, r::gigagram_per_cubic_meter>();
            test::<m::megagram, v::cubic_meter, r::megagram_per_cubic_meter>();
            test::<m::kilogram, v::cubic_meter, r::kilogram_per_cubic_meter>();
            test::<m::hectogram, v::cubic_meter, r::hectogram_per_cubic_meter>();
            test::<m::decagram, v::cubic_meter, r::decagram_per_cubic_meter>();
            test::<m::gram, v::cubic_meter, r::gram_per_cubic_meter>();
            test::<m::decigram, v::cubic_meter, r::decigram_per_cubic_meter>();
            test::<m::centigram, v::cubic_meter, r::centigram_per_cubic_meter>();
            test::<m::milligram, v::cubic_meter, r::milligram_per_cubic_meter>();
            test::<m::microgram, v::cubic_meter, r::microgram_per_cubic_meter>();
            test::<m::nanogram, v::cubic_meter, r::nanogram_per_cubic_meter>();
            test::<m::picogram, v::cubic_meter, r::picogram_per_cubic_meter>();
            test::<m::femtogram, v::cubic_meter, r::femtogram_per_cubic_meter>();
            test::<m::attogram, v::cubic_meter, r::attogram_per_cubic_meter>();
            test::<m::zeptogram, v::cubic_meter, r::zeptogram_per_cubic_meter>();
            test::<m::yoctogram, v::cubic_meter, r::yoctogram_per_cubic_meter>();

            test::<m::kilogram, v::cubic_decimeter, r::kilogram_per_cubic_decimeter>();
            test::<m::kilogram, v::liter, r::kilogram_per_liter>();
            test::<m::gram, v::cubic_decimeter, r::gram_per_cubic_decimeter>();
            test::<m::gram, v::liter, r::gram_per_liter>();
            test::<m::milligram, v::cubic_decimeter, r::milligram_per_cubic_decimeter>();
            test::<m::milligram, v::liter, r::milligram_per_liter>();
            test::<m::microgram, v::cubic_decimeter, r::microgram_per_cubic_decimeter>();
            test::<m::microgram, v::liter, r::microgram_per_liter>();
            test::<m::nanogram, v::cubic_decimeter, r::nanogram_per_cubic_decimeter>();
            test::<m::nanogram, v::liter, r::nanogram_per_liter>();
            test::<m::picogram, v::cubic_decimeter, r::picogram_per_cubic_decimeter>();
            test::<m::picogram, v::liter, r::picogram_per_liter>();
            test::<m::femtogram, v::cubic_decimeter, r::femtogram_per_cubic_decimeter>();
            test::<m::femtogram, v::liter, r::femtogram_per_liter>();

            test::<m::kilogram, v::deciliter, r::kilogram_per_deciliter>();
            test::<m::gram, v::deciliter, r::gram_per_deciliter>();
            test::<m::milligram, v::deciliter, r::milligram_per_deciliter>();
            test::<m::microgram, v::deciliter, r::microgram_per_deciliter>();
            test::<m::nanogram, v::deciliter, r::nanogram_per_deciliter>();
            test::<m::picogram, v::deciliter, r::picogram_per_deciliter>();
            test::<m::femtogram, v::deciliter, r::femtogram_per_deciliter>();

            test::<m::kilogram, v::milliliter, r::kilogram_per_milliliter>();
            test::<m::gram, v::milliliter, r::gram_per_milliliter>();
            test::<m::milligram, v::milliliter, r::milligram_per_milliliter>();
            test::<m::microgram, v::milliliter, r::microgram_per_milliliter>();
            test::<m::nanogram, v::milliliter, r::nanogram_per_milliliter>();
            test::<m::picogram, v::milliliter, r::picogram_per_milliliter>();
            test::<m::femtogram, v::milliliter, r::femtogram_per_milliliter>();

            fn test<M: m::Conversion<V>, U: v::Conversion<V>, R: r::Conversion<V>>() {
                Test::assert_approx_eq(&MassConcentration::new::<R>(V::one()),
                    &(Mass::new::<M>(V::one()) / Volume::new::<U>(V::one())).into());
            }
        }
    }
}
