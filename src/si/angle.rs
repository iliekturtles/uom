//! Angle (dimensionless quantity).

quantity! {
    /// Angle (dimensionless quantity).
    quantity: Angle; "angle";
    /// Dimension of angle, 1 (dimensionless).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: ::si::marker::AngleKind;
    units {
        /// SI derived unit of angle. It is the angle subtended at the center of a circle by an
        /// arc that is equal in length to the radius of the circle.
        @radian: 1.0_E0; "rad", "radian", "radians";
        @revolution: 6.283_185_307_179_586_E0; "r", "revolution", "revolutions";
        @degree: 1.745_329_251_994_329_5_E-2; "°", "degree", "degrees";
        @gon: 1.570_796_326_794_896_7_E-2; "gon", "gon", "gons";
        @mil: 9.817_477_E-4; "mil", "mil", "mils";
        @minute: 2.908_882_086_657_216_E-4; "′", "minute", "minutes";
        @second: 4.848_136_811_095_36_E-6; "″", "second", "seconds";
    }
}

mod convert {
    use super::*;

    impl<U, V> ::lib::convert::From<V> for Angle<U, V>
    where
        U: ::si::Units<V> + ?Sized,
        V: ::num::Num + ::Conversion<V>,
    {
        fn from(t: V) -> Self {
            Angle {
                dimension: ::lib::marker::PhantomData,
                units: ::lib::marker::PhantomData,
                value: t,
            }
        }
    }

    storage_types! {
        use super::*;

        impl<U> ::lib::convert::From<Angle<U, V>> for V
        where
            U: ::si::Units<V> + ?Sized,
            V: ::num::Num + ::Conversion<V>,
        {
            fn from(t: Angle<U, V>) -> Self {
                t.value
            }
        }
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use ::lib::f64::consts::PI;
        use num::One;
        use num_traits::FromPrimitive;
        use si::angle as a;
        use si::quantities::*;
        use tests::Test;

        #[test]
        fn from() {
            let r1: Angle<V> = Angle::<V>::from(V::one());
            let r2: Angle<V> = V::one().into();
            let _: V = V::from(r1);
            let _: V = r2.into();
        }

        #[test]
        fn check_units() {
            Test::assert_eq(&Angle::new::<a::radian>(V::from_f64(2.0 * PI).unwrap()),
                &Angle::new::<a::revolution>(V::one()));
            Test::assert_eq(&Angle::new::<a::degree>(V::from_f64(360.0).unwrap()),
                &Angle::new::<a::revolution>(V::one()));
            Test::assert_approx_eq(&Angle::new::<a::gon>(V::from_f64(400.0).unwrap()),
                &Angle::new::<a::revolution>(V::one()));
            Test::assert_eq(&Angle::new::<a::minute>(V::from_f64(60.0).unwrap()),
                &Angle::new::<a::degree>(V::one()));
            Test::assert_eq(&Angle::new::<a::second>(V::from_f64(60.0 * 60.0).unwrap()),
                &Angle::new::<a::degree>(V::one()));
        }
    }
}
