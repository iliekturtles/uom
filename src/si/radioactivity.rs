//! Radioactivity (base unit becquerel, s⁻¹).

quantity! {
    /// Radioactivity (base unit becquerel, s⁻¹).
    quantity: Radioactivity; "radioactivity";
    /// Dimension of radioactivity, T⁻¹ (base unit becquerel, s⁻¹).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @yottabecquerel: prefix!(yotta); "YBq", "yottabecquerel", "yottabecquerels";
        @zettabecquerel: prefix!(zetta); "ZBq", "zettabecquerel", "zettabecquerels";
        @exabecquerel: prefix!(exa); "EBq", "exabecquerel", "exabecquerels";
        @petabecquerel: prefix!(peta); "PBq", "petabecquerel", "petabecquerels";
        @terabecquerel: prefix!(tera); "TBq", "terabecquerel", "terabecquerels";
        @gigabecquerel: prefix!(giga); "GBq", "gigabecquerel", "gigabecquerels";
        @megabecquerel: prefix!(mega); "MBq", "megabecquerel", "megabecquerels";
        @kilobecquerel: prefix!(kilo); "kBq", "kilobecquerel", "kilobecquerels";
        @hectobecquerel: prefix!(hecto); "hBq", "hectobecquerel", "hectobecquerels";
        @decabecquerel: prefix!(deca); "daBq", "decabecquerel", "decabecquerels";
        /// The becquerel is one decay per second.
        @becquerel: prefix!(none); "Bq", "becquerel", "becquerels";
        @millibecquerel:  prefix!(milli); "mBq", "millibecquerel", "millibecquerels";
        @microbecquerel:  prefix!(micro); "µBq", "microbecquerel", "microbecquerels";
        @nanobecquerel: prefix!(nano); "nBq", "nanobecquerel", "nanobecquerels";

        @gigacurie: prefix!(giga) * 3.7_E10; "GCi", "gigacurie", "gigacuries";
        @megacurie: prefix!(mega) * 3.7_E10; "MCi", "megacurie", "megacuries";
        @kilocurie: prefix!(kilo) * 3.7_E10; "kCi", "kilocurie", "kilocuries";
        @curie: 3.7_E10; "Ci", "curie", "curies";
        @millicurie: prefix!(milli) * 3.7_E10; "mCi", "millicurie", "millicuries";
        @microcurie: prefix!(micro) * 3.7_E10; "µCi", "microcurie", "microcuries";
        @nanocurie: prefix!(nano) * 3.7_E10; "nCi", "nanocurie", "nanocuries";

        @disintegrations_per_minute: 1.0 / 6.0_E1; "dpm", "disintegration per minute",
            "disintegrations per minute";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::{FromPrimitive, One};
        use crate::si::radioactivity as rad;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: Time<V> = (V::one() / Radioactivity::new::<rad::becquerel>(V::one())).into();
            let _: Radioactivity<V> = (V::one() / Time::new::<t::second>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<t::second, rad::becquerel>();
            test::<t::decisecond, rad::decabecquerel>();
            test::<t::centisecond, rad::hectobecquerel>();
            test::<t::millisecond, rad::kilobecquerel>();
            test::<t::microsecond, rad::megabecquerel>();
            test::<t::nanosecond, rad::gigabecquerel>();
            test::<t::picosecond, rad::terabecquerel>();
            test::<t::femtosecond, rad::petabecquerel>();
            test::<t::attosecond, rad::exabecquerel>();
            test::<t::zeptosecond, rad::zettabecquerel>();
            test::<t::yoctosecond, rad::yottabecquerel>();

            fn test<T: t::Conversion<V>, RAD: rad::Conversion<V>>() {
                Test::assert_approx_eq(&(V::one() / Time::new::<T>(V::one())),
                    &Radioactivity::new::<RAD>(V::one()).into());
                Test::assert_approx_eq(&Time::new::<T>(V::one()),
                    &(V::one() / Radioactivity::new::<RAD>(V::one())).into());
            }
        }

        #[test]
        fn check_curie() {
            test::<rad::gigabecquerel, rad::gigacurie>();
            test::<rad::megabecquerel, rad::megacurie>();
            test::<rad::kilobecquerel, rad::kilocurie>();
            test::<rad::becquerel, rad::curie>();
            test::<rad::millibecquerel, rad::millicurie>();
            test::<rad::microbecquerel, rad::microcurie>();
            test::<rad::nanobecquerel, rad::nanocurie>();

            fn test<RadBq: rad::Conversion<V>, RadCi: rad::Conversion<V>>() {
                Test::assert_approx_eq(
                    &(V::one() * V::from_f64(3.7_E10).unwrap()
                        * Radioactivity::new::<RadBq>(V::one())),
                    &Radioactivity::new::<RadCi>(V::one()));
            }
        }

        #[test]
        fn check_dpm() {
            test::<rad::becquerel, rad::disintegrations_per_minute>();

            fn test<RadBq: rad::Conversion<V>, RadCi: rad::Conversion<V>>() {
                Test::assert_approx_eq(&(V::one() / V::from_f64(6_E1).unwrap()
                        * Radioactivity::new::<RadBq>(V::one())),
                    &Radioactivity::new::<RadCi>(V::one()));
            }
        }
    }
}
