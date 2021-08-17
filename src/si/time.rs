//! Time (base unit second, s).

use crate::lib::time::Duration;
use crate::num::{FromPrimitive, ToPrimitive, Zero};

quantity! {
    /// Time (base unit second, s).
    quantity: Time; "time";
    /// Dimension of time, T (base unit second, s).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        P1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottasecond: prefix!(yotta); "Ys", "yottasecond", "yottaseconds";
        @zettasecond: prefix!(zetta); "Zs", "zettasecond", "zettaseconds";
        @exasecond: prefix!(exa); "Es", "exasecond", "exaseconds";
        @petasecond: prefix!(peta); "Ps", "petasecond", "petaseconds";
        @terasecond: prefix!(tera); "Ts", "terasecond", "teraseconds";
        @gigasecond: prefix!(giga); "Gs", "gigasecond", "gigaseconds";
        @megasecond: prefix!(mega); "Ms", "megasecond", "megaseconds";
        @kilosecond: prefix!(kilo); "ks", "kilosecond", "kiloseconds";
        @hectosecond: prefix!(hecto); "hs", "hectosecond", "hectoseconds";
        @decasecond: prefix!(deca); "das", "decasecond", "decaseconds";
        /// The second is the SI unit of time. It is defined by taking the fixed numerical value of
        /// the caesium frequency ∆*ν*<sub>Cs</sub>, the unperturbed ground-state hyperfine
        /// transition frequency of the caesium 133 atom, to be 9 192 631 770 when expressed in the
        /// unit Hz, which is equal to s⁻¹.
        @second: prefix!(none); "s", "second", "seconds";
        @decisecond: prefix!(deci); "ds", "decisecond", "deciseconds";
        @centisecond: prefix!(centi); "cs", "centisecond", "centiseconds";
        @millisecond: prefix!(milli); "ms", "millisecond", "milliseconds";
        @microsecond: prefix!(micro); "µs", "microsecond", "microseconds";
        @nanosecond: prefix!(nano); "ns", "nanosecond", "nanoseconds";
        @picosecond: prefix!(pico); "ps", "picosecond", "picoseconds";
        @femtosecond: prefix!(femto); "fs", "femtosecond", "femtoseconds";
        @attosecond: prefix!(atto); "as", "attosecond", "attoseconds";
        @zeptosecond: prefix!(zepto); "zs", "zeptosecond", "zeptoseconds";
        @yoctosecond: prefix!(yocto); "ys", "yoctosecond", "yoctoseconds";

        @second_sidereal: 9.972_696_E-1; "s (sidereal)", "second (sidereal)", "seconds (sidereal)";
        @day: 8.64_E4; "d", "day", "days";
        @day_sidereal: 8.616_409_E4; "d (sidereal)", "day (sidereal)", "days (sidereal)";
        @hour: 3.6_E3; "h", "hour", "hours";
        @hour_sidereal: 3.590_170_E3; "h (sidereal)", "hour (sidereal)", "hours (sidereal)";
        @minute: 6.0_E1; "min", "minute", "minutes";
        @shake: 1.0_E-8; "10.0 ns", "shake", "shakes";
        @year: 3.1536_E7; "a", "year", "years";
        @year_sidereal: 3.155_815_E7; "a (sidereal)", "year (sidereal)", "years (sidereal)";
        @year_tropical: 3.155_693_E7; "a (tropical)", "year (tropical)", "years (tropical)";
    }
}

/// An error encountered converting between `Time` and `Duration`.
#[derive(Debug, Clone, Copy)]
pub enum TryFromError {
    /// The given time interval was negative, making conversion to a duration nonsensical.
    ///
    /// To convert a negative time interval to a duration, first use `abs` to make it positive.
    NegativeDuration,

    /// The given time interval exceeded the maximum size of a `Duration`.
    Overflow,
}

/// Attempt to convert the given `Time` to a `Duration`.
///
/// For possible failure modes see [`TryFromError`][TryFromError].
///
/// ## Notes
///
/// The `Duration` to `Time` conversion is tested to be accurate to within 1 nanosecond (to allow
/// for floating point rounding error). If greater precision is needed, consider using a different
/// underlying storage type or avoiding the conversion altogether.
///
/// [TryFromError]: enum.TryFromError.html
impl<U, V> crate::lib::convert::TryFrom<Time<U, V>> for Duration
where
    U: crate::si::Units<V> + ?Sized,
    V: crate::num::Num + crate::Conversion<V> + PartialOrd + ToPrimitive,
    second: crate::Conversion<V, T = V::T>,
    nanosecond: crate::Conversion<V, T = V::T>,
{
    type Error = TryFromError;

    fn try_from(time: Time<U, V>) -> Result<Self, Self::Error> {
        if time < Time::<U, V>::zero() {
            return Err(TryFromError::NegativeDuration);
        }

        let secs = time.get::<second>().to_u64();
        let nanos = (time % Time::<U, V>::new::<second>(V::one())).get::<nanosecond>().to_u32();

        match (secs, nanos) {
            (Some(secs), Some(nanos)) => Ok(Duration::new(secs, nanos)),
            _ => Err(TryFromError::Overflow),
        }
    }
}

/// Attempt to convert the given `Duration` to a `Time`.
///
/// For possible failure modes, see [`TryFromError`][TryFromError].
///
/// ## Notes
///
/// The `Duration` to `Time` conversion is tested to be accurate to within 100 nanoseconds (to
/// allow for floating point rounding error). If greater precision is needed, consider using a
/// different underlying storage type or avoiding the conversion altogether.
///
/// [TryFromError]: enum.TryFromError.html
impl<U, V> crate::lib::convert::TryFrom<Duration> for Time<U, V>
where
    U: crate::si::Units<V> + ?Sized,
    V: crate::num::Num + crate::Conversion<V> + FromPrimitive,
    second: crate::Conversion<V, T = V::T>,
    nanosecond: crate::Conversion<V, T = V::T>,
{
    type Error = TryFromError;

    fn try_from(duration: Duration) -> Result<Self, Self::Error> {
        let secs = V::from_u64(duration.as_secs());
        let nanos = V::from_u32(duration.subsec_nanos());

        match (secs, nanos) {
            (Some(secs), Some(nanos)) => {
                Ok(Time::<U, V>::new::<second>(secs) + Time::<U, V>::new::<nanosecond>(nanos))
            }
            _ => Err(TryFromError::Overflow),
        }
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        types: PrimInt, BigInt, BigUint, Float;

        use crate::ConversionFactor;
        use crate::lib::convert::TryFrom;
        use crate::lib::time::Duration;
        use crate::num::{FromPrimitive, ToPrimitive, One, Zero};
        use crate::si::quantities::*;
        use crate::si::time::{TryFromError, second, nanosecond};
        use crate::tests::*;
        use quickcheck::TestResult;

        quickcheck! {
            fn duration_try_from(v: A<V>) -> bool {
                let ns: V = <nanosecond as crate::Conversion<V>>::coefficient().value();
                let t = Time::new::<second>((*v).clone());
                let d = Duration::try_from(t);
                let r = (*v).clone() % V::one();
                let s = ((*v).clone() - r.clone()).to_u64();
                let n = (r * (V::one() / &ns)).to_u32();

                match (d, s, n) {
                    (Ok(d), Some(s), Some(n)) => d.as_secs() == s && d.subsec_nanos() == n,
                    (Err(TryFromError::NegativeDuration), _, _) if *v < V::zero() => true,
                    (Err(TryFromError::Overflow), None, _) => true,
                    (Err(TryFromError::Overflow), _, None) => true,
                    _ => false,
                }
            }

            fn time_try_from(v: A<V>) -> TestResult {
                if *v < V::zero()  {
                    return TestResult::discard();
                }

                let ns: V = <nanosecond as crate::Conversion<V>>::coefficient().value();
                let r = (*v).clone() % V::one();
                let s = ((*v).clone() - r.clone()).to_u64();
                let n = (r * (V::one() / &ns)).to_u32();

                return match (s, n) {
                        (Some(s), Some(n)) => TestResult::from_bool(
                            match (Time::try_from(Duration::new(s, n)), V::from_u64(s), V::from_u32(n)) {
                                (Ok(t), Some(s), Some(n)) => t == Time::new::<second>(s) + Time::new::<nanosecond>(n),
                                (Err(TryFromError::Overflow), None, _) => true,
                                (Err(TryFromError::Overflow), _, None) => true,
                                _ => false,
                            }),
                        _ => TestResult::discard(),
                    }
            }
        }
    }
}
