//! Available energy (base unit joule per kilogram, m² · s⁻²).

quantity! {
    /// Available energy (base unit joule per kilogram, m² · s⁻²).
    quantity: AvailableEnergy; "available energy";
    /// Dimension of available energy, L²T⁻² (base unit joule per kilogram, m² · s⁻²).
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottajoule_per_kilogram: prefix!(yotta); "YJ/kg", "yottajoule per kilogram",
            "yottajoules per kilogram";
        @zettajoule_per_kilogram: prefix!(zetta); "ZJ/kg", "zettajoule per kilogram",
            "zettajoules per kilogram";
        @exajoule_per_kilogram: prefix!(exa); "EJ/kg", "exajoule per kilogram",
            "exajoules per kilogram";
        @petajoule_per_kilogram: prefix!(peta); "PJ/kg", "petajoule per kilogram",
            "petajoules per kilogram";
        @terajoule_per_kilogram: prefix!(tera); "TJ/kg", "terajoule per kilogram",
            "terajoules per kilogram";
        @gigajoule_per_kilogram: prefix!(giga); "GJ/kg", "gigajoule per kilogram",
            "gigajoules per kilogram";
        @megajoule_per_kilogram: prefix!(mega); "MJ/kg", "megajoule per kilogram",
            "megajoules per kilogram";
        @kilojoule_per_kilogram: prefix!(kilo); "kJ/kg", "kilojoule per kilogram",
            "kilojoules per kilogram";
        @hectojoule_per_kilogram: prefix!(hecto); "hJ/kg", "hectojoule per kilogram",
            "hectojoules per kilogram";
        @decajoule_per_kilogram: prefix!(deca); "daJ/kg", "decajoule per kilogram",
            "decajoules per kilogram";
        /// Derived unit of available energy.
        @joule_per_kilogram: prefix!(none); "J/kg", "joule per kilogram", "joules per kilogram";
        @decijoule_per_kilogram: prefix!(deci); "dJ/kg", "decijoule per kilogram",
            "decijoules per kilogram";
        @centijoule_per_kilogram: prefix!(centi); "cJ/kg", "centijoule per kilogram",
            "centijoules per kilogram";
        @millijoule_per_kilogram: prefix!(milli); "mJ/kg", "millijoule per kilogram",
            "millijoules per kilogram";
        @microjoule_per_kilogram: prefix!(micro); "µJ/kg", "microjoule per kilogram",
            "microjoules per kilogram";
        @nanojoule_per_kilogram: prefix!(nano); "nJ/kg", "nanojoule per kilogram",
            "nanojoules per kilogram";
        @picojoule_per_kilogram: prefix!(pico); "pJ/kg", "picojoule per kilogram",
            "picojoules per kilogram";
        @femtojoule_per_kilogram: prefix!(femto); "fJ/kg", "femtojoule per kilogram",
            "femtojoules per kilogram";
        @attojoule_per_kilogram: prefix!(atto); "aJ/kg", "attojoule per kilogram",
            "attojoules per kilogram";
        @zeptojoule_per_kilogram: prefix!(zepto); "zJ/kg", "zeptojoule per kilogram",
            "zeptojoules per kilogram";
        @yoctojoule_per_kilogram: prefix!(yocto); "yJ/kg", "yoctojoule per kilogram",
            "yoctojoules per kilogram";

        @joule_per_zeptogram: prefix!(yotta); "J/zg", "joule per zeptogram",
            "joules per zeptogram";
        @joule_per_attogram: prefix!(zetta); "J/ag", "joule per attogram", "joules per attogram";
        @joule_per_femtogram: prefix!(exa); "J/fg", "joule per femtogram", "joules per femtogram";
        @joule_per_picogram: prefix!(peta); "J/pg", "joule per picogram", "joules per picogram";
        @joule_per_nanogram: prefix!(tera); "J/ng", "joule per nanogram", "joules per nanogram";
        @joule_per_microgram: prefix!(giga); "J/µg", "joule per microgram", "joules per microgram";
        @joule_per_milligram: prefix!(mega); "J/mg", "joule per milligram", "joules per milligram";
        @joule_per_gram: prefix!(kilo); "J/g", "joule per gram", "joules per gram";
        @joule_per_megagram: prefix!(milli); "J/Mg", "joule per megagram", "joules per megagram";
        @joule_per_gigagram: prefix!(micro); "J/Gg", "joule per gigagram", "joules per gigagram";
        @joule_per_teragram: prefix!(nano); "J/Tg", "joule per teragram", "joules per teragram";
        @joule_per_petagram: prefix!(pico); "J/Pg", "joule per petagram", "joules per petagram";
        @joule_per_exagram: prefix!(femto); "J/Eg", "joule per exagram", "joules per exagram";
        @joule_per_zettagram: prefix!(atto); "J/Zg", "joule per zettagram", "joules per zettagram";
        @joule_per_yottagram: prefix!(zepto); "J/Yg", "joule per yottagram",
            "joules per yottagram";

        @btu_it_per_pound: 2.326_000_171_078_704_E3; "Btu (IT)/lb",
            "British thermal unit (IT) per pound", "British thermal units (IT) per pound";
        @btu_per_pound: 2.324_443_707_610_621_E3; "Btu/lb", "British thermal unit per pound",
            "British thermal units per pound";
        @calorie_it_per_gram: 4.186_8_E3; "cal (IT)/lb", "calorie (IT) per gram",
            "calories (IT) per gram";
        @calorie_per_gram: 4.184_E3; "cal/lb", "calorie per gram", "calories per gram";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::mass as m;
        use si::energy as e;
        use si::available_energy as v;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: AvailableEnergy<V> = Energy::new::<e::joule>(V::one())
                / Mass::new::<m::kilogram>(V::one());
        }

        #[test]
        fn check_units() {
            test::<e::yottajoule, m::kilogram, v::yottajoule_per_kilogram>();
            test::<e::exajoule, m::kilogram, v::exajoule_per_kilogram>();
            test::<e::terajoule, m::kilogram, v::terajoule_per_kilogram>();
            test::<e::megajoule, m::kilogram, v::megajoule_per_kilogram>();
            test::<e::joule, m::kilogram, v::joule_per_kilogram>();
            test::<e::microjoule, m::kilogram, v::microjoule_per_kilogram>();
            test::<e::picojoule, m::kilogram, v::picojoule_per_kilogram>();
            test::<e::attojoule, m::kilogram, v::attojoule_per_kilogram>();
            test::<e::yoctojoule, m::kilogram, v::yoctojoule_per_kilogram>();

            test::<e::joule, m::zeptogram, v::joule_per_zeptogram>();
            test::<e::joule, m::femtogram, v::joule_per_femtogram>();
            test::<e::joule, m::nanogram, v::joule_per_nanogram>();
            test::<e::joule, m::milligram, v::joule_per_milligram>();
            test::<e::joule, m::gigagram, v::joule_per_gigagram>();
            test::<e::joule, m::petagram, v::joule_per_petagram>();
            test::<e::joule, m::zettagram, v::joule_per_zettagram>();

            test::<e::btu_it, m::pound, v::btu_it_per_pound>();
            test::<e::btu, m::pound, v::btu_per_pound>();
            test::<e::calorie_it, m::gram, v::calorie_it_per_gram>();
            test::<e::calorie, m::gram, v::calorie_per_gram>();

            fn test<E: e::Conversion<V>, M: m::Conversion<V>, A: v::Conversion<V>>() {
                Test::assert_approx_eq(&AvailableEnergy::new::<A>(V::one()),
                    &(Energy::new::<E>(V::one()) / Mass::new::<M>(V::one())));
            }
        }
    }
}
