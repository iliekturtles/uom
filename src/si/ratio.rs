//! Ratio (dimensionless).

quantity! {
    /// Ratio (dimensionless).
    quantity: Ratio; "ratio";
    /// Ratio dimension (dimensionless).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @ratio: 1.0; "", "", "";
        @part_per_hundred: 1.0_E-2; "parts per hundred", "part per hundred", "parts per hundred";
        @percent: 1.0_E-2; "%", "percent", "percent";
        @part_per_thousand: 1.0_E-3; "parts per thousand", "part per thousand",
            "parts per thousand";
        @per_mille: 1.0_E-3; "‰", "per mille", "per mille";
        @part_per_ten_thousand: 1.0_E-4; "parts per ten thousand", "part per then thousand",
            "parts per ten thousand"; // ‱, doesn't display properly.
        @basis_point: 1.0_E-4; "bp", "basis point", "basis points";
        @part_per_million: 1.0_E-6; "ppm", "part per million", "parts per million";
        @part_per_billion: 1.0_E-9; "ppb", "part per billion", "parts per billion";
        @part_per_trillion: 1.0_E-12; "ppt", "part per trillion", "parts per trillion";
        @part_per_quadrillion: 1.0_E-15; "ppq", "part per quadrillion", "parts per quadrillion";
    }
}

mod convert {
    use super::*;

    impl<U, V> ::lib::convert::From<V> for Ratio<U, V>
    where
        U: ::si::Units<V> + ?Sized,
        V: ::num::Num + ::Conversion<V>,
    {
        fn from(t: V) -> Self {
            Ratio {
                dimension: ::lib::marker::PhantomData,
                units: ::lib::marker::PhantomData,
                value: t,
            }
        }
    }

    storage_types! {
        use super::*;

        impl<U> ::lib::convert::From<Ratio<U, V>> for V
        where
            U: ::si::Units<V> + ?Sized,
            V: ::num::Num + ::Conversion<V>,
        {
            fn from(t: Ratio<U, V>) -> Self {
                t.value
            }
        }
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::{FromPrimitive, One};
        use si::quantities::*;
        use si::ratio as r;
        use tests::Test;

        #[test]
        fn from() {
            let r1: Ratio<V> = Ratio::<V>::from(V::one());
            let r2: Ratio<V> = V::one().into();
            let _: V = V::from(r1);
            let _: V = r2.into();
        }

        #[test]
        fn check_units() {
            Test::assert_eq(&Ratio::new::<r::ratio>(V::one() / V::from_f64(1.0_E2).unwrap()),
                &Ratio::new::<r::part_per_hundred>(V::one()));
            Test::assert_eq(&Ratio::new::<r::ratio>(V::one() / V::from_f64(1.0_E2).unwrap()),
                &Ratio::new::<r::percent>(V::one()));
            Test::assert_eq(&Ratio::new::<r::ratio>(V::one() / V::from_f64(1.0_E3).unwrap()),
                &Ratio::new::<r::part_per_thousand>(V::one()));
            Test::assert_eq(&Ratio::new::<r::ratio>(V::one() / V::from_f64(1.0_E3).unwrap()),
                &Ratio::new::<r::per_mille>(V::one()));
            Test::assert_eq(&Ratio::new::<r::ratio>(V::one() / V::from_f64(1.0_E4).unwrap()),
                &Ratio::new::<r::part_per_ten_thousand>(V::one()));
            Test::assert_eq(&Ratio::new::<r::ratio>(V::one() / V::from_f64(1.0_E4).unwrap()),
                &Ratio::new::<r::basis_point>(V::one()));
            Test::assert_eq(&Ratio::new::<r::ratio>(V::one() / V::from_f64(1.0_E6).unwrap()),
                &Ratio::new::<r::part_per_million>(V::one()));
            Test::assert_eq(&Ratio::new::<r::ratio>(V::one() / V::from_f64(1.0_E9).unwrap()),
                &Ratio::new::<r::part_per_billion>(V::one()));
            Test::assert_eq(&Ratio::new::<r::ratio>(V::one()
                    / V::from_f64(1.0_E12).unwrap()),
                &Ratio::new::<r::part_per_trillion>(V::one()));
            Test::assert_eq(&Ratio::new::<r::ratio>(V::one()
                    / V::from_f64(1.0_E15).unwrap()),
                &Ratio::new::<r::part_per_quadrillion>(V::one()));
        }
    }
}
