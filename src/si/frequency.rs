//! Frequency (base unit hertz, s⁻¹).

quantity! {
    /// Frequency (base unit hertz, s⁻¹).
    quantity: Frequency; "frequency";
    /// Dimension of frequency, T⁻¹ (base unit hertz, s⁻¹).
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

        @cycle_per_day: 1.157_407_407_407_407_4_E-5; "1/d", "cycle per day", "cycles per day";
        @cycle_per_hour: 2.777_777_777_777_777_E-4; "1/h", "cycle per hour", "cycles per hour";
        @cycle_per_minute: 1.666_666_666_666_666_6E-2; "1/min", "cycle per minute", "cycles per minute";
        @cycle_per_shake: 1.0_E8; "100 MHz", "cycle per shake", "cycles per shake";
        @cycle_per_year: 3.170_979_198_376_458_E-8; "1/a", "cycle per year", "cycles per year";
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
            test::<t::yottasecond, f::yoctohertz>();
            test::<t::zettasecond, f::zeptohertz>();
            test::<t::exasecond, f::attohertz>();
            test::<t::petasecond, f::femtohertz>();
            test::<t::terasecond, f::picohertz>();
            test::<t::gigasecond, f::nanohertz>();
            test::<t::megasecond, f::microhertz>();
            test::<t::kilosecond, f::millihertz>();
            test::<t::hectosecond, f::centihertz>();
            test::<t::decasecond, f::decihertz>();
            test::<t::second, f::hertz>();
            test::<t::decisecond, f::decahertz>();
            test::<t::centisecond, f::hectohertz>();
            test::<t::millisecond, f::kilohertz>();
            test::<t::microsecond, f::megahertz>();
            test::<t::nanosecond, f::gigahertz>();
            test::<t::picosecond, f::terahertz>();
            test::<t::femtosecond, f::petahertz>();
            test::<t::attosecond, f::exahertz>();
            test::<t::zeptosecond, f::zettahertz>();
            test::<t::yoctosecond, f::yottahertz>();
            test::<t::day, f::cycle_per_day>();
            test::<t::hour, f::cycle_per_hour>();
            test::<t::minute, f::cycle_per_minute>();
            test::<t::shake, f::cycle_per_shake>();
            test::<t::year, f::cycle_per_year>();

            fn test<T: t::Conversion<V>, F: f::Conversion<V>>() {
                Test::assert_approx_eq(&(V::one() / Time::new::<T>(V::one())),
                    &Frequency::new::<F>(V::one()));
                Test::assert_approx_eq(&Time::new::<T>(V::one()),
                    &(V::one() / Frequency::new::<F>(V::one())));
            }
        }
    }
}
