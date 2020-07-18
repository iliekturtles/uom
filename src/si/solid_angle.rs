//! Solid angle (dimensionless quantity).

quantity! {
    /// Solid angle (dimensionless quantity).
    quantity: SolidAngle; "solid angle";
    /// Dimension of solid angle, 1 (dimensionless).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (::si::marker::SolidAngleKind);
    units {
        /// SI derived unit of solid angle is steradians. It is the solid angle subtended at the
        /// center of a unit sphere by a unit area on its surface.
        @steradian: 1.0_E0; "sr", "steradian", "steradians";
        @spat: 1.256_637_061_435_917_3_E1; "sp", "spat", "spats";
        @square_degree: 3.046_174_197_867_086_E-4; "°²", "square degree", "square degrees";
        @square_minute: 8.461_594_994_075_238_9_E-8; "′²", "square minute", "square minutes";
        @square_second: 2.350_443_053_909_788_6_E-11; "″²", "square second", "square seconds";
    }
}

#[cfg(feature = "f32")]
impl SolidAngle<::si::SI<f32>, f32> {
    /// The solid angle subtended by a sphere at its center, i.e. with a value 4π as measured in
    /// steradians.
    pub const SPHERE: Self = Self {
        dimension: ::lib::marker::PhantomData,
        units: ::lib::marker::PhantomData,
        value: 4. * ::lib::f32::consts::PI,
    };
}

#[cfg(feature = "f64")]
impl SolidAngle<::si::SI<f64>, f64> {
    /// The solid angle subtended by a sphere at its center, i.e. with a value 4π as measured in
    /// steradians.
    pub const SPHERE: Self = Self {
        dimension: ::lib::marker::PhantomData,
        units: ::lib::marker::PhantomData,
        value: 4. * ::lib::f64::consts::PI,
    };
}

mod convert {
    use super::*;

    impl<U, V> ::lib::convert::From<V> for SolidAngle<U, V>
    where
        U: ::si::Units<V> + ?Sized,
        V: ::num::Num + ::Conversion<V>,
    {
        fn from(t: V) -> Self {
            SolidAngle {
                dimension: ::lib::marker::PhantomData,
                units: ::lib::marker::PhantomData,
                value: t,
            }
        }
    }

    storage_types! {
        use super::*;

        impl<U> ::lib::convert::From<SolidAngle<U, V>> for V
        where
            U: ::si::Units<V> + ?Sized,
            V: ::num::Num + ::Conversion<V>,
        {
            fn from(t: SolidAngle<U, V>) -> Self {
                t.value
            }
        }
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use ::lib::f64::consts::PI;
        use num::{FromPrimitive, One};
        use si::solid_angle as sa;
        use si::quantities::*;
        use tests::Test;

        #[test]
        fn from() {
            let r1: SolidAngle<V> = SolidAngle::<V>::from(V::one());
            let r2: SolidAngle<V> = V::one().into();
            let _: V = V::from(r1);
            let _: V = r2.into();
        }

        #[test]
        fn check_units() {
            Test::assert_eq(&SolidAngle::new::<sa::steradian>(V::from_f64(4.0 * PI).unwrap()),
                &SolidAngle::new::<sa::spat>(V::one()));
            Test::assert_approx_eq(
                &SolidAngle::new::<sa::square_degree>(V::from_f64(360.0 * 360.0 / PI).unwrap()),
                &SolidAngle::new::<sa::spat>(V::one()));
            Test::assert_approx_eq(
                &SolidAngle::new::<sa::square_minute>(V::from_f64(60.0 * 60.0).unwrap()),
                &SolidAngle::new::<sa::square_degree>(V::one()));
            Test::assert_approx_eq(
                &SolidAngle::new::<sa::square_second>(
                    V::from_f64((60.0 * 60.0) * (60.0 * 60.0)).unwrap()),
                &SolidAngle::new::<sa::square_degree>(V::one()));
        }
    }
}
