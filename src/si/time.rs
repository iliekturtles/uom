//! Time (base unit second, s).

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
        /// The second is the duration of 9 192 631 770 periods of the radiation corresponding to
        /// the transition between the two hyperfine levels of ground state of the caesium 133 atom.
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

        @day: 8.64_E4; "d", "day", "days";
        @hour: 3.6_E3; "h", "hour", "hours";
        @minute: 6.0_E1; "min", "minute", "minutes";
        @shake: 1.0_E-8; "10.0 ns", "shake", "shakes";
        @year: 3.1536_E7; "a", "year", "years";
    }
}


/// `Time` ⇌ `Duration` conversions.
#[cfg(feature = "try-from")]
pub mod convert {
    use lib::time::Duration;
    use num::{AsPrimitive, FromPrimitive, Zero};
    use super::{nanosecond, second, Time};
    
    /// An error encountered in converting a `Time` to a `Duration`.
    #[derive(Debug, Clone, Copy)]
    pub enum TryFromError {
        /// The given time interval was negative, making conversion to a duration nonsensical.
        ///
        /// To convert a negative time interval to a duration, first use `abs` to make it positive.
        NegativeDuration,
        /// The given time interval exceeded the maximum size of a `Duration`.
        Overflow,
    }

    /// Attempts to convert the given `Time` to a `Duration`.
    ///
    /// For possible failure modes, see [`TryFromError`][TryFromError].
    ///
    /// ## Notes
    ///
    /// The `Duration` to `Time` conversion is tested to be accurate to within 1 nanosecond
    /// (to allow for floating point rounding error). If greater precision is needed, consider
    /// using a different backing storage type or avoiding the conversion altogether.
    ///
    /// [TryFromError]: enum.TryFromError.html
    impl<U, V> ::lib::convert::TryFrom<Time<U, V>> for Duration
    where
        U: ::si::Units<V> + ?Sized,
        V: ::num::Num + ::Conversion<V> + ::lib::cmp::PartialOrd + AsPrimitive<u64> + AsPrimitive<u32>,
        second: ::Conversion<V, T = V::T>,
        nanosecond: ::Conversion<V, T = V::T>,
    {
        type Error = TryFromError;
    
        fn try_from(time: Time<U, V>) -> Result<Self, Self::Error> {
            if time < Time::<U, V>::zero() {
                return Err(TryFromError::NegativeDuration);
            }
    
            let secs = time.get::<second>().as_();
            let nanos = (time % Time::<U, V>::new::<second>(V::one())).get::<nanosecond>().as_();
    
            Ok(Duration::new(secs, nanos))
        }
    }
    
    impl<U, V> ::lib::convert::TryFrom<Duration> for Time<U, V>
    where
        U: ::si::Units<V> + ?Sized,
        V: ::num::Num + ::Conversion<V> + FromPrimitive,
        second: ::Conversion<V, T = V::T>,
        nanosecond: ::Conversion<V, T = V::T>,
    {
        type Error = TryFromError;
        /// Attempts to convert the given `Duration` to a `Time`.
        ///
        /// For possible failure modes, see [`TryFromError`][TryFromError].
        ///
        /// ## Notes
        ///
        /// The `Duration` to `Time` conversion is tested to be accurate to within 100 nanoseconds
        /// (to allow for floating point rounding error). If greater precision is needed, consider
        /// using a different backing storage type or avoiding the conversion altogether.
        ///
        /// [TryFromError]: enum.TryFromError.html
        fn try_from(duration: Duration) -> Result<Self, Self::Error> {
            let secs = V::from_u64(duration.as_secs());
            let nanos = V::from_u32(duration.subsec_micros());
    
            match (secs, nanos) {
                (Some(secs), Some(nanos)) => {
                    Ok(Time::<U, V>::new::<second>(secs) + Time::<U, V>::new::<nanosecond>(nanos))
                },
                _ => Err(TryFromError::Overflow),
            }
        }
    }
}

#[cfg(all(test, feature = "try-from"))]
mod tests {
    storage_types! {
        use lib::convert::{TryFrom, TryInto};
        use lib::time::Duration;
        use num::{AsPrimitive, FromPrimitive, Zero};
        use tests::*;
        use si::quantities::*;
        use si::time::nanosecond;
        use si::time::convert::TryFromError;
        use quickcheck::TestResult;

        quickcheck! {
            fn duration_try_from(v: A<V>) -> TestResult {
               test_try_from(Duration::try_from(Time::new::<nanosecond>(*v)), v)
            }

            fn time_try_into(v: A<V>) -> TestResult {
                test_try_from(Time::new::<nanosecond>(*v).try_into(), v)
            }

            fn time_try_from(v: A<V>) -> TestResult {
                if *v < V::zero() {
                    return TestResult::discard();
                }
                test_try_into(Time::try_from(Duration::from_nanos(v.as_())), v)
            }

            fn duration_try_into(v: A<V>) -> TestResult {
                if *v < V::zero() {
                    return TestResult::discard();
                }

                test_try_into(Duration::from_nanos(v.as_()).try_into(), v)
            }
        }

        fn test_try_from(t: Result<Duration, TryFromError>, v: A<V>) -> TestResult {
            let ok = match (t, *v >= V::zero()) {
                (Ok(t), true) => {
                    let d = Duration::from_nanos(v.as_());
                    let r = if d > t { d - t } else { t - d };
                    Duration::from_nanos(1) >= r
                },
                (Err(_), false) => true,
                _ => false,
            };
            TestResult::from_bool(ok)
        }
        
        fn test_try_into(t: Result<Time<V>, TryFromError>, v: A<V>) -> TestResult {
            let ok = if let Ok(t) = t {
                let b = Time::new::<nanosecond>(v.as_());
                let d = if t > b { t - b } else { b - t };
                d <= Time::new::<nanosecond>(V::from_u8(100).unwrap())
            } else {
                false
            };
            TestResult::from_bool(ok)
        }
    }
}
