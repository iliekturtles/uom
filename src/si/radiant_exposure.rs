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
        // square meter denominator
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
        @kilojoule_per_square_meter: prefix!(kilo); "kJ/m²",
            "kilojoule per square meter", "kilojoules per square meter";
        @hectojoule_per_square_meter: prefix!(hecto); "hJ/m²",
            "hectojoule per square meter", "hectojoules per square meter";
        @decajoule_per_square_meter: prefix!(deca); "daJ/m²",
            "decajoule per square meter", "decajoules per square meter";
        /// Derived unit of radiant exposure.
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

        // square centimeter denominator
        @gigajoule_per_square_centimeter: 1.0E13; "GJ/cm²",
            "gigajoule per square centimeter", "gigajoules per square centimeter";
        @megajoule_per_square_centimeter: 1.0E10; "MJ/cm²",
            "megajoule per square centimeter", "megajoules per square centimeter";
        @kilojoule_per_square_centimeter: 1.0E7; "kJ/cm²",
            "kilojoule per square centimeter", "kilojoules per square centimeter";
        @hectojoule_per_square_centimeter: 1.0E6; "hJ/cm²",
            "hectojoule per square centimeter", "hectojoules per square centimeter";
        @decajoule_per_square_centimeter: 1.0E5; "daJ/cm²",
            "decajoule per square centimeter", "decajoules per square centimeter";
        @joule_per_square_centimeter: 1.0E4; "J/cm²",
            "joule per square centimeter", "joules per square centimeter";
        @decijoule_per_square_centimeter: 1.0E3; "dJ/cm²",
            "decijoule per square centimeter", "decijoules per square centimeter";
        @centijoule_per_square_centimeter: 1.0E2; "cJ/cm²",
            "centijoule per square centimeter", "centijoules per square centimeter";
        @millijoule_per_square_centimeter: 1.0E1; "mJ/cm²",
            "millijoule per square centimeter", "millijoules per square centimeter";
        @microjoule_per_square_centimeter: 1.0E-2; "µJ/cm²",
            "microjoule per square centimeter", "microjoules per square centimeter";
        @nanojoule_per_square_centimeter: 1.0E-5; "nJ/cm²",
            "nanojoule per square centimeter", "nanojoules per square centimeter";

        // square millimeter denominator
        @gigajoule_per_square_millimeter: 1.0E15; "GJ/mm²",
            "gigajoule per square millimeter", "gigajoules per square millimeter";
        @megajoule_per_square_millimeter: 1.0E12; "MJ/mm²",
            "megajoule per square millimeter", "megajoules per square millimeter";
        @kilojoule_per_square_millimeter: 1.0E9; "kJ/mm²",
            "kilojoule per square millimeter", "kilojoules per square millimeter";
        @hectojoule_per_square_millimeter: 1.0E8; "hJ/mm²",
            "hectojoule per square millimeter", "hectojoules per square millimeter";
        @decajoule_per_square_millimeter: 1.0E7; "daJ/mm²",
            "decajoule per square millimeter", "decajoules per square millimeter";
        @joule_per_square_millimeter: 1.0E6; "J/mm²",
            "joule per square millimeter", "joules per square millimeter";
        @decijoule_per_square_millimeter: 1.0E5; "dJ/mm²",
            "decijoule per square millimeter", "decijoules per square millimeter";
        @centijoule_per_square_millimeter: 1.0E4; "cJ/mm²",
            "centijoule per square millimeter", "centijoules per square millimeter";
        @millijoule_per_square_millimeter: 1.0E3; "mJ/mm²",
            "millijoule per square millimeter", "millijoules per square millimeter";
        @microjoule_per_square_millimeter: 1.0E0; "µJ/mm²",
            "microjoule per square millimeter", "microjoules per square millimeter";
        @nanojoule_per_square_millimeter: 1.0E-3; "nJ/mm²",
            "nanojoule per square millimeter", "nanojoules per square millimeter";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::area as a;
        use crate::si::energy as e;
        use crate::si::length as l;
        use crate::si::quantities::*;
        use crate::si::radiant_exposure as re;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: RadiantExposure<V> =
                Energy::new::<e::kilojoule>(V::one())
                / (Length::new::<l::meter>(V::one())
                   * Length::new::<l::meter>(V::one()));
        }

        #[test]
        fn check_units() {
            test::<e::yottajoule, a::square_meter, re::yottajoule_per_square_meter>();
            test::<e::zettajoule, a::square_meter, re::zettajoule_per_square_meter>();
            test::<e::exajoule, a::square_meter, re::exajoule_per_square_meter>();
            test::<e::petajoule, a::square_meter, re::petajoule_per_square_meter>();
            test::<e::terajoule, a::square_meter, re::terajoule_per_square_meter>();
            test::<e::gigajoule, a::square_meter, re::gigajoule_per_square_meter>();
            test::<e::megajoule, a::square_meter, re::megajoule_per_square_meter>();
            test::<e::kilojoule, a::square_meter, re::kilojoule_per_square_meter>();
            test::<e::hectojoule, a::square_meter, re::hectojoule_per_square_meter>();
            test::<e::decajoule, a::square_meter, re::decajoule_per_square_meter>();
            test::<e::joule, a::square_meter, re::joule_per_square_meter>();
            test::<e::decijoule, a::square_meter, re::decijoule_per_square_meter>();
            test::<e::centijoule, a::square_meter, re::centijoule_per_square_meter>();
            test::<e::millijoule, a::square_meter, re::millijoule_per_square_meter>();
            test::<e::microjoule, a::square_meter, re::microjoule_per_square_meter>();
            test::<e::nanojoule, a::square_meter, re::nanojoule_per_square_meter>();
            test::<e::picojoule, a::square_meter, re::picojoule_per_square_meter>();
            test::<e::femtojoule, a::square_meter, re::femtojoule_per_square_meter>();
            test::<e::attojoule, a::square_meter, re::attojoule_per_square_meter>();
            test::<e::zeptojoule, a::square_meter, re::zeptojoule_per_square_meter>();
            test::<e::yoctojoule, a::square_meter, re::yoctojoule_per_square_meter>();

            test::<e::gigajoule, a::square_centimeter, re::gigajoule_per_square_centimeter>();
            test::<e::megajoule, a::square_centimeter, re::megajoule_per_square_centimeter>();
            test::<e::kilojoule, a::square_centimeter, re::kilojoule_per_square_centimeter>();
            test::<e::hectojoule, a::square_centimeter, re::hectojoule_per_square_centimeter>();
            test::<e::decajoule, a::square_centimeter, re::decajoule_per_square_centimeter>();
            test::<e::joule, a::square_centimeter, re::joule_per_square_centimeter>();
            test::<e::decijoule, a::square_centimeter, re::decijoule_per_square_centimeter>();
            test::<e::centijoule, a::square_centimeter, re::centijoule_per_square_centimeter>();
            test::<e::millijoule, a::square_centimeter, re::millijoule_per_square_centimeter>();
            test::<e::microjoule, a::square_centimeter, re::microjoule_per_square_centimeter>();
            test::<e::nanojoule, a::square_centimeter, re::nanojoule_per_square_centimeter>();

            test::<e::gigajoule, a::square_millimeter, re::gigajoule_per_square_millimeter>();
            test::<e::megajoule, a::square_millimeter, re::megajoule_per_square_millimeter>();
            test::<e::kilojoule, a::square_millimeter, re::kilojoule_per_square_millimeter>();
            test::<e::hectojoule, a::square_millimeter, re::hectojoule_per_square_millimeter>();
            test::<e::decajoule, a::square_millimeter, re::decajoule_per_square_millimeter>();
            test::<e::joule, a::square_millimeter, re::joule_per_square_millimeter>();
            test::<e::decijoule, a::square_millimeter, re::decijoule_per_square_millimeter>();
            test::<e::centijoule, a::square_millimeter, re::centijoule_per_square_millimeter>();
            test::<e::millijoule, a::square_millimeter, re::millijoule_per_square_millimeter>();
            test::<e::microjoule, a::square_millimeter, re::microjoule_per_square_millimeter>();
            test::<e::nanojoule, a::square_millimeter, re::nanojoule_per_square_millimeter>();

            fn test<E: e::Conversion<V>, A: a::Conversion<V>, RE: re::Conversion<V>>() {
                Test::assert_approx_eq(
                    &RadiantExposure::new::<RE>(V::one()),
                    &(Energy::new::<E>(V::one()) / Area::new::<A>(V::one())));
            }
        }
    }
}
