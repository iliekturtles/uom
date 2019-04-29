//! Energy (base unit joule, kg · m² · s⁻²).

quantity! {
    /// Energy (base unit joule, kg · m² · s⁻²).
    quantity: Energy; "energy";
    /// Dimension of energy, L²MT⁻² (base unit joule, kg · m² · s⁻²).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottajoule: prefix!(yotta); "YJ", "yottajoule", "yottajoules";
        @zettajoule: prefix!(zetta); "ZJ", "zettajoule", "zettajoules";
        @exajoule: prefix!(exa); "EJ", "exajoule", "exajoules";
        @petajoule: prefix!(peta); "PJ", "petajoule", "petajoules";
        @terajoule: prefix!(tera); "TJ", "terajoule", "terajoules";
        @gigajoule: prefix!(giga); "GJ", "gigajoule", "gigajoules";
        @megajoule: prefix!(mega); "MJ", "megajoule", "megajoules";
        @kilojoule: prefix!(kilo); "kJ", "kilojoule", "kilojoules";
        @hectojoule: prefix!(hecto); "hJ", "hectojoule", "hectojoules";
        @decajoule: prefix!(deca); "daJ", "decajoule", "decajoules";
        /// Derived unit of energy.
        @joule: prefix!(none); "J", "joule", "joules";
        @decijoule: prefix!(deci); "dJ", "decijoule", "decijoules";
        @centijoule: prefix!(centi); "cJ", "centijoule", "centijoules";
        @millijoule: prefix!(milli); "mJ", "milljoule", "milljoules";
        @microjoule: prefix!(micro); "µJ", "microjoule", "microjoules";
        @nanojoule: prefix!(nano); "nJ", "nanojoule", "nanojoules";
        @picojoule: prefix!(pico); "pJ", "picojoule", "picojoules";
        @femtojoule: prefix!(femto); "fJ", "femtojoule", "femtojoules";
        @attojoule: prefix!(atto); "aJ", "attojoule", "attojoules";
        @zeptojoule: prefix!(zepto); "zJ", "zeptojoule", "zeptojoules";
        @yoctojoule: prefix!(yocto); "yJ", "yoctojoule", "yoctojoules";

        @petawatt_hour: 3.6_E18; "PW · h", "petawatt hour", "petawatt hours";
        @terawatt_hour: 3.6_E15; "TW · h", "terawatt hour", "terawatt hours";
        @gigawatt_hour: 3.6_E12; "GW · h", "gigawatt hour", "gigawatt hours";
        @megawatt_hour: 3.6_E9; "MW · h", "megawatt hour", "megawatt hours";
        @kilowatt_hour: 3.6_E6; "kW · h", "kilowatt hour", "kilowatt hours";
        @hectowatt_hour: 3.6_E5; "hW · h", "hectowatt hour", "hectowatt hours";
        @decawatt_hour: 3.6_E4; "daW · h", "decawatt hour", "decawatt hours";
        @watt_hour: 3.6_E3; "W · h", "watt hour", "watt hours";
        @milliwatt_hour: 3.6_E0; "mW · h", "milliwatt hour", "milliwatt hours";
        @microwatt_hour: 3.6_E-3; "µW · h", "microwatt hour", "microwatt hours";

        @btu_it: 1.055_056_E3; "Btu (IT)", "British thermal unit (IT)",
            "British thermal units (IT)";
        @btu: 1.054_350_E3; "Btu", "British thermal unit", "British thermal units";
        @btu_39: 1.059_67_E3; "Btu₃₉", "British thermal unit (39 °F)",
            "British thermal units (39 °F)";
        @btu_59: 1.054_80_E3; "Btu₅₉", "British thermal unit (59 °F)",
            "British thermal units (59 °F)";
        @btu_60: 1.054_68_E3; "Btu₆₀", "British thermal unit (60 °F)",
            "British thermal units (60 °F)";
        @calorie_it: 4.186_8_E0; "cal (IT)", "calorie (IT)", "calories (IT)";
        @calorie: 4.184_E0; "cal", "calorie", "calories";
        @calorie_15: 4.185_80_E0; "cal₁₅", "calorie (15 °C)", "calories (15 °C)";
        @calorie_20: 4.181_90_E0; "cal₂₀", "calorie (20 °C)", "calories (20 °C)";
        @calorie_it_nutrition: 4.186_8_E3; "Cal (IT)", "Calorie (IT)", "Calories (IT)";
        @calorie_nutrition: 4.184_E3; "Cal", "Calorie", "Calories";
        @electronvolt: 1.602_177_E-19; "eV", "electronvolt", "electronvolts";
        @erg: 1.0_E-7; "erg", "erg", "ergs";
        @foot_poundal: 4.214_011_E-2; "ft · pdl", "foot poundal", "foot poundals";
        @foot_pound: 1.355_818_E0; "ft · lbf", "foot pound-force",
            "foot pounds-force"; // @foot_pound_force
        @kilocalorie_it: 4.186_8_E3; "kcal (IT)", "kilocalorie (IT)", "kilocalories (IT)";
        @kilocalorie: 4.184_E3; "kcal", "kilocalorie", "kilocalories";
        @quad: 1.055_056_E18; "10¹⁵ Btu (IT)", "quad", "quads";
        @therm_ec: 1.055_06_E8; "thm (EC)", "therm (EC)", "therms (EC)";
        @therm_us: 1.054_804_E8; "thm", "therm", "therms";
        @ton_tnt: 4.184_E9; "t of TNT", "ton of TNT", "tons of TNT";
        @watt_second: 1.0_E0; "W · s", "watt second", "watt seconds";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::area as a;
        use si::mass as m;
        use si::time as t;
        use si::energy as e;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Energy<V> = Area::new::<a::square_meter>(V::one())
                * Mass::new::<m::kilogram>(V::one())
                / (Time::new::<t::second>(V::one()) * Time::new::<t::second>(V::one()));
        }

        #[test]
        fn check_units() {
            test::<m::yottagram, e::zettajoule>();
            test::<m::zettagram, e::exajoule>();
            test::<m::exagram, e::petajoule>();
            test::<m::petagram, e::terajoule>();
            test::<m::teragram, e::gigajoule>();
            test::<m::gigagram, e::megajoule>();
            test::<m::megagram, e::kilojoule>();
            test::<m::kilogram, e::joule>();
            test::<m::decagram, e::centijoule>();
            test::<m::gram, e::millijoule>();
            test::<m::milligram, e::microjoule>();
            test::<m::microgram, e::nanojoule>();
            test::<m::nanogram, e::picojoule>();
            test::<m::picogram, e::femtojoule>();
            test::<m::femtogram, e::attojoule>();
            test::<m::attogram, e::zeptojoule>();
            test::<m::zeptogram, e::yoctojoule>();

            fn test<M: m::Conversion<V>, E: e::Conversion<V>>() {
                Test::assert_approx_eq(&Energy::new::<E>(V::one()),
                    &(Area::new::<a::square_meter>(V::one())
                        * Mass::new::<M>(V::one())
                        / (Time::new::<t::second>(V::one()) * Time::new::<t::second>(V::one()))));
            }
        }
    }
}
