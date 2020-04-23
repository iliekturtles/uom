//! Time (base unit second, s).

#[cfg(feature = "try-from")]
use lib::time::Duration;
#[cfg(feature = "try-from")]
use num::{FromPrimitive, ToPrimitive, Zero};

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
#[cfg(feature = "try-from")]
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
#[cfg(feature = "try-from")]
impl<U, V> ::lib::convert::TryFrom<Time<U, V>> for Duration
where
    U: ::si::Units<V> + ?Sized,
    V: ::num::Num + ::Conversion<V> + ::lib::cmp::PartialOrd + ToPrimitive,
    second: ::Conversion<V, T = V::T>,
    nanosecond: ::Conversion<V, T = V::T>,
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
#[cfg(feature = "try-from")]
impl<U, V> ::lib::convert::TryFrom<Duration> for Time<U, V>
where
    U: ::si::Units<V> + ?Sized,
    V: ::num::Num + ::Conversion<V> + FromPrimitive,
    second: ::Conversion<V, T = V::T>,
    nanosecond: ::Conversion<V, T = V::T>,
{
    type Error = TryFromError;

    fn try_from(duration: Duration) -> Result<Self, Self::Error> {
        let secs = V::from_u64(duration.as_secs());
        let nanos = V::from_u32(duration.subsec_micros());

        match (secs, nanos) {
            (Some(secs), Some(nanos)) => {
                Ok(Time::<U, V>::new::<second>(secs) + Time::<U, V>::new::<nanosecond>(nanos))
            }
            _ => Err(TryFromError::Overflow),
        }
    }
}

#[cfg(all(test, feature = "try-from"))]
mod tests {
    storage_types! {
        types: PrimInt, BigInt, BigUint, Float;

        use lib::convert::{TryFrom, TryInto};
        use lib::time::Duration;
        use num::{FromPrimitive, ToPrimitive, Zero};
        use tests::*;
        use si::quantities::*;
        use si::time::{TryFromError, nanosecond};
        use quickcheck::TestResult;

        quickcheck! {
            fn duration_try_from(v: A<V>) -> TestResult {
                test_try_from(Duration::try_from(Time::new::<nanosecond>((*v).clone())), v)
            }

            fn time_try_into(v: A<V>) -> TestResult {
                test_try_from(Time::new::<nanosecond>((*v).clone()).try_into(), v)
            }

            fn time_try_from(v: A<V>) -> TestResult {
                match v.to_u64() {
                    Some(u) => test_try_into(Time::try_from(Duration::from_nanos(u)), v),
                    None => TestResult::discard(),
                }
            }

            fn duration_try_into(v: A<V>) -> TestResult {
                match v.to_u64() {
                    Some(u) => test_try_into(Duration::from_nanos(u).try_into(), v),
                    None => TestResult::discard(),
                }
            }
        }

        fn test_try_from(t: Result<Duration, TryFromError>, v: A<V>) -> TestResult {
            if *v < V::zero() {
                return TestResult::discard();
            }

            let ok = match (t, v.to_u64()) {
                (Ok(t), Some(u)) => {
                    let d = Duration::from_nanos(u);
                    let r = if d > t { d - t } else { t - d };

                    Duration::from_nanos(1) >= r
                },
                (Err(_), None) => true,
                _ => false,
            };

            TestResult::from_bool(ok)
        }

        fn test_try_into(t: Result<Time<V>, TryFromError>, v: A<V>) -> TestResult {
            if *v < V::zero() {
                return TestResult::discard();
            }

            let ok = match t {
                Ok(t) => {
                    let b = Time::new::<nanosecond>((*v).clone());
                    let d = if t > b { t - b } else { b - t };

                    d <= Time::new::<nanosecond>(V::from_u8(100).unwrap())
                },
                _ => false,
            };

            TestResult::from_bool(ok)
        }
    }
}
