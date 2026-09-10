//! Compile-time evaluation of conversion factors.
//!
//! Conversion factors are specified as `f64` literals in the [`unit!`] macro. For integer storage
//! types they are converted to rational numbers with `num_rational::Ratio::from_f64`, which runs
//! a continued fraction approximation at run time on every `Quantity::new` and `Quantity::get`
//! call. On targets without a floating point unit this pulls in software floating point and
//! rational arithmetic that dominates the code size of otherwise small programs. The [`Fraction`]
//! type performs the same approximation, and the multiplication of all base unit coefficients, in
//! `const fn`s so that the complete factor is a compile-time constant.

/// Rational number used to evaluate conversion factors at compile time.
///
/// The numerator carries the sign, the denominator is always greater than zero. Values are kept
/// reduced (numerator and denominator have no common divisor).
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fraction {
    /// Signed numerator.
    pub numer: i128,
    /// Denominator, always greater than zero.
    pub denom: u128,
}

impl Fraction {
    /// Multiplicative identity.
    pub const ONE: Fraction = Fraction { numer: 1, denom: 1 };

    /// Greatest common divisor.
    const fn gcd(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }

        a
    }

    /// Creates a reduced fraction from a sign, an unsigned numerator, and a denominator.
    const fn new(negative: bool, numer: u128, denom: u128) -> Option<Fraction> {
        if denom == 0 {
            return None;
        }

        let g = Self::gcd(numer, denom);
        let numer = numer / g;

        if numer > i128::MAX as u128 {
            return None;
        }

        Some(Fraction {
            numer: if negative { -(numer as i128) } else { numer as i128 },
            denom: denom / g,
        })
    }

    /// Returns `true` if the fraction is zero.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn is_zero(self) -> bool {
        self.numer == 0
    }

    /// Approximates `value` by a fraction whose numerator and denominator do not exceed `max`.
    ///
    /// This is the same continued fraction algorithm as `num_rational::Ratio::<T>::from_f64`
    /// (`approximate_float` with a maximum error of `10e-20` and 30 iterations), with `max` in
    /// place of `T::max_value()`, so that compile-time factors are identical to the run-time
    /// factors of the corresponding integer storage type.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn from_f64(value: f64, max: u128) -> Option<Fraction> {
        // NaN check without `f64::is_nan`, which is not `const` on the MSRV.
        #[allow(clippy::eq_op)]
        if value != value {
            return None;
        }

        let negative = value < 0.0;
        let value = if negative { -value } else { value };

        match Self::approximate(value, max) {
            Some((numer, denom)) => Self::new(negative, numer, denom),
            None => None,
        }
    }

    /// Continued fraction approximation of a non-negative `value`.
    const fn approximate(value: f64, max: u128) -> Option<(u128, u128)> {
        let max_error = 10e-20;
        let max_iterations = 30;
        let max_f = max as f64;
        let epsilon = 1.0 / max_f;

        if value > max_f {
            return None;
        }

        let mut q = value;
        let mut n0: u128 = 0;
        let mut d0: u128 = 1;
        let mut n1: u128 = 1;
        let mut d1: u128 = 0;
        let mut i = 0;

        while i < max_iterations {
            i += 1;

            if q > max_f {
                break;
            }

            let a = q as u128;
            let a_f = a as f64;
            let f = q - a_f;

            // Prevent overflow.
            if a != 0 && (n1 > max / a || d1 > max / a || a * n1 > max - n0 || a * d1 > max - d0) {
                break;
            }

            let n = a * n1 + n0;
            let d = a * d1 + d0;

            n0 = n1;
            d0 = d1;
            n1 = n;
            d1 = d;

            // `d1` is never zero here, so neither is the greatest common divisor.
            let g = Self::gcd(n1, d1);
            n1 /= g;
            d1 /= g;

            // Close enough?
            let error = n as f64 / d as f64 - value;
            let error = if error < 0.0 { -error } else { error };

            if error < max_error {
                break;
            }

            // Prevent division by ~0.
            if f < epsilon {
                break;
            }

            q = 1.0 / f;
        }

        if d1 == 0 {
            return None;
        }

        Some((n1, d1))
    }

    /// Multiplies two fractions. Returns `None` on overflow.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn mul(self, rhs: Fraction) -> Option<Fraction> {
        // Reduce across the fractions first to limit overflow, like `num_rational`.
        let (a, b) = (self.numer.unsigned_abs(), self.denom);
        let (c, d) = (rhs.numer.unsigned_abs(), rhs.denom);
        let g1 = Self::gcd(a, d);
        let g2 = Self::gcd(c, b);
        let numer = match (a / g1).checked_mul(c / g2) {
            Some(numer) => numer,
            None => return None,
        };
        let denom = match (b / g2).checked_mul(d / g1) {
            Some(denom) => denom,
            None => return None,
        };

        Self::new((self.numer < 0) != (rhs.numer < 0), numer, denom)
    }

    /// Reciprocal of the fraction. Returns `None` for zero.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn recip(self) -> Option<Fraction> {
        if self.numer == 0 {
            return None;
        }

        Self::new(self.numer < 0, self.denom, self.numer.unsigned_abs())
    }

    /// Divides two fractions. Returns `None` on overflow or division by zero.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn div(self, rhs: Fraction) -> Option<Fraction> {
        match rhs.recip() {
            Some(recip) => self.mul(recip),
            None => None,
        }
    }

    /// Raises the fraction to an integer power. Returns `None` on overflow.
    #[must_use = "method returns a new number and does not mutate the original value"]
    pub const fn powi(self, e: i32) -> Option<Fraction> {
        let base = if e < 0 {
            match self.recip() {
                Some(recip) => recip,
                None => return None,
            }
        } else {
            self
        };
        let mut e = e.unsigned_abs();
        let mut acc = Self::ONE;

        while e > 0 {
            acc = match acc.mul(base) {
                Some(acc) => acc,
                None => return None,
            };
            e -= 1;
        }

        Some(acc)
    }
}

#[cfg(test)]
mod tests {
    use super::Fraction;

    const fn fraction(numer: i128, denom: u128) -> Fraction {
        Fraction { numer, denom }
    }

    #[test]
    fn from_f64() {
        assert_eq!(Fraction::from_f64(1.0, u32::MAX as u128), Some(fraction(1, 1)));
        assert_eq!(Fraction::from_f64(1.0_E-3, u32::MAX as u128), Some(fraction(1, 1000)));
        assert_eq!(Fraction::from_f64(0.05, u32::MAX as u128), Some(fraction(1, 20)));
        assert_eq!(Fraction::from_f64(-273.15, i32::MAX as u128), Some(fraction(-5463, 20)));
        assert_eq!(Fraction::from_f64(f64::NAN, u32::MAX as u128), None);
        assert_eq!(Fraction::from_f64(1.0_E10, u32::MAX as u128), None);
        // Does not fit: falls back to the best approximation within bounds, like `num_rational`.
        assert_eq!(Fraction::from_f64(1.0_E-3, u8::MAX as u128), Some(fraction(0, 1)));
    }

    #[cfg(feature = "u32")]
    #[test]
    fn from_f64_matches_num_rational() {
        use crate::num::rational::Ratio;
        use crate::num::FromPrimitive;

        for value in [
            1.0,
            1.0_E-3,
            1.0_E3,
            0.05,
            0.3048,
            1.745_329_251_994_329_5_E-2,
            9.806_65_E0,
            273.15,
            1.0_E-24,
            1.0_E24,
        ] {
            let expected = Ratio::<u32>::from_f64(value)
                .map(|r| fraction(u128::from(*r.numer()) as i128, u128::from(*r.denom())));

            assert_eq!(Fraction::from_f64(value, u32::MAX as u128), expected, "{value}");
        }
    }

    #[test]
    fn arithmetic() {
        let km = fraction(1000, 1);
        let mm = fraction(1, 1000);

        assert_eq!(km.mul(mm), Some(fraction(1, 1)));
        assert_eq!(km.div(mm), Some(fraction(1_000_000, 1)));
        assert_eq!(mm.powi(2), Some(fraction(1, 1_000_000)));
        assert_eq!(mm.powi(-1), Some(km));
        assert_eq!(mm.powi(0), Some(Fraction::ONE));
        assert_eq!(fraction(0, 1).recip(), None);
        assert_eq!(fraction(-3, 4).mul(fraction(-2, 3)), Some(fraction(1, 2)));
        assert_eq!(fraction(i128::MAX, 1).mul(fraction(2, 1)), None);
    }
}
