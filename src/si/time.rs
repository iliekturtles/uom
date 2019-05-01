//! Time (base unit second, s<sup>1</sup>).

use ::lib::time::Duration;

quantity! {
    /// Time (base unit second, s<sup>1</sup>).
    quantity: Time; "time";
    /// Time dimension, s<sup>1</sup>.
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

impl<U, V> From<Time<U, V>> for Duration
where
    U: ::si::Units<V> + ?Sized,
    V: ::num::Num + ::num::AsPrimitive<f64> + ::Conversion<V>,
{
    fn from(t: Time<U, V>) -> Duration {
        let secs = t.value.as_().abs();
        let nanos = (secs * 1e9) as u64 % 1e9 as u64;
        Duration::new(secs as u64, nanos as u32)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    storage_types! {
        types: Float;

        use ::lib::time::Duration;
        use si::quantities::*;
        use si::time::second;

        #[test]
        fn from() {
            let t = Time::new::<second>(-21.5);
            let d: Duration = t.into();
            assert_eq!(21, d.as_secs());
            assert_eq!(5e8 as u32, d.subsec_nanos());
        }
    }
}
