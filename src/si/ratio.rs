//! Ratio (dimensionless quantity).

#[cfg(feature = "std")]
use super::angle::{Angle, radian};

quantity! {
    /// Ratio (dimensionless quantity).
    quantity: Ratio; "ratio";
    /// Dimension of ratio, 1 (dimensionless).
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

/// Implementation of various stdlib functions.
#[cfg(feature = "std")]
impl<U, V> Ratio<U, V>
where
    U: crate::si::Units<V> + ?Sized,
    V: crate::num::Float + crate::Conversion<V>,
    radian: crate::Conversion<V, T = V::T>,
    ratio: crate::Conversion<V, T = V::T>,
{
    /// Computes the value of the inverse cosine of the ratio.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn acos(self) -> Angle<U, V> {
        Angle::new::<radian>(self.value.acos())
    }

    /// Computes the value of the inverse hyperbolic cosine of the ratio.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn acosh(self) -> Angle<U, V> {
        Angle::new::<radian>(self.value.acosh())
    }

    /// Computes the value of the inverse sine of the ratio.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn asin(self) -> Angle<U, V> {
        Angle::new::<radian>(self.value.asin())
    }

    /// Computes the value of the inverse hyperbolic sine of the ratio.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn asinh(self) -> Angle<U, V> {
        Angle::new::<radian>(self.value.asinh())
    }

    /// Computes the value of the inverse tangent of the ratio.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn atan(self) -> Angle<U, V> {
        Angle::new::<radian>(self.value.atan())
    }

    /// Computes the value of the inverse hyperbolic tangent of the ratio.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn atanh(self) -> Angle<U, V> {
        Angle::new::<radian>(self.value.atanh())
    }

    /// Returns `e^(self)`, (the exponential function).
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn exp(self) -> Ratio<U, V> {
        Ratio::new::<ratio>(self.value.exp())
    }

    /// Returns 2^(self).
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn exp2(self) -> Ratio<U, V> {
        Ratio::new::<ratio>(self.value.exp2())
    }

    /// Returns the natural logarithm of the number.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn ln(self) -> Ratio<U, V> {
        Ratio::new::<ratio>(self.value.ln())
    }

    /// Returns the logarithm of the number with respect to an arbitrary base.
    ///
    /// The result might not be correctly rounded owing to implementation
    /// details; self.log2() can produce more accurate results for base 2, and
    /// self.log10() can produce more accurate results for base 10.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn log(self, base: V) -> Ratio<U, V> {
        Ratio::new::<ratio>(self.value.log(base))
    }

    /// Returns the base 2 logarithm of the number.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn log2(self) -> Ratio<U, V> {
        Ratio::new::<ratio>(self.value.log2())
    }

    /// Returns the base 10 logarithm of the number.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn log10(self) -> Ratio<U, V> {
        Ratio::new::<ratio>(self.value.log10())
    }

    /// Returns e^(self) - 1 in a way that is accurate even if the number is close to zero.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn exp_m1(self) -> Ratio<U, V> {
        Ratio::new::<ratio>(self.value.exp_m1())
    }

    /// Returns ln(1+n) (natural logarithm) more accurately than if the
    /// operations were performed separately.
    #[must_use = "method returns a new number and does not mutate the original value"]
    #[inline(always)]
    pub fn ln_1p(self) -> Ratio<U, V> {
        Ratio::new::<ratio>(self.value.ln_1p())
    }
}

mod convert {
    use super::*;

    impl<U, V> From<V> for Ratio<U, V>
    where
        U: crate::si::Units<V> + ?Sized,
        V: crate::num::Num + crate::Conversion<V>,
    {
        fn from(t: V) -> Self {
            Ratio {
                dimension: crate::lib::marker::PhantomData,
                units: crate::lib::marker::PhantomData,
                value: t,
            }
        }
    }

    storage_types! {
        use super::*;

        impl<U> From<Ratio<U, V>> for V
        where
            U: crate::si::Units<V> + ?Sized,
            V: crate::num::Num + crate::Conversion<V>,
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
        use crate::num::{FromPrimitive, One};
        use crate::si::quantities::*;
        use crate::si::ratio as r;
        use crate::tests::Test;

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

    #[cfg(feature = "std")]
    mod float {
        storage_types! {
            types: Float;

            use crate::si::angle as a;
            use crate::si::ratio as r;
            use crate::si::quantities::*;
            use crate::tests::Test;

            quickcheck! {
                fn acos(x: V) -> bool {
                    Test::eq(&x.acos(), &Ratio::from(x).acos().get::<a::radian>())
                }

                fn acosh(x: V) -> bool {
                    Test::eq(&x.acosh(), &Ratio::from(x).acosh().get::<a::radian>())
                }

                fn asin(x: V) -> bool {
                    Test::eq(&x.asin(), &Ratio::from(x).asin().get::<a::radian>())
                }

                fn asinh(x: V) -> bool {
                    Test::eq(&x.asinh(), &Ratio::from(x).asinh().get::<a::radian>())
                }

                fn atan(x: V) -> bool {
                    Test::eq(&x.atan(), &Ratio::from(x).atan().get::<a::radian>())
                }

                fn atanh(x: V) -> bool {
                    Test::eq(&x.atanh(), &Ratio::from(x).atanh().get::<a::radian>())
                }

                fn exp(x: V) -> bool {
                    Test::eq(&x.exp(), &Ratio::from(x).exp().get::<r::ratio>())
                }

                fn exp2(x: V) -> bool {
                    Test::eq(&x.exp2(), &Ratio::from(x).exp2().get::<r::ratio>())
                }

                fn ln(x: V) -> bool {
                    Test::eq(&x.ln(), &Ratio::from(x).ln().get::<r::ratio>())
                }

                fn log(x: V, y: V) -> bool {
                    Test::eq(&x.log(y), &Ratio::from(x).log(y).get::<r::ratio>())
                }

                fn log2(x: V) -> bool {
                    Test::eq(&x.log2(), &Ratio::from(x).log2().get::<r::ratio>())
                }

                fn log10(x: V) -> bool {
                    Test::eq(&x.log10(), &Ratio::from(x).log10().get::<r::ratio>())
                }

                fn exp_m1(x: V) -> bool {
                    Test::eq(&x.exp_m1(), &Ratio::from(x).exp_m1().get::<r::ratio>())
                }

                fn ln_1p(x: V) -> bool {
                    Test::eq(&x.ln_1p(), &Ratio::from(x).ln_1p().get::<r::ratio>())
                }
            }
        }
    }
}
