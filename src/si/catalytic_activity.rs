//! Catalytic activity (base unit katal, mol · s⁻¹).

quantity! {
    /// Catalytic activity (base unit katal, mol · s⁻¹).
    quantity: CatalyticActivity; "catalytic activity";
    /// Dimension of catalytic activity, T⁻¹N (base unit katal, mol · s⁻¹).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        P1,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottakatal: prefix!(yotta); "Ykat", "yottakatal", "yottakatals";
        @zettakatal: prefix!(zetta); "Zkat", "zettakatal", "zettakatals";
        @exakatal: prefix!(exa); "Ekat", "exakatal", "exakatals";
        @petakatal: prefix!(peta); "Pkat", "petakatal", "petakatals";
        @terakatal: prefix!(tera); "Tkat", "terakatal", "terakatals";
        @gigakatal: prefix!(giga); "Gkat", "gigakatal", "gigakatals";
        @megakatal: prefix!(mega); "Mkat", "megakatal", "megakatals";
        @kilokatal: prefix!(kilo); "kkat", "kilokatal", "kilokatals";
        @hectokatal: prefix!(hecto); "hkat", "hectokatal", "hectokatals";
        @decakatal: prefix!(deca); "dakat", "decakatal", "decakatals";
        @katal: prefix!(none); "kat", "katal", "katals";
        @decikatal: prefix!(deci); "dkat", "decikatal", "decikatals";
        @centikatal: prefix!(centi); "ckat", "centikatal", "centikatals";
        @millikatal: prefix!(milli); "mkat", "millikatal", "millikatals";
        @microkatal: prefix!(micro); "µkat", "microkatal", "microkatals";
        @nanokatal: prefix!(nano); "nkat", "nanokatal", "nanokatals";
        @picokatal: prefix!(pico); "pkat", "picokatal", "picokatals";
        @femtokatal: prefix!(femto); "fkat", "femtokatal", "femtokatals";
        @attokatal: prefix!(atto); "akat", "attokatal", "attokatals";
        @zeptokatal: prefix!(zepto); "zkat", "zeptokatal", "zeptokatals";
        @yoctokatal: prefix!(yocto); "ykat", "yoctokatal", "yoctokatals";

        @yotta_enzyme_unit: prefix!(yotta) * prefix!(micro) / 6.0_E1;
            "YU", "yotta enzyme unit", "yotta enzyme units";
        @zetta_enzyme_unit: prefix!(zetta) * prefix!(micro) / 6.0_E1;
            "ZU", "zetta enzyme unit", "zetta enzyme units";
        @exa_enzyme_unit: prefix!(exa) * prefix!(micro) / 6.0_E1;
            "EU", "exa enzyme unit", "exa enzyme units";
        @peta_enzyme_unit: prefix!(peta) * prefix!(micro) / 6.0_E1;
            "PU", "peta enzyme unit", "peta enzyme units";
        @tera_enzyme_unit: prefix!(tera) * prefix!(micro) / 6.0_E1;
            "TU", "tera enzyme unit", "tera enzyme units";
        @giga_enzyme_unit: prefix!(giga) * prefix!(micro) / 6.0_E1;
            "GU", "giga enzyme unit", "giga enzyme units";
        @mega_enzyme_unit: prefix!(mega) * prefix!(micro) / 6.0_E1;
            "MU", "mega enzyme unit", "mega enzyme units";
        @kilo_enzyme_unit: prefix!(kilo) * prefix!(micro) / 6.0_E1;
            "kU", "kilo enzyme unit", "kilo enzyme units";
        @hecto_enzyme_unit: prefix!(hecto) * prefix!(micro) / 6.0_E1;
            "hU", "hecto enzyme unit", "hecto enzyme units";
        @deca_enzyme_unit: prefix!(deca) * prefix!(micro) / 6.0_E1;
            "daU", "deca enzyme unit", "deca enzyme units";
        @enzyme_unit: prefix!(none) * prefix!(micro) / 6.0_E1;
            "U", "enzyme unit", "enzyme units";
        @deci_enzyme_unit: prefix!(deci) * prefix!(micro) / 6.0_E1;
            "dU", "deci enzyme unit", "deci enzyme units";
        @centi_enzyme_unit: prefix!(centi) * prefix!(micro) / 6.0_E1;
            "cU", "centi enzyme unit", "centi enzyme units";
        @milli_enzyme_unit: prefix!(milli) * prefix!(micro) / 6.0_E1;
            "mU", "milli enzyme unit", "milli enzyme units";
        @micro_enzyme_unit: prefix!(micro) * prefix!(micro) / 6.0_E1;
            "μU", "micro enzyme unit", "micro enzyme units";
        @nano_enzyme_unit: prefix!(nano) * prefix!(micro) / 6.0_E1;
            "nU", "nano enzyme unit", "nano enzyme units";
        @pico_enzyme_unit: prefix!(pico) * prefix!(micro) / 6.0_E1;
            "pU", "pico enzyme unit", "pico enzyme units";
        @femto_enzyme_unit: prefix!(femto) * prefix!(micro) / 6.0_E1;
            "fU", "femto enzyme unit", "femto enzyme units";
        @atto_enzyme_unit: prefix!(atto) * prefix!(micro) / 6.0_E1;
            "aU", "atto enzyme unit", "atto enzyme units";
        @zepto_enzyme_unit: prefix!(zepto) * prefix!(micro) / 6.0_E1;
            "zU", "zepto enzyme unit", "zepto enzyme units";
        @yocto_enzyme_unit: prefix!(yocto) * prefix!(micro) / 6.0_E1;
            "yU", "yocto enzyme unit", "yocto enzyme units";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::catalytic_activity as ca;
        use si::amount_of_substance as aos;
        use si::time as t;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Time<V> = AmountOfSubstance::new::<aos::mole>(V::one())
                / CatalyticActivity::new::<ca::katal>(V::one());
            let _: CatalyticActivity<V> = AmountOfSubstance::new::<aos::mole>(V::one())
                / Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<aos::yottamole, t::second, ca::yottakatal>();
            test::<aos::zettamole, t::second, ca::zettakatal>();
            test::<aos::examole, t::second, ca::exakatal>();
            test::<aos::petamole, t::second, ca::petakatal>();
            test::<aos::teramole, t::second, ca::terakatal>();
            test::<aos::gigamole, t::second, ca::gigakatal>();
            test::<aos::megamole, t::second, ca::megakatal>();
            test::<aos::kilomole, t::second, ca::kilokatal>();
            test::<aos::hectomole, t::second, ca::hectokatal>();
            test::<aos::decamole, t::second, ca::decakatal>();
            test::<aos::mole, t::second, ca::katal>();
            test::<aos::decimole, t::second, ca::decikatal>();
            test::<aos::centimole, t::second, ca::centikatal>();
            test::<aos::millimole, t::second, ca::millikatal>();
            test::<aos::micromole, t::second, ca::microkatal>();
            test::<aos::nanomole, t::second, ca::nanokatal>();
            test::<aos::picomole, t::second, ca::picokatal>();
            test::<aos::femtomole, t::second, ca::femtokatal>();
            test::<aos::attomole, t::second, ca::attokatal>();
            test::<aos::zeptomole, t::second, ca::zeptokatal>();
            test::<aos::yoctomole, t::second, ca::yoctokatal>();

            test::<aos::examole, t::minute, ca::yotta_enzyme_unit>();
            test::<aos::petamole, t::minute, ca::zetta_enzyme_unit>();
            test::<aos::teramole, t::minute, ca::exa_enzyme_unit>();
            test::<aos::gigamole, t::minute, ca::peta_enzyme_unit>();
            test::<aos::megamole, t::minute, ca::tera_enzyme_unit>();
            test::<aos::kilomole, t::minute, ca::giga_enzyme_unit>();
            test::<aos::mole, t::minute, ca::mega_enzyme_unit>();
            test::<aos::millimole, t::minute, ca::kilo_enzyme_unit>();
            test::<aos::micromole, t::minute, ca::enzyme_unit>();
            test::<aos::nanomole, t::minute, ca::milli_enzyme_unit>();
            test::<aos::picomole, t::minute, ca::micro_enzyme_unit>();
            test::<aos::femtomole, t::minute, ca::nano_enzyme_unit>();
            test::<aos::attomole, t::minute, ca::pico_enzyme_unit>();
            test::<aos::zeptomole, t::minute, ca::femto_enzyme_unit>();
            test::<aos::yoctomole, t::minute, ca::atto_enzyme_unit>();

            fn test<AOS: aos::Conversion<V>, T: t::Conversion<V>, CA: ca::Conversion<V>>() {
                Test::assert_approx_eq(
                    &(AmountOfSubstance::new::<AOS>(V::one()) / Time::new::<T>(V::one())),
                    &CatalyticActivity::new::<CA>(V::one()));
                Test::assert_approx_eq(
                    &Time::new::<T>(V::one()),
                    &(AmountOfSubstance::new::<AOS>(V::one())
                        / CatalyticActivity::new::<CA>(V::one())));
            }
        }
    }
}
