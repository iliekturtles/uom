//! Frequency (base unit hertz, s<sup>-1</sup>).

quantity! {
    /// Frequency (base unit hertz, s<sup>-1</sup>).
    quantity: Frequency; "frequency";
    /// Frequency dimension, s<sup>-1</sup>.
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottahertz: prefix!(yotta); "YHz", "yottahertz", "yottahertz";
        @zettahertz: prefix!(zetta); "ZHz", "zettahertz", "zettahertz";
        @exahertz: prefix!(exa); "EHz", "exahertz", "exahertz";
        @petahertz: prefix!(peta); "PHz", "petahertz", "petahertz";
        @terahertz: prefix!(tera); "THz", "terahertz", "terahertz";
        @gigahertz: prefix!(giga); "GHz", "gigahertz", "gigahertz";
        @megahertz: prefix!(mega); "MHz", "megahertz", "megahertz";
        @kilohertz: prefix!(kilo); "kHz", "kilohertz", "kilohertz";
        @hectohertz: prefix!(hecto); "hHz", "hectohertz", "hectohertz";
        @decahertz: prefix!(deca); "daHz", "decahertz", "decahertz";
        /// The hertz is one cycle per second.
        @hertz: prefix!(none); "Hz", "hertz", "hertz";
        @decihertz: prefix!(deci); "dHz", "decihertz", "decihertz";
        @centihertz: prefix!(centi); "cHz", "centihertz", "centihertz";
        @millihertz: prefix!(milli); "mHz", "millihertz", "millihertz";
        @microhertz: prefix!(micro); "µHz", "microhertz", "microhertz";
        @nanohertz: prefix!(nano); "nHz", "nanohertz", "nanohertz";
        @picohertz: prefix!(pico); "pHz", "picohertz", "picohertz";
        @femtohertz: prefix!(femto); "fHz", "femtohertz", "femtohertz";
        @attohertz: prefix!(atto); "aHz", "attohertz", "attohertz";
        @zeptohertz: prefix!(zepto); "zHz", "zeptohertz", "zeptohertz";
        @yoctohertz: prefix!(yocto); "yHz", "yoctohertz", "yoctohertz";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::frequency as f;
        use si::time as t;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Time<V> = V::one() / Frequency::new::<f::hertz>(V::one());
            let _: Frequency<V> = V::one() / Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test(t::yottasecond, f::yoctohertz);
            test(t::zettasecond, f::zeptohertz);
            test(t::exasecond, f::attohertz);
            test(t::petasecond, f::femtohertz);
            test(t::terasecond, f::picohertz);
            test(t::gigasecond, f::nanohertz);
            test(t::megasecond, f::microhertz);
            test(t::kilosecond, f::millihertz);
            test(t::hectosecond, f::centihertz);
            test(t::decasecond, f::decihertz);
            test(t::second, f::hertz);
            test(t::decisecond, f::decahertz);
            test(t::centisecond, f::hectohertz);
            test(t::millisecond, f::kilohertz);
            test(t::microsecond, f::megahertz);
            test(t::nanosecond, f::gigahertz);
            test(t::picosecond, f::terahertz);
            test(t::femtosecond, f::petahertz);
            test(t::attosecond, f::exahertz);
            test(t::zeptosecond, f::zettahertz);
            test(t::yoctosecond, f::yottahertz);

            // TODO #17 Convert to == once PartialEq is implemented.
            fn test<T: t::Conversion<V>, F: f::Conversion<V>>(t: T, f: F) {
                Test::assert_approx_eq(&(V::one() / Time::new::<T>(V::one())).get(f),
                    &Frequency::new::<F>(V::one()).get(f));
                Test::assert_approx_eq(&Time::new::<T>(V::one()).get(t),
                    &(V::one() / Frequency::new::<F>(V::one())).get(t));
            }
        }
    }
}
