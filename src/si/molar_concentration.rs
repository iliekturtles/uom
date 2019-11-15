//! Molar concentration (base unit mole per cubic meter, mol · m⁻³).

quantity! {
    /// Molar concentration (base unit mole per cubic meter, mol · m⁻³).
    quantity: MolarConcentration; "molar concentration";
    /// Dimension of molar concentration, L⁻³N (base unit mole per cubic meter, mol · m⁻³).
    dimension: ISQ<
        N3,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        P1,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn(::si::marker::ConstituentConcentrationKind);
    units {
        @yottamole_per_cubic_meter: prefix!(yotta); "Ymol/m³",
            "yottamole per cubic meter", "yottamoles per cubic meter";
        @zettamole_per_cubic_meter: prefix!(zetta); "Zmol/m³",
            "zettamole per cubic meter", "zettamoles per cubic meter";
        @examole_per_cubic_meter: prefix!(exa); "Emol/m³",
            "examole per cubic meter", "examoles per cubic meter";
        @petamole_per_cubic_meter: prefix!(peta); "Pmol/m³",
            "petamole per cubic meter", "petamoles per cubic meter";
        @teramole_per_cubic_meter: prefix!(tera); "Tmol/m³",
            "teramole per cubic meter", "teramoles per cubic meter";
        @gigamole_per_cubic_meter: prefix!(giga); "Gmol/m³",
            "gigamole per cubic meter", "gigamoles per cubic meter";
        @megamole_per_cubic_meter: prefix!(mega); "Mmol/m³",
            "megamole per cubic meter", "megamoles per cubic meter";
        @kilomole_per_cubic_meter: prefix!(kilo); "kmol/m³",
            "kilomole per cubic meter", "kilomoles per cubic meter";
        @hectomole_per_cubic_meter: prefix!(hecto); "hmol/m³",
            "hectomole per cubic meter", "hectomoles per cubic meter";
        @decamole_per_cubic_meter: prefix!(deca); "damol/m³",
            "decamole per cubic meter", "decamoles per cubic meter";
        @mole_per_cubic_meter: prefix!(none); "mol/m³",
            "mole per cubic meter", "moles per cubic meter";
        @decimole_per_cubic_meter: prefix!(deci); "dmol/m³",
            "decimole per cubic meter", "decimoles per cubic meter";
        @centimole_per_cubic_meter: prefix!(centi); "cmol/m³",
            "centimole per cubic meter", "centimoles per cubic meter";
        @millimole_per_cubic_meter: prefix!(milli); "mmol/m³",
            "millimole per cubic meter", "millimoles per cubic meter";
        @micromole_per_cubic_meter: prefix!(micro); "µmol/m³",
            "micromole per cubic meter", "micromoles per cubic meter";
        @nanomole_per_cubic_meter: prefix!(nano); "nmol/m³",
            "nanomole per cubic meter", "nanomoles per cubic meter";
        @picomole_per_cubic_meter: prefix!(pico); "pmol/m³",
            "picomole per cubic meter", "picomoles per cubic meter";
        @femtomole_per_cubic_meter: prefix!(femto); "fmol/m³",
            "femtomole per cubic meter", "femtomoles per cubic meter";
        @attomole_per_cubic_meter: prefix!(atto); "amol/m³",
            "attomole per cubic meter", "attomoles per cubic meter";
        @zeptomole_per_cubic_meter: prefix!(zepto); "zmol/m³",
            "zeptomole per cubic meter", "zeptomoles per cubic meter";
        @yoctomole_per_cubic_meter: prefix!(yocto); "ymol/m³",
            "yoctomole per cubic meter", "yoctomoles per cubic meter";

        @kilomole_per_cubic_decimeter:
            prefix!(kilo) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "kmol/dm³", "kilomole per cubic decimeter", "kilomoles per cubic decimeter";
        @kilomole_per_liter:
            prefix!(kilo) / prefix!(milli);
            "kmol/L", "kilomole per liter", "kilomoles per liter";
        @mole_per_cubic_decimeter:
            prefix!(none) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "mol/dm³", "mole per cubic decimeter", "moles per cubic decimeter";
        @mole_per_liter:
            prefix!(none) / prefix!(milli);
            "mol/L", "mole per liter", "moles per liter";
        @millimole_per_cubic_decimeter:
            prefix!(milli) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "mmol/dm³", "millimole per cubic decimeter", "millimoles per cubic decimeter";
        @millimole_per_liter:
            prefix!(milli) / prefix!(milli);
            "mmol/L", "millimole per liter", "millimoles per liter";
        @micromole_per_cubic_decimeter:
            prefix!(micro) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "μmol/dm³", "micromole per cubic decimeter", "micromoles per cubic decimeter";
        @micromole_per_liter:
            prefix!(micro) / prefix!(milli);
            "μmol/L", "micromole per liter", "micromoles per liter";
        @nanomole_per_cubic_decimeter:
            prefix!(nano) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "nmol/dm³", "nanomole per cubic decimeter", "nanomoles per cubic decimeter";
        @nanomole_per_liter:
            prefix!(nano) / prefix!(milli);
            "nmol/L", "nanomole per liter", "nanomoles per liter";
        @picomole_per_cubic_decimeter:
            prefix!(pico) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "pmol/dm³", "picomole per cubic decimeter", "picomoles per cubic decimeter";
        @picomole_per_liter:
            prefix!(pico) / prefix!(milli);
            "pmol/L", "picomole per liter", "picomoles per liter";
        @femtomole_per_cubic_decimeter:
            prefix!(femto) / prefix!(deci) / prefix!(deci) / prefix!(deci);
            "fmol/dm³", "femtomole per cubic decimeter", "femtomoles per cubic decimeter";
        @femtomole_per_liter:
            prefix!(femto) / prefix!(milli);
            "fmol/L", "femtomole per liter", "femtomoles per liter";

        @kilomole_per_deciliter:
            prefix!(kilo) / prefix!(deci) / prefix!(milli);
            "kmol/dL", "kilomole per deciliter", "kilomoles per deciliter";
        @mole_per_deciliter:
            prefix!(none) / prefix!(deci) / prefix!(milli);
            "mol/dL", "mole per deciliter", "moles per deciliter";
        @millimole_per_deciliter:
            prefix!(milli) / prefix!(deci) / prefix!(milli);
            "mmol/dL", "millimole per deciliter", "millimoles per deciliter";
        @micromole_per_deciliter:
            prefix!(micro) / prefix!(deci) / prefix!(milli);
            "μmol/dL", "micromole per deciliter", "micromoles per deciliter";
        @nanomole_per_deciliter:
            prefix!(nano) / prefix!(deci) / prefix!(milli);
            "nmol/dL", "nanomole per deciliter", "nanomoles per deciliter";
        @picomole_per_deciliter:
            prefix!(pico) / prefix!(deci) / prefix!(milli);
            "pmol/dL", "picomole per deciliter", "picomoles per deciliter";
        @femtomole_per_deciliter:
            prefix!(femto) / prefix!(deci) / prefix!(milli);
            "fmol/dL", "femtomole per deciliter", "femtomoles per deciliter";

        @kilomole_per_milliliter:
            prefix!(kilo) / prefix!(milli) / prefix!(milli);
            "kmol/mL", "kilomole per milliliter", "kilomoles per milliliter";
        @mole_per_milliliter:
            prefix!(none) / prefix!(milli) / prefix!(milli);
            "mol/mL", "mole per milliliter", "moles per milliliter";
        @millimole_per_milliliter:
            prefix!(milli) / prefix!(milli) / prefix!(milli);
            "mmol/mL", "millimole per milliliter", "millimoles per milliliter";
        @micromole_per_milliliter:
            prefix!(micro) / prefix!(milli) / prefix!(milli);
            "μmol/mL", "micromole per milliliter", "micromoles per milliliter";
        @nanomole_per_milliliter:
            prefix!(nano) / prefix!(milli) / prefix!(milli);
            "nmol/mL", "nanomole per milliliter", "nanomoles per milliliter";
        @picomole_per_milliliter:
            prefix!(pico) / prefix!(milli) / prefix!(milli);
            "pmol/mL", "picomole per milliliter", "picomoles per milliliter";
        @femtomole_per_milliliter:
            prefix!(femto) / prefix!(milli) / prefix!(milli);
            "fmol/mL", "femtomole per milliliter", "femtomoles per milliliter";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::amount_of_substance as aos;
        use si::volume as v;
        use si::molar_concentration as c;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: MolarConcentration<V> = (AmountOfSubstance::new::<aos::mole>(V::one())
                / Volume::new::<v::cubic_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<aos::yottamole, v::cubic_meter, c::yottamole_per_cubic_meter>();
            test::<aos::zettamole, v::cubic_meter, c::zettamole_per_cubic_meter>();
            test::<aos::examole, v::cubic_meter, c::examole_per_cubic_meter>();
            test::<aos::petamole, v::cubic_meter, c::petamole_per_cubic_meter>();
            test::<aos::teramole, v::cubic_meter, c::teramole_per_cubic_meter>();
            test::<aos::gigamole, v::cubic_meter, c::gigamole_per_cubic_meter>();
            test::<aos::megamole, v::cubic_meter, c::megamole_per_cubic_meter>();
            test::<aos::kilomole, v::cubic_meter, c::kilomole_per_cubic_meter>();
            test::<aos::hectomole, v::cubic_meter, c::hectomole_per_cubic_meter>();
            test::<aos::decamole, v::cubic_meter, c::decamole_per_cubic_meter>();
            test::<aos::mole, v::cubic_meter, c::mole_per_cubic_meter>();
            test::<aos::decimole, v::cubic_meter, c::decimole_per_cubic_meter>();
            test::<aos::centimole, v::cubic_meter, c::centimole_per_cubic_meter>();
            test::<aos::millimole, v::cubic_meter, c::millimole_per_cubic_meter>();
            test::<aos::micromole, v::cubic_meter, c::micromole_per_cubic_meter>();
            test::<aos::nanomole, v::cubic_meter, c::nanomole_per_cubic_meter>();
            test::<aos::picomole, v::cubic_meter, c::picomole_per_cubic_meter>();
            test::<aos::femtomole, v::cubic_meter, c::femtomole_per_cubic_meter>();
            test::<aos::attomole, v::cubic_meter, c::attomole_per_cubic_meter>();
            test::<aos::zeptomole, v::cubic_meter, c::zeptomole_per_cubic_meter>();
            test::<aos::yoctomole, v::cubic_meter, c::yoctomole_per_cubic_meter>();

            test::<aos::kilomole, v::cubic_decimeter, c::kilomole_per_cubic_decimeter>();
            test::<aos::kilomole, v::liter, c::kilomole_per_liter>();
            test::<aos::mole, v::cubic_decimeter, c::mole_per_cubic_decimeter>();
            test::<aos::mole, v::liter, c::mole_per_liter>();
            test::<aos::millimole, v::cubic_decimeter, c::millimole_per_cubic_decimeter>();
            test::<aos::millimole, v::liter, c::millimole_per_liter>();
            test::<aos::micromole, v::cubic_decimeter, c::micromole_per_cubic_decimeter>();
            test::<aos::micromole, v::liter, c::micromole_per_liter>();
            test::<aos::nanomole, v::cubic_decimeter, c::nanomole_per_cubic_decimeter>();
            test::<aos::nanomole, v::liter, c::nanomole_per_liter>();
            test::<aos::picomole, v::cubic_decimeter, c::picomole_per_cubic_decimeter>();
            test::<aos::picomole, v::liter, c::picomole_per_liter>();
            test::<aos::femtomole, v::cubic_decimeter, c::femtomole_per_cubic_decimeter>();
            test::<aos::femtomole, v::liter, c::femtomole_per_liter>();

            test::<aos::kilomole, v::deciliter, c::kilomole_per_deciliter>();
            test::<aos::mole, v::deciliter, c::mole_per_deciliter>();
            test::<aos::millimole, v::deciliter, c::millimole_per_deciliter>();
            test::<aos::micromole, v::deciliter, c::micromole_per_deciliter>();
            test::<aos::nanomole, v::deciliter, c::nanomole_per_deciliter>();
            test::<aos::picomole, v::deciliter, c::picomole_per_deciliter>();
            test::<aos::femtomole, v::deciliter, c::femtomole_per_deciliter>();

            test::<aos::kilomole, v::milliliter, c::kilomole_per_milliliter>();
            test::<aos::mole, v::milliliter, c::mole_per_milliliter>();
            test::<aos::millimole, v::milliliter, c::millimole_per_milliliter>();
            test::<aos::micromole, v::milliliter, c::micromole_per_milliliter>();
            test::<aos::nanomole, v::milliliter, c::nanomole_per_milliliter>();
            test::<aos::picomole, v::milliliter, c::picomole_per_milliliter>();
            test::<aos::femtomole, v::milliliter, c::femtomole_per_milliliter>();

            fn test<AOS: aos::Conversion<V>, U: v::Conversion<V>, C: c::Conversion<V>>() {
                Test::assert_approx_eq(&MolarConcentration::new::<C>(V::one()),
                    &(AmountOfSubstance::new::<AOS>(V::one()) / Volume::new::<U>(V::one())).into());
            }
        }
    }
}
