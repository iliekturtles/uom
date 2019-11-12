//! Catalytic activity concentration (base unit katal per cubic meter, mol · s⁻¹ · m⁻³).

quantity! {
    /// Catalytic activity concentration (base unit katal per cubic meter, mol · s⁻¹ · m⁻³).
    quantity: CatalyticActivityConcentration; "catalytic activity concentration";
    /// Dimension of catalytic activity concentration, L⁻³T⁻¹N
    /// (base unit katal per cubic meter, mol · s⁻¹ · m⁻³).
    dimension: ISQ<
        N3,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        P1,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn(::si::marker::ConstituentConcentrationKind);
    units {
        @yottakatal_per_cubic_meter: prefix!(yotta); "Ykat/m³",
            "yottakatal per cubic meter", "yottakatals per cubic meter";
        @zettakatal_per_cubic_meter: prefix!(zetta); "Zkat/m³",
            "zettakatal per cubic meter", "zettakatals per cubic meter";
        @exakatal_per_cubic_meter: prefix!(exa); "Ekat/m³",
            "exakatal per cubic meter", "exakatals per cubic meter";
        @petakatal_per_cubic_meter: prefix!(peta); "Pkat/m³",
            "petakatal per cubic meter", "petakatals per cubic meter";
        @terakatal_per_cubic_meter: prefix!(tera); "Tkat/m³",
            "terakatal per cubic meter", "terakatals per cubic meter";
        @gigakatal_per_cubic_meter: prefix!(giga); "Gkat/m³",
            "gigakatal per cubic meter", "gigakatals per cubic meter";
        @megakatal_per_cubic_meter: prefix!(mega); "Mkat/m³",
            "megakatal per cubic meter", "megakatals per cubic meter";
        @kilokatal_per_cubic_meter: prefix!(kilo); "kkat/m³",
            "kilokatal per cubic meter", "kilokatals per cubic meter";
        @hectokatal_per_cubic_meter: prefix!(hecto); "hkat/m³",
            "hectokatal per cubic meter", "hectokatals per cubic meter";
        @decakatal_per_cubic_meter: prefix!(deca); "dakat/m³",
            "decakatal per cubic meter", "decakatals per cubic meter";
        @katal_per_cubic_meter: prefix!(none); "kat/m³",
            "katal per cubic meter", "katals per cubic meter";
        @decikatal_per_cubic_meter: prefix!(deci); "dkat/m³",
            "decikatal per cubic meter", "decikatals per cubic meter";
        @centikatal_per_cubic_meter: prefix!(centi); "ckat/m³",
            "centikatal per cubic meter", "centikatals per cubic meter";
        @millikatal_per_cubic_meter: prefix!(milli); "mkat/m³",
            "millikatal per cubic meter", "millikatals per cubic meter";
        @microkatal_per_cubic_meter: prefix!(micro); "µkat/m³",
            "microkatal per cubic meter", "microkatals per cubic meter";
        @nanokatal_per_cubic_meter: prefix!(nano); "nkat/m³",
            "nanokatal per cubic meter", "nanokatals per cubic meter";
        @picokatal_per_cubic_meter: prefix!(pico); "pkat/m³",
            "picokatal per cubic meter", "picokatals per cubic meter";
        @femtokatal_per_cubic_meter: prefix!(femto); "fkat/m³",
            "femtokatal per cubic meter", "femtokatals per cubic meter";
        @attokatal_per_cubic_meter: prefix!(atto); "akat/m³",
            "attokatal per cubic meter", "attokatals per cubic meter";
        @zeptokatal_per_cubic_meter: prefix!(zepto); "zkat/m³",
            "zeptokatal per cubic meter", "zeptokatals per cubic meter";
        @yoctokatal_per_cubic_meter: prefix!(yocto); "ykat/m³",
            "yoctokatal per cubic meter", "yoctokatals per cubic meter";

        @kilokatal_per_cubic_decimeter:
            prefix!(kilo) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "kkat/dm³", "kilokatal per cubic decimeter", "kilokatals per cubic decimeter";
        @kilokatal_per_liter:
            prefix!(kilo) / prefix!(milli);
            "kkat/L", "kilokatal per liter", "kilokatals per liter";
        @katal_per_cubic_decimeter:
            prefix!(none) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "kat/dm³", "katal per cubic decimeter", "katals per cubic decimeter";
        @katal_per_liter:
            prefix!(none) / prefix!(milli);
            "kat/L", "katal per liter", "katals per liter";
        @millikatal_per_cubic_decimeter:
            prefix!(milli) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "mkat/dm³", "millikatal per cubic decimeter", "millikatals per cubic decimeter";
        @millikatal_per_liter:
            prefix!(milli) / prefix!(milli);
            "mkat/L", "millikatal per liter", "millikatals per liter";
        @microkatal_per_cubic_decimeter:
            prefix!(micro) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "μkat/dm³", "microkatal per cubic decimeter", "microkatals per cubic decimeter";
        @microkatal_per_liter:
            prefix!(micro) / prefix!(milli);
            "μkat/L", "microkatal per liter", "microkatals per liter";
        @nanokatal_per_cubic_decimeter:
            prefix!(nano) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "nkat/dm³", "nanokatal per cubic decimeter", "nanokatals per cubic decimeter";
        @nanokatal_per_liter:
            prefix!(nano) / prefix!(milli);
            "nkat/L", "nanokatal per liter", "nanokatals per liter";
        @picokatal_per_cubic_decimeter:
            prefix!(pico) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "pkat/dm³", "picokatal per cubic decimeter", "picokatals per cubic decimeter";
        @picokatal_per_liter:
            prefix!(pico) / prefix!(milli);
            "pkat/L", "picokatal per liter", "picokatals per liter";
        @femtokatal_per_cubic_decimeter:
            prefix!(femto) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "fkat/dm³", "femtokatal per cubic decimeter", "femtokatals per cubic decimeter";
        @femtokatal_per_liter:
            prefix!(femto) / prefix!(milli);
            "fkat/L", "femtokatal per liter", "femtokatals per liter";

        @kilokatal_per_deciliter:
            prefix!(kilo) / prefix!(deci) / prefix!(milli);
            "kkat/dL", "kilokatal per deciliter", "kilokatals per deciliter";
        @katal_per_deciliter:
            prefix!(none) / prefix!(deci) / prefix!(milli);
            "kat/dL", "katal per deciliter", "katals per deciliter";
        @millikatal_per_deciliter:
            prefix!(milli) / prefix!(deci) / prefix!(milli);
            "mkat/dL", "millikatal per deciliter", "millikatals per deciliter";
        @microkatal_per_deciliter:
            prefix!(micro) / prefix!(deci) / prefix!(milli);
            "μkat/dL", "microkatal per deciliter", "microkatals per deciliter";
        @nanokatal_per_deciliter:
            prefix!(nano) / prefix!(deci) / prefix!(milli);
            "nkat/dL", "nanokatal per deciliter", "nanokatals per deciliter";
        @picokatal_per_deciliter:
            prefix!(pico) / prefix!(deci) / prefix!(milli);
            "pkat/dL", "picokatal per deciliter", "picokatals per deciliter";
        @femtokatal_per_deciliter:
            prefix!(femto) / prefix!(deci) / prefix!(milli);
            "fkat/dL", "femtokatal per deciliter", "femtokatals per deciliter";

        @kilokatal_per_milliliter:
            prefix!(kilo) / prefix!(milli) / prefix!(milli);
            "kkat/mL", "kilokatal per milliliter", "kilokatals per milliliter";
        @katal_per_milliliter:
            prefix!(none) / prefix!(milli) / prefix!(milli);
            "kat/mL", "katal per milliliter", "katals per milliliter";
        @millikatal_per_milliliter:
            prefix!(milli) / prefix!(milli) / prefix!(milli);
            "mkat/mL", "millikatal per milliliter", "millikatals per milliliter";
        @microkatal_per_milliliter:
            prefix!(micro) / prefix!(milli) / prefix!(milli);
            "μkat/mL", "microkatal per milliliter", "microkatals per milliliter";
        @nanokatal_per_milliliter:
            prefix!(nano) / prefix!(milli) / prefix!(milli);
            "nkat/mL", "nanokatal per milliliter", "nanokatals per milliliter";
        @picokatal_per_milliliter:
            prefix!(pico) / prefix!(milli) / prefix!(milli);
            "pkat/mL", "picokatal per milliliter", "picokatals per milliliter";
        @femtokatal_per_milliliter:
            prefix!(femto) / prefix!(milli) / prefix!(milli);
            "fkat/mL", "femtokatal per milliliter", "femtokatals per milliliter";

        @yotta_enzyme_unit_per_cubic_meter: prefix!(yotta) * prefix!(micro) / 6.0_E1; "YU/m³",
            "yotta enzyme unit per cubic meter", "yotta enzyme units per cubic meter";
        @zetta_enzyme_unit_per_cubic_meter: prefix!(zetta) * prefix!(micro) / 6.0_E1; "ZU/m³",
            "zetta enzyme unit per cubic meter", "zetta enzyme units per cubic meter";
        @exa_enzyme_unit_per_cubic_meter: prefix!(exa) * prefix!(micro) / 6.0_E1; "EU/m³",
            "exa enzyme unit per cubic meter", "exa enzyme units per cubic meter";
        @peta_enzyme_unit_per_cubic_meter: prefix!(peta) * prefix!(micro) / 6.0_E1; "PU/m³",
            "peta enzyme unit per cubic meter", "peta enzyme units per cubic meter";
        @tera_enzyme_unit_per_cubic_meter: prefix!(tera) * prefix!(micro) / 6.0_E1; "TU/m³",
            "tera enzyme unit per cubic meter", "tera enzyme units per cubic meter";
        @giga_enzyme_unit_per_cubic_meter: prefix!(giga) * prefix!(micro) / 6.0_E1; "GU/m³",
            "giga enzyme unit per cubic meter", "giga enzyme units per cubic meter";
        @mega_enzyme_unit_per_cubic_meter: prefix!(mega) * prefix!(micro) / 6.0_E1; "MU/m³",
            "mega enzyme unit per cubic meter", "mega enzyme units per cubic meter";
        @kilo_enzyme_unit_per_cubic_meter: prefix!(kilo) * prefix!(micro) / 6.0_E1; "kU/m³",
            "kilo enzyme unit per cubic meter", "kilo enzyme units per cubic meter";
        @hecto_enzyme_unit_per_cubic_meter: prefix!(hecto) * prefix!(micro) / 6.0_E1; "hU/m³",
            "hecto enzyme unit per cubic meter", "hecto enzyme units per cubic meter";
        @deca_enzyme_unit_per_cubic_meter: prefix!(deca) * prefix!(micro) / 6.0_E1; "daU/m³",
            "deca enzyme unit per cubic meter", "deca enzyme units per cubic meter";
        @enzyme_unit_per_cubic_meter: prefix!(none) * prefix!(micro) / 6.0_E1; "U/m³",
            "enzyme unit per cubic meter", "enzyme units per cubic meter";
        @deci_enzyme_unit_per_cubic_meter: prefix!(deci) * prefix!(micro) / 6.0_E1; "dU/m³",
            "deci enzyme unit per cubic meter", "deci enzyme units per cubic meter";
        @centi_enzyme_unit_per_cubic_meter: prefix!(centi) * prefix!(micro) / 6.0_E1; "cU/m³",
            "centi enzyme unit per cubic meter", "centi enzyme units per cubic meter";
        @milli_enzyme_unit_per_cubic_meter: prefix!(milli) * prefix!(micro) / 6.0_E1; "mU/m³",
            "milli enzyme unit per cubic meter", "milli enzyme units per cubic meter";
        @micro_enzyme_unit_per_cubic_meter: prefix!(micro) * prefix!(micro) / 6.0_E1; "µU/m³",
            "micro enzyme unit per cubic meter", "micro enzyme units per cubic meter";
        @nano_enzyme_unit_per_cubic_meter: prefix!(nano) * prefix!(micro) / 6.0_E1; "nU/m³",
            "nano enzyme unit per cubic meter", "nano enzyme units per cubic meter";
        @pico_enzyme_unit_per_cubic_meter: prefix!(pico) * prefix!(micro) / 6.0_E1; "pU/m³",
            "pico enzyme unit per cubic meter", "pico enzyme units per cubic meter";
        @femto_enzyme_unit_per_cubic_meter: prefix!(femto) * prefix!(micro) / 6.0_E1; "fU/m³",
            "femto enzyme unit per cubic meter", "femto enzyme units per cubic meter";
        @atto_enzyme_unit_per_cubic_meter: prefix!(atto) * prefix!(micro) / 6.0_E1; "aU/m³",
            "atto enzyme unit per cubic meter", "atto enzyme units per cubic meter";
        @zepto_enzyme_unit_per_cubic_meter: prefix!(zepto) * prefix!(micro) / 6.0_E1; "zU/m³",
            "zepto enzyme unit per cubic meter", "zepto enzyme units per cubic meter";
        @yocto_enzyme_unit_per_cubic_meter: prefix!(yocto) * prefix!(micro) / 6.0_E1; "yU/m³",
            "yocto enzyme unit per cubic meter", "yocto enzyme units per cubic meter";

        @kilo_enzyme_unit_per_cubic_decimeter:
            prefix!(kilo) * prefix!(micro) / 6.0_E1 / prefix!(milli); "kU/dm³",
            "kilo enzyme unit per cubic decimeter", "kilo enzyme units per cubic decimeter";
        @kilo_enzyme_unit_per_liter:
            prefix!(kilo) * prefix!(micro) / 6.0_E1 / prefix!(milli); "kU/L",
            "kilo enzyme unit per liter", "kilo enzyme units per liter";
        @enzyme_unit_per_cubic_decimeter:
            prefix!(none) * prefix!(micro) / 6.0_E1 / prefix!(milli); "U/dm³",
            "enzyme unit per cubic decimeter", "enzyme units per cubic decimeter";
        @enzyme_unit_per_liter:
            prefix!(none) * prefix!(micro) / 6.0_E1 / prefix!(milli); "U/L",
            "enzyme unit per liter", "enzyme units per liter";
        @milli_enzyme_unit_per_cubic_decimeter:
            prefix!(milli) * prefix!(micro) / 6.0_E1 / prefix!(milli); "mU/dm³",
            "milli enzyme unit per cubic decimeter", "milli enzyme units per cubic decimeter";
        @milli_enzyme_unit_per_liter:
            prefix!(milli) * prefix!(micro) / 6.0_E1 / prefix!(milli); "mU/L",
            "milli enzyme unit per liter", "milli enzyme units per liter";
        @micro_enzyme_unit_per_cubic_decimeter:
            prefix!(micro) * prefix!(micro) / 6.0_E1 / prefix!(milli); "μU/dm³",
            "micro enzyme unit per cubic decimeter", "micro enzyme units per cubic decimeter";
        @micro_enzyme_unit_per_liter:
            prefix!(micro) * prefix!(micro) / 6.0_E1 / prefix!(milli); "μU/L",
            "micro enzyme unit per liter", "micro enzyme units per liter";
        @nano_enzyme_unit_per_cubic_decimeter:
            prefix!(nano) * prefix!(micro) / 6.0_E1 / prefix!(milli); "nU/dm³",
            "nano enzyme unit per cubic decimeter", "nano enzyme units per cubic decimeter";
        @nano_enzyme_unit_per_liter:
            prefix!(nano) * prefix!(micro) / 6.0_E1 / prefix!(milli); "nU/L",
            "nano enzyme unit per liter", "nano enzyme units per liter";
        @pico_enzyme_unit_per_cubic_decimeter:
            prefix!(pico) * prefix!(micro) / 6.0_E1 / prefix!(milli); "pU/dm³",
            "pico enzyme unit per cubic decimeter", "pico enzyme units per cubic decimeter";
        @pico_enzyme_unit_per_liter:
            prefix!(pico) * prefix!(micro) / 6.0_E1 / prefix!(milli); "pU/L",
            "pico enzyme unit per liter", "pico enzyme units per liter";
        @femto_enzyme_unit_per_cubic_decimeter:
            prefix!(femto) * prefix!(micro) / 6.0_E1 / prefix!(milli); "fU/dm³",
            "femto enzyme unit per cubic decimeter", "femto enzyme units per cubic decimeter";
        @femto_enzyme_unit_per_liter:
            prefix!(femto) * prefix!(micro) / 6.0_E1 / prefix!(milli); "fU/L",
            "femto enzyme unit per liter", "femto enzyme units per liter";

        @kilo_enzyme_unit_per_deciliter:
            prefix!(kilo) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "kU/dL", "kilo enzyme unit per deciliter", "kilo enzyme units per deciliter";
        @enzyme_unit_per_deciliter:
            prefix!(none) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "U/dL", "enzyme unit per deciliter", "enzyme units per deciliter";
        @milli_enzyme_unit_per_deciliter:
            prefix!(milli) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "mU/dL", "milli enzyme unit per deciliter", "milli enzyme units per deciliter";
        @micro_enzyme_unit_per_deciliter:
            prefix!(micro) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "μU/dL", "micro enzyme unit per deciliter", "micro enzyme units per deciliter";
        @nano_enzyme_unit_per_deciliter:
            prefix!(nano) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "nU/dL", "nano enzyme unit per deciliter", "nano enzyme units per deciliter";
        @pico_enzyme_unit_per_deciliter:
            prefix!(pico) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "pU/dL", "pico enzyme unit per deciliter", "pico enzyme units per deciliter";
        @femto_enzyme_unit_per_deciliter:
            prefix!(femto) * prefix!(micro) / 6.0_E1 / prefix!(deci) / prefix!(milli);
            "fU/dL", "femto enzyme unit per deciliter", "femto enzyme units per deciliter";

        @kilo_enzyme_unit_per_milliliter:
            prefix!(kilo) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "kU/mL", "kilo enzyme unit per milliliter", "kilo enzyme units per milliliter";
        @enzyme_unit_per_milliliter:
            prefix!(none) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "U/mL", "enzyme unit per milliliter", "enzyme units per milliliter";
        @milli_enzyme_unit_per_milliliter:
            prefix!(milli) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "mU/mL", "milli enzyme unit per milliliter", "milli enzyme units per milliliter";
        @micro_enzyme_unit_per_milliliter:
            prefix!(micro) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "μU/mL", "micro enzyme unit per milliliter", "micro enzyme units per milliliter";
        @nano_enzyme_unit_per_milliliter:
            prefix!(nano) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "nU/mL", "nano enzyme unit per milliliter", "nano enzyme units per milliliter";
        @pico_enzyme_unit_per_milliliter:
            prefix!(pico) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "pU/mL", "pico enzyme unit per milliliter", "pico enzyme units per milliliter";
        @femto_enzyme_unit_per_milliliter:
            prefix!(femto) * prefix!(micro) / 6.0_E1 / prefix!(milli) / prefix!(milli);
            "fU/mL", "femto enzyme unit per milliliter", "femto enzyme units per milliliter";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::catalytic_activity as ca;
        use si::volume as v;
        use si::catalytic_activity_concentration as c;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: CatalyticActivityConcentration<V> =
                (CatalyticActivity::new::<ca::katal>(V::one())
                    / Volume::new::<v::cubic_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<ca::yottakatal, v::cubic_meter, c::yottakatal_per_cubic_meter>();
            test::<ca::zettakatal, v::cubic_meter, c::zettakatal_per_cubic_meter>();
            test::<ca::exakatal, v::cubic_meter, c::exakatal_per_cubic_meter>();
            test::<ca::petakatal, v::cubic_meter, c::petakatal_per_cubic_meter>();
            test::<ca::terakatal, v::cubic_meter, c::terakatal_per_cubic_meter>();
            test::<ca::gigakatal, v::cubic_meter, c::gigakatal_per_cubic_meter>();
            test::<ca::megakatal, v::cubic_meter, c::megakatal_per_cubic_meter>();
            test::<ca::kilokatal, v::cubic_meter, c::kilokatal_per_cubic_meter>();
            test::<ca::hectokatal, v::cubic_meter, c::hectokatal_per_cubic_meter>();
            test::<ca::decakatal, v::cubic_meter, c::decakatal_per_cubic_meter>();
            test::<ca::katal, v::cubic_meter, c::katal_per_cubic_meter>();
            test::<ca::decikatal, v::cubic_meter, c::decikatal_per_cubic_meter>();
            test::<ca::centikatal, v::cubic_meter, c::centikatal_per_cubic_meter>();
            test::<ca::millikatal, v::cubic_meter, c::millikatal_per_cubic_meter>();
            test::<ca::microkatal, v::cubic_meter, c::microkatal_per_cubic_meter>();
            test::<ca::nanokatal, v::cubic_meter, c::nanokatal_per_cubic_meter>();
            test::<ca::picokatal, v::cubic_meter, c::picokatal_per_cubic_meter>();
            test::<ca::femtokatal, v::cubic_meter, c::femtokatal_per_cubic_meter>();
            test::<ca::attokatal, v::cubic_meter, c::attokatal_per_cubic_meter>();
            test::<ca::zeptokatal, v::cubic_meter, c::zeptokatal_per_cubic_meter>();
            test::<ca::yoctokatal, v::cubic_meter, c::yoctokatal_per_cubic_meter>();

            test::<ca::kilokatal, v::cubic_decimeter, c::kilokatal_per_cubic_decimeter>();
            test::<ca::kilokatal, v::liter, c::kilokatal_per_liter>();
            test::<ca::katal, v::cubic_decimeter, c::katal_per_cubic_decimeter>();
            test::<ca::katal, v::liter, c::katal_per_liter>();
            test::<ca::millikatal, v::cubic_decimeter, c::millikatal_per_cubic_decimeter>();
            test::<ca::millikatal, v::liter, c::millikatal_per_liter>();
            test::<ca::microkatal, v::cubic_decimeter, c::microkatal_per_cubic_decimeter>();
            test::<ca::microkatal, v::liter, c::microkatal_per_liter>();
            test::<ca::nanokatal, v::cubic_decimeter, c::nanokatal_per_cubic_decimeter>();
            test::<ca::nanokatal, v::liter, c::nanokatal_per_liter>();
            test::<ca::picokatal, v::cubic_decimeter, c::picokatal_per_cubic_decimeter>();
            test::<ca::picokatal, v::liter, c::picokatal_per_liter>();
            test::<ca::femtokatal, v::cubic_decimeter, c::femtokatal_per_cubic_decimeter>();
            test::<ca::femtokatal, v::liter, c::femtokatal_per_liter>();

            test::<ca::kilokatal, v::deciliter, c::kilokatal_per_deciliter>();
            test::<ca::katal, v::deciliter, c::katal_per_deciliter>();
            test::<ca::millikatal, v::deciliter, c::millikatal_per_deciliter>();
            test::<ca::microkatal, v::deciliter, c::microkatal_per_deciliter>();
            test::<ca::nanokatal, v::deciliter, c::nanokatal_per_deciliter>();
            test::<ca::picokatal, v::deciliter, c::picokatal_per_deciliter>();
            test::<ca::femtokatal, v::deciliter, c::femtokatal_per_deciliter>();

            test::<ca::kilokatal, v::milliliter, c::kilokatal_per_milliliter>();
            test::<ca::katal, v::milliliter, c::katal_per_milliliter>();
            test::<ca::millikatal, v::milliliter, c::millikatal_per_milliliter>();
            test::<ca::microkatal, v::milliliter, c::microkatal_per_milliliter>();
            test::<ca::nanokatal, v::milliliter, c::nanokatal_per_milliliter>();
            test::<ca::picokatal, v::milliliter, c::picokatal_per_milliliter>();
            test::<ca::femtokatal, v::milliliter, c::femtokatal_per_milliliter>();

            test::<ca::yotta_enzyme_unit, v::cubic_meter, c::yotta_enzyme_unit_per_cubic_meter>();
            test::<ca::zetta_enzyme_unit, v::cubic_meter, c::zetta_enzyme_unit_per_cubic_meter>();
            test::<ca::exa_enzyme_unit, v::cubic_meter, c::exa_enzyme_unit_per_cubic_meter>();
            test::<ca::peta_enzyme_unit, v::cubic_meter, c::peta_enzyme_unit_per_cubic_meter>();
            test::<ca::tera_enzyme_unit, v::cubic_meter, c::tera_enzyme_unit_per_cubic_meter>();
            test::<ca::giga_enzyme_unit, v::cubic_meter, c::giga_enzyme_unit_per_cubic_meter>();
            test::<ca::mega_enzyme_unit, v::cubic_meter, c::mega_enzyme_unit_per_cubic_meter>();
            test::<ca::kilo_enzyme_unit, v::cubic_meter, c::kilo_enzyme_unit_per_cubic_meter>();
            test::<ca::hecto_enzyme_unit, v::cubic_meter, c::hecto_enzyme_unit_per_cubic_meter>();
            test::<ca::deca_enzyme_unit, v::cubic_meter, c::deca_enzyme_unit_per_cubic_meter>();
            test::<ca::enzyme_unit, v::cubic_meter, c::enzyme_unit_per_cubic_meter>();
            test::<ca::deci_enzyme_unit, v::cubic_meter, c::deci_enzyme_unit_per_cubic_meter>();
            test::<ca::centi_enzyme_unit, v::cubic_meter, c::centi_enzyme_unit_per_cubic_meter>();
            test::<ca::milli_enzyme_unit, v::cubic_meter, c::milli_enzyme_unit_per_cubic_meter>();
            test::<ca::micro_enzyme_unit, v::cubic_meter, c::micro_enzyme_unit_per_cubic_meter>();
            test::<ca::nano_enzyme_unit, v::cubic_meter, c::nano_enzyme_unit_per_cubic_meter>();
            test::<ca::pico_enzyme_unit, v::cubic_meter, c::pico_enzyme_unit_per_cubic_meter>();
            test::<ca::femto_enzyme_unit, v::cubic_meter, c::femto_enzyme_unit_per_cubic_meter>();
            test::<ca::atto_enzyme_unit, v::cubic_meter, c::atto_enzyme_unit_per_cubic_meter>();
            test::<ca::zepto_enzyme_unit, v::cubic_meter, c::zepto_enzyme_unit_per_cubic_meter>();
            test::<ca::yocto_enzyme_unit, v::cubic_meter, c::yocto_enzyme_unit_per_cubic_meter>();

            test::<ca::kilo_enzyme_unit, v::cubic_decimeter,
                c::kilo_enzyme_unit_per_cubic_decimeter>();
            test::<ca::kilo_enzyme_unit, v::liter, c::kilo_enzyme_unit_per_liter>();
            test::<ca::enzyme_unit, v::cubic_decimeter,
                c::enzyme_unit_per_cubic_decimeter>();
            test::<ca::enzyme_unit, v::liter, c::enzyme_unit_per_liter>();
            test::<ca::milli_enzyme_unit, v::cubic_decimeter,
                c::milli_enzyme_unit_per_cubic_decimeter>();
            test::<ca::milli_enzyme_unit, v::liter, c::milli_enzyme_unit_per_liter>();
            test::<ca::micro_enzyme_unit, v::cubic_decimeter,
                c::micro_enzyme_unit_per_cubic_decimeter>();
            test::<ca::micro_enzyme_unit, v::liter, c::micro_enzyme_unit_per_liter>();
            test::<ca::nano_enzyme_unit, v::cubic_decimeter,
                c::nano_enzyme_unit_per_cubic_decimeter>();
            test::<ca::nano_enzyme_unit, v::liter, c::nano_enzyme_unit_per_liter>();
            test::<ca::pico_enzyme_unit, v::cubic_decimeter,
                c::pico_enzyme_unit_per_cubic_decimeter>();
            test::<ca::pico_enzyme_unit, v::liter, c::pico_enzyme_unit_per_liter>();
            test::<ca::femto_enzyme_unit, v::cubic_decimeter,
                c::femto_enzyme_unit_per_cubic_decimeter>();
            test::<ca::femto_enzyme_unit, v::liter, c::femto_enzyme_unit_per_liter>();

            test::<ca::kilo_enzyme_unit, v::deciliter, c::kilo_enzyme_unit_per_deciliter>();
            test::<ca::enzyme_unit, v::deciliter, c::enzyme_unit_per_deciliter>();
            test::<ca::milli_enzyme_unit, v::deciliter, c::milli_enzyme_unit_per_deciliter>();
            test::<ca::micro_enzyme_unit, v::deciliter, c::micro_enzyme_unit_per_deciliter>();
            test::<ca::nano_enzyme_unit, v::deciliter, c::nano_enzyme_unit_per_deciliter>();
            test::<ca::pico_enzyme_unit, v::deciliter, c::pico_enzyme_unit_per_deciliter>();
            test::<ca::femto_enzyme_unit, v::deciliter, c::femto_enzyme_unit_per_deciliter>();

            test::<ca::kilo_enzyme_unit, v::milliliter, c::kilo_enzyme_unit_per_milliliter>();
            test::<ca::enzyme_unit, v::milliliter, c::enzyme_unit_per_milliliter>();
            test::<ca::milli_enzyme_unit, v::milliliter, c::milli_enzyme_unit_per_milliliter>();
            test::<ca::micro_enzyme_unit, v::milliliter, c::micro_enzyme_unit_per_milliliter>();
            test::<ca::nano_enzyme_unit, v::milliliter, c::nano_enzyme_unit_per_milliliter>();
            test::<ca::pico_enzyme_unit, v::milliliter, c::pico_enzyme_unit_per_milliliter>();
            test::<ca::femto_enzyme_unit, v::milliliter, c::femto_enzyme_unit_per_milliliter>();

            fn test<CA: ca::Conversion<V>, U: v::Conversion<V>, C: c::Conversion<V>>() {
                Test::assert_approx_eq(&CatalyticActivityConcentration::new::<C>(V::one()),
                    &(CatalyticActivity::new::<CA>(V::one()) / Volume::new::<U>(V::one())).into());
            }
        }
    }
}
