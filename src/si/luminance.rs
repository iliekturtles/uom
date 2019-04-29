//! Luminance (base unit candela per square meter, cd · m⁻²).

quantity! {
    /// Luminance (base unit candela per square meter, cd · m⁻²).
    quantity: Luminance; "luminance";
    /// Dimension of luminance, L⁻²J (base unit candela per square meter, cd · m⁻²).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        P1>;    // luminous intensity
    units {
        @yottacandela_per_square_meter: prefix!(yotta); "Ycd/m²", "yottacandela per square meter",
            "yottacandelas per square meter";
        @zettacandela_per_square_meter: prefix!(zetta); "Zcd/m²", "zettacandela per square meter",
            "zettacandelas per square meter";
        @exacandela_per_square_meter: prefix!(exa); "Ecd/m²", "exacandela per square meter",
            "exacandelas per square meter";
        @petacandela_per_square_meter: prefix!(peta); "Pcd/m²", "petacandela per square meter",
            "petacandelas per square meter";
        @teracandela_per_square_meter: prefix!(tera); "Tcd/m²", "teracandela per square meter",
            "teracandelas per square meter";
        @gigacandela_per_square_meter: prefix!(giga); "Gcd/m²", "gigacandela per square meter",
            "gigacandelas per square meter";
        @megacandela_per_square_meter: prefix!(mega); "Mcd/m²", "megacandela per square meter",
            "megacandelas per square meter";
        @kilocandela_per_square_meter: prefix!(kilo); "kcd/m²", "kilocandela per square meter",
            "kilocandelas per square meter";
        @hectocandela_per_square_meter: prefix!(hecto); "hcd/m²", "hectocandela per square meter",
            "hectocandelas per square meter";
        @decacandela_per_square_meter: prefix!(deca); "dacd/m²", "decacandela per square meter",
            "decacandelas per square meter";
        /// Derived unit of luminance.
        @candela_per_square_meter: prefix!(none); "cd/m²", "candela per square meter",
            "candelas per square meter";
        @decicandela_per_square_meter: prefix!(deci); "dcd/m²", "decicandela per square meter",
            "decicandelas per square meter";
        @centicandela_per_square_meter: prefix!(centi); "ccd/m²", "centicandela per square meter",
            "centicandelas per square meter";
        @millicandela_per_square_meter: prefix!(milli); "mcd/m²", "millicandela per square meter",
            "millicandelas per square meter";
        @microcandela_per_square_meter: prefix!(micro); "µcd/m²", "microcandela per square meter",
            "microcandelas per square meter";
        @nanocandela_per_square_meter: prefix!(nano); "ncd/m²", "nanocandela per square meter",
            "nanocandelas per square meter";
        @picocandela_per_square_meter: prefix!(pico); "pcd/m²", "picocandela per square meter",
            "picocandelas per square meter";
        @femtocandela_per_square_meter: prefix!(femto); "fcd/m²", "femtocandela per square meter",
            "femtocandelas per square meter";
        @attocandela_per_square_meter: prefix!(atto); "acd/m²", "attocandela per square meter",
            "attocandelas per square meter";
        @zeptocandela_per_square_meter: prefix!(zepto); "zcd/m²", "zeptocandela per square meter",
            "zeptocandelas per square meter";
        @yoctocandela_per_square_meter: prefix!(yocto); "ycd/m²", "yoctocandela per square meter",
            "yoctocandelas per square meter";

        @candela_per_square_picometer: prefix!(yotta); "cd/pm²", "candela per square picometer",
            "candelas per square picometer";
        @candela_per_square_nanometer: prefix!(exa); "cd/nm²", "candela per square nanometer",
            "candelas per square nanometer";
        @candela_per_square_micrometer: prefix!(tera); "cd/µm²", "candela per square micrometer",
            "candelas per square micrometer";
        @candela_per_square_millimeter: prefix!(mega); "cd/mm²", "candela per square millimeter",
            "candelas per square millimeter";
        @candela_per_square_centimeter: 1.0_E4; "cd/cm²", "candela per square centimeter",
            "candelas per square centimeter";
        @candela_per_square_kilometer: prefix!(micro); "cd/km²", "candela per square kilometer",
            "candelas per square kilometer";
        @candela_per_square_megameter: prefix!(pico); "cd/Mm²", "candela per square megameter",
            "candelas per square megameter";
        @candela_per_square_gigameter: prefix!(atto); "cd/Gm²", "candela per square gigameter",
            "candelas per square gigameter";
        @candela_per_square_terameter: prefix!(yocto); "cd/Tm²", "candela per square terameter",
            "candelas per square terameter";

        @candela_per_square_inch: 1.550_003_100_006_200_2_E3; "cd/in²", "candela per square inch",
            "candelas per square inch";
        @footlambert: 3.426_259_099_635_390_5_E0; "fl", "footlambert", "footlamberts";
        @lambert: 3.183_098_861_837_906_7_E3; "la", "lambert", "lamberts";
        @stilb: 1.0_E4; "sb", "stilb", "stilbs";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::luminous_intensity as i;
        use si::area as a;
        use si::luminance as l;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Luminance<V> = LuminousIntensity::new::<i::candela>(V::one())
                / Area::new::<a::square_meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<i::yottacandela, a::square_meter, l::yottacandela_per_square_meter>();
            test::<i::zettacandela, a::square_meter, l::zettacandela_per_square_meter>();
            test::<i::exacandela, a::square_meter, l::exacandela_per_square_meter>();
            test::<i::petacandela, a::square_meter, l::petacandela_per_square_meter>();
            test::<i::teracandela, a::square_meter, l::teracandela_per_square_meter>();
            test::<i::gigacandela, a::square_meter, l::gigacandela_per_square_meter>();
            test::<i::megacandela, a::square_meter, l::megacandela_per_square_meter>();
            test::<i::kilocandela, a::square_meter, l::kilocandela_per_square_meter>();
            test::<i::hectocandela, a::square_meter, l::hectocandela_per_square_meter>();
            test::<i::decacandela, a::square_meter, l::decacandela_per_square_meter>();
            test::<i::candela, a::square_meter, l::candela_per_square_meter>();
            test::<i::decicandela, a::square_meter, l::decicandela_per_square_meter>();
            test::<i::centicandela, a::square_meter, l::centicandela_per_square_meter>();
            test::<i::millicandela, a::square_meter, l::millicandela_per_square_meter>();
            test::<i::microcandela, a::square_meter, l::microcandela_per_square_meter>();
            test::<i::nanocandela, a::square_meter, l::nanocandela_per_square_meter>();
            test::<i::picocandela, a::square_meter, l::picocandela_per_square_meter>();
            test::<i::femtocandela, a::square_meter, l::femtocandela_per_square_meter>();
            test::<i::attocandela, a::square_meter, l::attocandela_per_square_meter>();
            test::<i::zeptocandela, a::square_meter, l::zeptocandela_per_square_meter>();
            test::<i::yoctocandela, a::square_meter, l::yoctocandela_per_square_meter>();

            test::<i::candela, a::square_picometer, l::candela_per_square_picometer>();
            test::<i::candela, a::square_nanometer, l::candela_per_square_nanometer>();
            test::<i::candela, a::square_micrometer, l::candela_per_square_micrometer>();
            test::<i::candela, a::square_millimeter, l::candela_per_square_millimeter>();
            test::<i::candela, a::square_centimeter, l::candela_per_square_centimeter>();
            test::<i::candela, a::square_kilometer, l::candela_per_square_kilometer>();
            test::<i::candela, a::square_megameter, l::candela_per_square_megameter>();
            test::<i::candela, a::square_gigameter, l::candela_per_square_gigameter>();
            test::<i::candela, a::square_terameter, l::candela_per_square_terameter>();

            test::<i::candela, a::square_inch, l::candela_per_square_inch>();

            fn test<I: i::Conversion<V>, A: a::Conversion<V>, L: l::Conversion<V>>() {
                Test::assert_approx_eq(&Luminance::new::<L>(V::one()),
                    &(LuminousIntensity::new::<I>(V::one()) / Area::new::<A>(V::one())));
            }
        }
    }
}
