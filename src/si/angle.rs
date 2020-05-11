//! Angle (dimensionless quantity).

#[cfg(feature = "std")]
use super::ratio::Ratio;

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
    /// Computes the value of the cosine of the angle.
    #[inline(always)]
    pub fn cos(self) -> Ratio<U, V> {
        self.value.cos().into()
    }

    /// Computes the value of the hyperbolic cosine of the angle.
    #[inline(always)]
    pub fn cosh(self) -> Ratio<U, V> {
        self.value.cosh().into()
    }

    /// Computes the value of the sine of the angle.
    #[inline(always)]
    pub fn sin(self) -> Ratio<U, V> {
        self.value.sin().into()
    }

    /// Computes the value of the hyperbolic sine of the angle.
    #[inline(always)]
    pub fn sinh(self) -> Ratio<U, V> {
        self.value.sinh().into()
    }

    /// Computes the value of both the sine and cosine of the angle.
    #[inline(always)]
    pub fn sin_cos(self) -> (Ratio<U, V>, Ratio<U, V>) {
        let (sin, cos) = self.value.sin_cos();
        (sin.into(), cos.into())
    }

    /// Computes the value of the tangent of the angle.
    #[inline(always)]
    pub fn tan(self) -> Ratio<U, V> {
        self.value.tan().into()
    }

    /// Computes the value of the hyperbolic tangent of the angle.
    #[inline(always)]
    pub fn tanh(self) -> Ratio<U, V> {
        self.value.tanh().into()
    }
}

#[cfg(feature = "std")]
impl<D, U, V> ::si::Quantity<D, U, V>
where
    D: ::si::Dimension + ?Sized,
    U: ::si::Units<V> + ?Sized,
    V: ::num::Float + ::Conversion<V>,
    radian: ::Conversion<V, T = V::T>,
{
    /// Computes the four quadrant arctangent of self (y) and other (x).
    #[inline(always)]
    pub fn atan2(self, other: Self) -> Angle<U, V> {
        Angle::new::<radian>(self.value.atan2(other.value))
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

    #[cfg(feature = "std")]
    mod trig {
        storage_types! {
            types: Float;

            use ::lib::f64::consts::PI;
            use num::{FromPrimitive, Zero};
            use si::angle as a;
            use si::length as l;
            use si::quantities::*;
            use tests::Test;

            #[test]
            fn sanity() {
                let zero: Angle<V> = Angle::zero();
                let nzero: Angle<V> = -Angle::zero();
                let pi: Angle<V> = Angle::new::<a::radian>(V::from_f64(PI).unwrap());
                let half: Angle<V> = Angle::new::<a::radian>(V::from_f64(PI / 2.0).unwrap());

                Test::assert_approx_eq(&zero.cos().into(), &1.0);
                Test::assert_approx_eq(&nzero.cos().into(), &1.0);

                Test::assert_approx_eq(&pi.cos().into(), &-1.0);
                Test::assert_approx_eq(&half.cos().into(), &0.0);

                Test::assert_approx_eq(&zero.sin().into(), &0.0);
                Test::assert_approx_eq(&nzero.sin().into(), &0.0);

                // Float inaccuracy does not guarantee approximate values
                // In these tests, it diverges slightly over the epsilon value
                //Test::assert_approx_eq(&pi.sin().into(), &0.0);
                //Test::assert_approx_eq(&half.sin().into(), &1.0);

                Test::assert_approx_eq(&zero.tan().into(), &0.0);
                Test::assert_approx_eq(&nzero.tan().into(), &0.0);

                //Test::assert_approx_eq(&pi.tan(), &0.0);
                // Cannot test for PI / 2 equality as it diverges to infinity
                // Float inaccuracy does not guarantee a NAN or INFINITY result
                //let result = half.tan().into();
                //assert!(result == V::nan() || result == V::infinity());

                Test::assert_approx_eq(&zero.cosh().into(), &1.0);
                Test::assert_approx_eq(&nzero.cosh().into(), &1.0);

                Test::assert_approx_eq(&zero.sinh().into(), &0.0);
                Test::assert_approx_eq(&nzero.sinh().into(), &0.0);

                Test::assert_approx_eq(&zero.tanh().into(), &0.0);
                Test::assert_approx_eq(&nzero.tanh().into(), &0.0);
            }

            quickcheck! {
                #[allow(trivial_casts)]
                fn atan2(y: V, x: V) -> bool {
                    Test::eq(&y.atan2(x),
                        &Length::new::<l::meter>(y).atan2(Length::new::<l::meter>(x)).get::<a::radian>())
                }
            }
        }
    }
}
