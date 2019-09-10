//! Molar energy (base unit joule per mole, kg · m² · s⁻² · mol⁻¹).

quantity! {
    /// Molar energy (base unit joule per mole, kg · m² · s⁻² · mol⁻¹).
    quantity: MolarEnergy; "molar energy";
    /// Dimension of molar energy, L²MT⁻²N⁻¹ (base unit joule per mole, kg · m² · s⁻² · mol⁻¹).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        N1,     // amount of substance
        Z0>;    // luminous intensity
    units {
        /// Molar energy expressed in base units. Equivalent to J/mol.
        @kilogram_square_meter_per_second_squared_mole: prefix!(kilo) / prefix!(kilo);
            "kg · m²/(s² · mol)", "kilogram square meter per second squared mole",
            "kilograms square meter per second squared mole";

        // Molar energy is commonly expressed in terms of derived unit joule
        @yottajoule_per_mole: prefix!(yotta); "YJ/mol", "yottajoule per mole",
            "yottajoules per mole";
        @zettajoule_per_mole: prefix!(zetta); "ZJ/mol", "zettajoule per mole",
            "zettajoules per mole";
        @exajoule_per_mole: prefix!(exa); "EJ/mol", "exajoule per mole", "exajoules per mole";
        @petajoule_per_mole: prefix!(peta); "PJ/mol", "petajoule per mole", "petajoules per mole";
        @terajoule_per_mole: prefix!(tera); "TJ/mol", "terajoule per mole", "terajoules per mole";
        @gigajoule_per_mole: prefix!(giga); "GJ/mol", "gigajoule per mole", "gigajoules per mole";
        @megajoule_per_mole: prefix!(mega); "MJ/mol", "megajoule per mole", "megajoules per mole";
        @kilojoule_per_mole: prefix!(kilo); "kJ/mol", "kilojoule per mole", "kilojoules per mole";
        @hectojoule_per_mole: prefix!(hecto); "hJ/mol", "hectojoule per mole",
            "hectojoules per mole";
        @decajoule_per_mole: prefix!(deca); "daJ/mol", "decajoule per mole", "decajoules per mole";
        /// The derived unit of molar energy. Equivalent to kg · m²/(s² · mol).
        @joule_per_mole: prefix!(none); "J/mol", "joule per mole", "joules per mole";
        @decijoule_per_mole: prefix!(deci); "dJ/mol", "decijoule per mole", "decijoules per mole";
        @centijoule_per_mole: prefix!(centi); "cJ/mol", "centijoule per mole",
            "centijoules per mole";
        @millijoule_per_mole: prefix!(milli); "mJ/mol", "millijoule per mole",
            "millijoules per mole";
        @microjoule_per_mole: prefix!(micro); "µJ/mol", "microjoule per mole",
            "microjoules per mole";
        @nanojoule_per_mole: prefix!(nano); "nJ/mol", "nanojoule per mole", "nanojoules per mole";
        @picojoule_per_mole: prefix!(pico); "pJ/mol", "picojoule per mole", "picojoules per mole";
        @femtojoule_per_mole: prefix!(femto); "fJ/mol", "femtojoule per mole",
            "femtojoules per mole";
        @attojoule_per_mole: prefix!(atto); "aJ/mol", "attojoule per mole", "attojoules per mole";
        @zeptojoule_per_mole: prefix!(zepto); "zJ/mol", "zeptojoule per mole",
            "zeptojoules per mole";
        @yoctojoule_per_mole: prefix!(yocto); "yJ/mol", "yoctojoule per mole",
            "yoctojoules per mole";

        @petawatt_hour_per_mole: 3.6_E18; "PW · h/mol", "petawatt hour per mole",
            "petawatt hours per mole";
        @terawatt_hour_per_mole: 3.6_E15; "TW · h/mol", "terawatt hour per mole",
            "terawatt hours per mole";
        @gigawatt_hour_per_mole: 3.6_E12; "GW · h/mol", "gigawatt hour per mole",
            "gigawatt hours per mole";
        @megawatt_hour_per_mole: 3.6_E9; "MW · h/mol", "megawatt hour per mole",
            "megawatt hours per mole";
        @kilowatt_hour_per_mole: 3.6_E6; "kW · h/mol", "kilowatt hour per mole",
            "kilowatt hours per mole";
        @hectowatt_hour_per_mole: 3.6_E5; "hW · h/mol", "hectowatt hour per mole",
            "hectowatt hours per mole";
        @decawatt_hour_per_mole: 3.6_E4; "daW · h/mol", "decawatt hour per mole",
            "decawatt hours per mole";
        @watt_hour_per_mole: 3.6_E3; "W · h/mol", "watt hour per mole", "watt hours per mole";
        @milliwatt_hour_per_mole: 3.6_E0; "mW · h/mol", "milliwatt hour per mole",
            "milliwatt hours per mole";
        @microwatt_hour_per_mole: 3.6_E-3; "µW · h/mol", "microwatt hour per mole",
            "microwatt hours per mole";

        @btu_it_per_mole: 1.055_056_E3; "Btu (IT)/mol", "British thermal unit (IT) per mole",
            "British thermal units (IT) per mole";
        @btu_per_mole: 1.054_350_E3; "Btu/mol", "British thermal unit per mole",
            "British thermal units per mole";
        @btu_39_per_mole: 1.059_67_E3; "Btu₃₉/mol", "British thermal unit (39 °F) per mole",
            "British thermal units (39 °F) per mole";
        @btu_59_per_mole: 1.054_80_E3; "Btu₅₉/mol", "British thermal unit (59 °F) per mole",
            "British thermal units (59 °F) per mole";
        @btu_60_per_mole: 1.054_68_E3; "Btu₆₀/mol", "British thermal unit (60 °F) per mole",
            "British thermal units (60 °F) per mole";
        @calorie_it_per_mole: 4.186_8_E0; "cal (IT)/mol", "calorie (IT) per mole",
            "calories (IT) per mole";
        @calorie_per_mole: 4.184_E0; "cal/mol", "calorie per mole", "calories per mole";
        @calorie_15_per_mole: 4.185_80_E0; "cal₁₅/mol", "calorie (15 °C) per mole",
            "calories (15 °C) per mole";
        @calorie_20_per_mole: 4.181_90_E0; "cal₂₀/mol", "calorie (20 °C) per mole",
            "calories (20 °C) per mole";
        @calorie_it_nutrition_per_mole: 4.186_8_E3; "Cal (IT)/mol", "Calorie (IT) per mole",
            "Calories (IT) per mole";
        @calorie_nutrition_per_mole: 4.184_E3; "Cal/mol", "Calorie per mole", "Calories per mole";
        @electronvolt_per_mole: 1.602_177_E-19; "eV/mol", "electronvolt per mole",
            "electronvolts per mole";
        @erg_per_mole: 1.0_E-7; "erg/mol", "erg per mole", "ergs per mole";
        @foot_poundal_per_mole: 4.214_011_E-2; "ft · pdl/mol", "foot poundal per mole",
            "foot poundals per mole";
        @foot_pound_force_per_mole: 1.355_818_E0; "ft · lbf/mol", "foot pound-force per mole",
            "foot pounds-force per mole";
        @kilocalorie_it_per_mole: 4.186_8_E3; "kcal (IT)/mol", "kilocalorie (IT) per mole",
            "kilocalories (IT) per mole";
        @kilocalorie_per_mole: 4.184_E3; "kcal/mol", "kilocalorie per mole",
            "kilocalories per mole";
        @quad_per_mole: 1.055_056_E18; "10¹⁵ Btu (IT)/mol", "quad per mole", "quads per mole";
        @therm_ec_per_mole: 1.055_06_E8; "thm (EC)/mol", "therm (EC) per mole",
            "therms (EC) per mole";
        @therm_us_per_mole: 1.054_804_E8; "thm/mol", "therm per mole", "therms per mole";
        @ton_tnt_per_mole: 4.184_E9; "t of TNT/mol", "ton of TNT per mole", "tons of TNT per mole";
        @watt_second_per_mole: 1.0_E0; "W · s/mol", "watt second per mole", "watt seconds per mole";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::mass as m;
        use si::length as l;
        use si::energy as e;
        use si::time as t;
        use si::amount_of_substance as aos;
        use si::molar_energy as me;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _base: MolarEnergy<V> = Mass::new::<m::kilogram>(V::one())
                * Length::new::<l::meter>(V::one())
                * Length::new::<l::meter>(V::one())
                / (Time::new::<t::second>(V::one())
                    * Time::new::<t::second>(V::one())
                    * AmountOfSubstance::new::<aos::mole>(V::one()));

            let _derived: MolarEnergy<V> = Energy::new::<e::joule>(V::one())
                / AmountOfSubstance::new::<aos::mole>(V::one());
        }

        #[test]
        fn check_base_units() {
            test::<m::kilogram, me::kilogram_square_meter_per_second_squared_mole>();

            fn test<M: m::Conversion<V>, ME: me::Conversion<V>>() {
                Test::assert_approx_eq(&MolarEnergy::new::<ME>(V::one()),
                    &(Mass::new::<M>(V::one())
                        * Length::new::<l::meter>(V::one())
                        * Length::new::<l::meter>(V::one())
                        / (Time::new::<t::second>(V::one())
                            * Time::new::<t::second>(V::one())
                            * AmountOfSubstance::new::<aos::mole>(V::one()))));
            }
        }

        #[test]
        fn check_derived_units() {
            test::<e::yottajoule, me::yottajoule_per_mole>();
            test::<e::zettajoule, me::zettajoule_per_mole>();
            test::<e::exajoule, me::exajoule_per_mole>();
            test::<e::petajoule, me::petajoule_per_mole>();
            test::<e::terajoule, me::terajoule_per_mole>();
            test::<e::gigajoule, me::gigajoule_per_mole>();
            test::<e::megajoule, me::megajoule_per_mole>();
            test::<e::kilojoule, me::kilojoule_per_mole>();
            test::<e::hectojoule, me::hectojoule_per_mole>();
            test::<e::decajoule, me::decajoule_per_mole>();
            test::<e::joule, me::joule_per_mole>();
            test::<e::decijoule, me::decijoule_per_mole>();
            test::<e::centijoule, me::centijoule_per_mole>();
            test::<e::millijoule, me::millijoule_per_mole>();
            test::<e::microjoule, me::microjoule_per_mole>();
            test::<e::nanojoule, me::nanojoule_per_mole>();
            test::<e::picojoule, me::picojoule_per_mole>();
            test::<e::femtojoule, me::femtojoule_per_mole>();
            test::<e::attojoule, me::attojoule_per_mole>();
            test::<e::zeptojoule, me::zeptojoule_per_mole>();
            test::<e::yoctojoule, me::yoctojoule_per_mole>();

            test::<e::petawatt_hour, me::petawatt_hour_per_mole>();
            test::<e::terawatt_hour, me::terawatt_hour_per_mole>();
            test::<e::gigawatt_hour, me::gigawatt_hour_per_mole>();
            test::<e::megawatt_hour, me::megawatt_hour_per_mole>();
            test::<e::kilowatt_hour, me::kilowatt_hour_per_mole>();
            test::<e::hectowatt_hour, me::hectowatt_hour_per_mole>();
            test::<e::decawatt_hour, me::decawatt_hour_per_mole>();
            test::<e::watt_hour, me::watt_hour_per_mole>();
            test::<e::milliwatt_hour, me::milliwatt_hour_per_mole>();
            test::<e::microwatt_hour, me::microwatt_hour_per_mole>();

            test::<e::btu_it, me::btu_it_per_mole>();
            test::<e::btu, me::btu_per_mole>();
            test::<e::btu_39, me::btu_39_per_mole>();
            test::<e::btu_59, me::btu_59_per_mole>();
            test::<e::btu_60, me::btu_60_per_mole>();
            test::<e::calorie_it, me::calorie_it_per_mole>();
            test::<e::calorie, me::calorie_per_mole>();
            test::<e::calorie_15, me::calorie_15_per_mole>();
            test::<e::calorie_20, me::calorie_20_per_mole>();
            test::<e::calorie_it_nutrition, me::calorie_it_nutrition_per_mole>();
            test::<e::calorie_nutrition, me::calorie_nutrition_per_mole>();
            test::<e::electronvolt, me::electronvolt_per_mole>();
            test::<e::erg, me::erg_per_mole>();
            test::<e::foot_poundal, me::foot_poundal_per_mole>();
            test::<e::foot_pound, me::foot_pound_force_per_mole>();
            test::<e::kilocalorie_it, me::kilocalorie_it_per_mole>();
            test::<e::kilocalorie, me::kilocalorie_per_mole>();
            test::<e::quad, me::quad_per_mole>();
            test::<e::therm_ec, me::therm_ec_per_mole>();
            test::<e::therm_us, me::therm_us_per_mole>();
            test::<e::ton_tnt, me::ton_tnt_per_mole>();
            test::<e::watt_second, me::watt_second_per_mole>();

            fn test<E: e::Conversion<V>, ME: me::Conversion<V>>() {
                Test::assert_approx_eq(&MolarEnergy::new::<ME>(V::one()),
                    &(Energy::new::<E>(V::one())
                        / AmountOfSubstance::new::<aos::mole>(V::one())));
            }
        }
    }
}
