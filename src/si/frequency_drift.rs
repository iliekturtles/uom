//! Frequency drift (base unit hertz per second, s⁻²).
//!
//! Typical application: Frequency slope in FMCW radars.

quantity! {
    /// Frequency drift (base unit hertz per second, s⁻²).
    quantity: FrequencyDrift; "frequency drift";
    /// Dimension of frequency drift, T⁻² (base unit hertz per second, s⁻²).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @terahertz_per_second: prefix!(tera) / prefix!(none); "THz/s", "terahertz per second",
            "terahertz per second";
        @gigahertz_per_second: prefix!(giga) / prefix!(none); "GHz/s", "gigahertz per second",
            "gigahertz per second";
        @megahertz_per_second: prefix!(mega) / prefix!(none); "MHz/s", "megahertz per second",
            "megahertz per second";
        @kilohertz_per_second: prefix!(kilo) / prefix!(none); "kHz/s", "kilohertz per second",
            "kilohertz per second";
        @hertz_per_second: prefix!(none) / prefix!(none); "Hz/s", "hertz per second",
            "hertz per second";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::frequency_drift as fd;
        use crate::si::frequency as f;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: FrequencyDrift<V> = Frequency::new::<f::hertz>(V::one())
                / Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<f::terahertz, t::second, fd::terahertz_per_second>();
            test::<f::gigahertz, t::second, fd::gigahertz_per_second>();
            test::<f::megahertz, t::second, fd::megahertz_per_second>();
            test::<f::kilohertz, t::second, fd::kilohertz_per_second>();
            test::<f::hertz, t::second, fd::hertz_per_second>();

            fn test<F: f::Conversion<V>, T: t::Conversion<V>, FD: fd::Conversion<V>>() {
                Test::assert_approx_eq(&FrequencyDrift::new::<FD>(V::one()),
                    &(Frequency::new::<F>(V::one())
                        / Time::new::<T>(V::one())));
            }
        }
    }
}
