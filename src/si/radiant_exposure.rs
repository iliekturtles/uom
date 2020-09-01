//! Radiant exposure (base unit joule per square meter, kg · s⁻²).

quantity! {
    /// Radiant exposure (base unit joule per square meter, kg · s⁻²).
    quantity: RadiantExposure; "radiant exposure";
    /// Dimension of radiant exposure, MT⁻² (base unit joule per square meter, kg · s⁻²).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottajoule_per_square_meter: prefix!(yotta); "YJ/m²",
            "yottajoule per square meter", "yottajoules per square meter";
        @zettajoule_per_square_meter: prefix!(zetta); "ZJ/m²",
            "zettajoule per square meter", "zettajoules per square meter";
        @exajoule_per_square_meter: prefix!(exa); "EJ/m²",
            "exajoule per square meter", "exajoules per square meter";
        @petajoule_per_square_meter: prefix!(peta); "PJ/m²",
            "petajoule per square meter", "petajoules per square meter";
        @terajoule_per_square_meter: prefix!(tera); "TJ/m²",
            "terajoule per square meter", "terajoules per square meter";
        @gigajoule_per_square_meter: prefix!(giga); "GJ/m²",
            "gigajoule per square meter", "gigajoules per square meter";
        @megajoule_per_square_meter: prefix!(mega); "MJ/m²",
            "megajoule per square meter", "megajoules per square meter";
        /// Derived unit of density.
        @kilojoule_per_square_meter: prefix!(kilo); "kJ/m²",
            "kilojoule per square meter", "kilojoules per square meter";
        @hectojoule_per_square_meter: prefix!(hecto); "hJ/m²",
            "hectojoule per square meter", "hectojoules per square meter";
        @decajoule_per_square_meter: prefix!(deca); "daJ/m²",
            "decajoule per square meter", "decajoules per square meter";
        @joule_per_square_meter: prefix!(none); "J/m²",
            "joule per square meter", "joules per square meter";
        @decijoule_per_square_meter: prefix!(deci); "dJ/m²",
            "decijoule per square meter", "decijoules per square meter";
        @centijoule_per_square_meter: prefix!(centi); "cJ/m²",
            "centijoule per square meter", "centijoules per square meter";
        @millijoule_per_square_meter: prefix!(milli); "mJ/m²",
            "millijoule per square meter", "millijoules per square meter";
        @microjoule_per_square_meter: prefix!(micro); "µJ/m²",
            "microjoule per square meter", "microjoules per square meter";
        @nanojoule_per_square_meter: prefix!(nano); "nJ/m²",
            "nanojoule per square meter", "nanojoules per square meter";
        @picojoule_per_square_meter: prefix!(pico); "pJ/m²",
            "picojoule per square meter", "picojoules per square meter";
        @femtojoule_per_square_meter: prefix!(femto); "fJ/m²",
            "femtojoule per square meter", "femtojoules per square meter";
        @attojoule_per_square_meter: prefix!(atto); "aJ/m²",
            "attojoule per square meter", "attojoules per square meter";
        @zeptojoule_per_square_meter: prefix!(zepto); "zJ/m²",
            "zeptojoule per square meter", "zeptojoules per square meter";
        @yoctojoule_per_square_meter: prefix!(yocto); "yJ/m²",
            "yoctojoule per square meter", "yoctojoules per square meter";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::energy as e;
        use si::length as l;
        use si::radiant_exposure as re;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: RadiantExposure<V> =
                Energy::new::<e::kilojoule>(V::one())
                / (Length::new::<l::meter>(V::one())
                   * Length::new::<l::meter>(V::one()));
        }

        #[test]
        fn check_units() {
            test::<e::yottajoule, re::yottajoule_per_square_meter>();
            test::<e::zettajoule, re::zettajoule_per_square_meter>();
            test::<e::exajoule, re::exajoule_per_square_meter>();
            test::<e::petajoule, re::petajoule_per_square_meter>();
            test::<e::terajoule, re::terajoule_per_square_meter>();
            test::<e::gigajoule, re::gigajoule_per_square_meter>();
            test::<e::megajoule, re::megajoule_per_square_meter>();
            test::<e::kilojoule, re::kilojoule_per_square_meter>();
            test::<e::hectojoule, re::hectojoule_per_square_meter>();
            test::<e::decajoule, re::decajoule_per_square_meter>();
            test::<e::joule, re::joule_per_square_meter>();
            test::<e::decijoule, re::decijoule_per_square_meter>();
            test::<e::centijoule, re::centijoule_per_square_meter>();
            test::<e::millijoule, re::millijoule_per_square_meter>();
            test::<e::microjoule, re::microjoule_per_square_meter>();
            test::<e::nanojoule, re::nanojoule_per_square_meter>();
            test::<e::picojoule, re::picojoule_per_square_meter>();
            test::<e::femtojoule, re::femtojoule_per_square_meter>();
            test::<e::attojoule, re::attojoule_per_square_meter>();
            test::<e::zeptojoule, re::zeptojoule_per_square_meter>();
            test::<e::yoctojoule, re::yoctojoule_per_square_meter>();

            fn test<E: e::Conversion<V>, RE: re::Conversion<V>>() {
                Test::assert_approx_eq(
                    &RadiantExposure::new::<RE>(V::one()),
                    &(Energy::new::<E>(V::one())
                      / (Length::new::<l::meter>(V::one())
                         * Length::new::<l::meter>(V::one()))));
            }
        }
    }
}
