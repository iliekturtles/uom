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
    kind: dyn (::si::marker::AngleKind);
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

/// Implementation of various stdlib trigonometric functions
#[cfg(feature = "std")]
impl<U, V> Angle<U, V>
where
    U: ::si::Units<V> + ?Sized,
    V: ::num::Float + ::Conversion<V>,
{
    /// Computes the value of the cosine of `Self`
    /// Return type: `V`
    #[inline(always)]
    pub fn cos(self) -> V {
        self.value.cos()
    }
    /// Computes the value of the hyperbolic cosine of `Self`
    /// Return type: `V`
    #[inline(always)]
    pub fn cosh(self) -> V {
        self.value.cosh()
    }
    /// Computes the value of the sine of `Self`
    /// Return type: `V`
    #[inline(always)]
    pub fn sin(self) -> V {
        self.value.sin()
    }
    /// Computes the value of the hyperbolic sine of `Self`
    /// Return type: `V`
    #[inline(always)]
    pub fn sinh(self) -> V {
        self.value.sinh()
    }
    /// Computes the value of both the sine and cosine of `Self`
    /// Return type: `(V, V)`
    #[inline(always)]
    pub fn sin_cos(self) -> (V, V) {
        self.value.sin_cos()
    }
    /// Computes the value of the tangent of `Self`
    /// Return type: `V`
    #[inline(always)]
    pub fn tan(self) -> V {
        self.value.tan()
    }
    /// Computes the value of the hyperbolic tangent of `Self`
    /// Return type: `V`
    #[inline(always)]
    pub fn tanh(self) -> V {
        self.value.tanh()
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
        use num::{FromPrimitive, One};
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


#[cfg(test)]
mod trig {
    storage_types! {
        use approx::assert_ulps_eq;
        use num::Zero;
        use si::quantities::*;

        #[test]
        fn sanity() {
            let zero: Angle<V> = Angle::<V>::from(V::zero());
            let nzero: Angle<V> = Angle::<V>::from(-1.0 * V::zero());

            // PI constant taken from Rust stdlib source
            let mut pi: Angle<V> = Angle::<V>::from(V::zero());
            pi.value = 3.14159265358979323846264338327950288;

            // PI / 2 constant taken from Rust stdlib source
            let mut half: Angle<V> = Angle::<V>::from(V::zero());
            half.value = 1.57079632679489661923132169163975144;

            assert_ulps_eq!(zero.cos(), 1.0);
            assert_ulps_eq!(nzero.cos(), 1.0);

            assert_ulps_eq!(pi.cos(), -1.0);
            assert_ulps_eq!(half.cos(), 0.0);

            assert_ulps_eq!(zero.sin(), 0.0);
            assert_ulps_eq!(nzero.sin(), 0.0);

            assert_ulps_eq!(pi.sin(), 0.0);
            assert_ulps_eq!(half.sin(), 1.0);

            assert_ulps_eq!(zero.tan(), 0.0);
            assert_ulps_eq!(nzero.tan(), 0.0);

            assert_ulps_eq!(pi.tan(), 0.0);

            // Cannot test for PI / 2 equality as it diverges to infinity
            // Float inaccuracy does not guarantee a NAN or INFINITY result
            //let result = half.tan();
            //assert!(result == V::nan() || result == V::infinity());

            assert_ulps_eq!(zero.cosh(), 1.0);
            assert_ulps_eq!(nzero.cosh(), 1.0);

            assert_ulps_eq!(zero.sinh(), 0.0);
            assert_ulps_eq!(nzero.sinh(), 0.0);

            assert_ulps_eq!(zero.tanh(), 0.0);
            assert_ulps_eq!(nzero.tanh(), 0.0);
        }
    }
}