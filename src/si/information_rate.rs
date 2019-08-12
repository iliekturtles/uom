//! Information rate (base unit byte per second, s⁻¹).

quantity! {
    /// Information rate (base unit byte per second, s⁻¹).
    quantity: InformationRate; "information rate";
    /// Dimension of information rate, T⁻¹ (base unit byte per second, s⁻¹).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn(::si::marker::InformationKind);
    units {
        @yobibit_per_second: prefix!(yobi) * prefix!(none) / 8.0; "Yib/s", "yobibit per second",
            "yobibits per second";
        @yottabit_per_second: prefix!(yotta) * prefix!(none) / 8.0; "Yb/s", "yottabit per second",
            "yottabits per second";
        @zebibit_per_second: prefix!(zebi) * prefix!(none) / 8.0; "Zib/s", "zebibit per second",
            "zebibits per second";
        @zettabit_per_second: prefix!(zetta) * prefix!(none) / 8.0; "Zb/s", "zettabit per second",
            "zettabits per second";
        @exbibit_per_second: prefix!(exbi) * prefix!(none) / 8.0; "Eib/s", "exbibit per second",
            "exbibits per second";
        @exabit_per_second: prefix!(exa) * prefix!(none) / 8.0; "Eb/s", "exabit per second",
            "exabits per second";
        @pebibit_per_second: prefix!(pebi) * prefix!(none) / 8.0; "Pib/s", "pebibit per second",
            "pebibits per second";
        @petabit_per_second: prefix!(peta) * prefix!(none) / 8.0; "Pb/s", "petabit per second",
            "petabits per second";
        @tebibit_per_second: prefix!(tebi) * prefix!(none) / 8.0; "Tib/s", "tebibit per second",
            "tebibits per second";
        @terabit_per_second: prefix!(tera) * prefix!(none) / 8.0; "Tb/s", "terabit per second",
            "terabits per second";
        @gibibit_per_second: prefix!(gibi) * prefix!(none) / 8.0; "Gib/s", "gibibit per second",
            "gibibits per second";
        @gigabit_per_second: prefix!(giga) * prefix!(none) / 8.0; "Gb/s", "gigabit per second",
            "gigabits per second";
        @mebibit_per_second: prefix!(mebi) * prefix!(none) / 8.0; "Mib/s", "mebibit per second",
            "mebibits per second";
        @megabit_per_second: prefix!(mega) * prefix!(none) / 8.0; "Mb/s", "megabit per second",
            "megabits per second";
        @kibibit_per_second: prefix!(kibi) * prefix!(none) / 8.0; "Kib/s", "kibibit per second",
            "kibibits per second";
        @kilobit_per_second: prefix!(kilo) * prefix!(none) / 8.0; "kb/s", "kilobit per second",
            "kilobits per second";
        @bit_per_second: prefix!(none) / 8.0; "b/s", "bit per second", "bits per second";

        @yobibyte_per_second: prefix!(yobi); "YiB/s", "yobibyte per second", "yobibytes per second";
        @yottabyte_per_second: prefix!(yotta); "YB/s", "yottabyte per second",
            "yottabytes per second";
        @zebibyte_per_second: prefix!(zebi); "ZiB/s", "zebibyte per second", "zebibytes per second";
        @zettabyte_per_second: prefix!(zetta); "ZB/s", "zettabyte per second",
            "zettabytes per second";
        @exbibyte_per_second: prefix!(exbi); "EiB/s", "exbibyte per second", "exbibytes per second";
        @exabyte_per_second: prefix!(exa); "EB/s", "exabyte per second", "exabytes per second";
        @pebibyte_per_second: prefix!(pebi); "PiB/s", "pebibyte per second", "pebibytes per second";
        @petabyte_per_second: prefix!(peta); "PB/s", "petabyte per second", "petabytes per second";
        @tebibyte_per_second: prefix!(tebi); "TiB/s", "tebibyte per second", "tebibytes per second";
        @terabyte_per_second: prefix!(tera); "TB/s", "terabyte per second", "terabytes per second";
        @gibibyte_per_second: prefix!(gibi); "GiB/s", "gibibyte per second", "gibibytes per second";
        @gigabyte_per_second: prefix!(giga); "GB/s", "gigabyte per second", "gigabytes per second";
        @mebibyte_per_second: prefix!(mebi); "MiB/s", "mebibyte per second", "mebibytes per second";
        @megabyte_per_second: prefix!(mega); "MB/s", "megabyte per second", "megabytes per second";
        @kibibyte_per_second: prefix!(kibi); "KiB/s", "kibibyte per second", "kibibytes per second";
        @kilobyte_per_second: prefix!(kilo); "kB/s", "kilobyte per second", "kilobytes per second";
        @byte_per_second: prefix!(none); "B/s", "byte per second", "bytes per second";

        @octet_per_second: prefix!(none); "o/s", "octet per second", "octets per second";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::information as i;
        use si::information_rate as r;
        use si::quantities::*;
        use si::time as t;
        use tests::Test;

        #[test]
        fn check_units() {
            test::<i::yobibit, t::second, r::yobibit_per_second>();
            test::<i::yottabit, t::second, r::yottabit_per_second>();
            test::<i::zebibit, t::second, r::zebibit_per_second>();
            test::<i::zettabit, t::second, r::zettabit_per_second>();
            test::<i::exbibit, t::second, r::exbibit_per_second>();
            test::<i::exabit, t::second, r::exabit_per_second>();
            test::<i::pebibit, t::second, r::pebibit_per_second>();
            test::<i::petabit, t::second, r::petabit_per_second>();
            test::<i::tebibit, t::second, r::tebibit_per_second>();
            test::<i::terabit, t::second, r::terabit_per_second>();
            test::<i::gibibit, t::second, r::gibibit_per_second>();
            test::<i::gigabit, t::second, r::gigabit_per_second>();
            test::<i::mebibit, t::second, r::mebibit_per_second>();
            test::<i::megabit, t::second, r::megabit_per_second>();
            test::<i::kibibit, t::second, r::kibibit_per_second>();
            test::<i::kilobit, t::second, r::kilobit_per_second>();
            test::<i::bit, t::second, r::bit_per_second>();

            test::<i::yobibyte, t::second, r::yobibyte_per_second>();
            test::<i::yottabyte, t::second, r::yottabyte_per_second>();
            test::<i::zebibyte, t::second, r::zebibyte_per_second>();
            test::<i::zettabyte, t::second, r::zettabyte_per_second>();
            test::<i::exbibyte, t::second, r::exbibyte_per_second>();
            test::<i::exabyte, t::second, r::exabyte_per_second>();
            test::<i::pebibyte, t::second, r::pebibyte_per_second>();
            test::<i::petabyte, t::second, r::petabyte_per_second>();
            test::<i::tebibyte, t::second, r::tebibyte_per_second>();
            test::<i::terabyte, t::second, r::terabyte_per_second>();
            test::<i::gibibyte, t::second, r::gibibyte_per_second>();
            test::<i::gigabyte, t::second, r::gigabyte_per_second>();
            test::<i::mebibyte, t::second, r::mebibyte_per_second>();
            test::<i::megabyte, t::second, r::megabyte_per_second>();
            test::<i::kibibyte, t::second, r::kibibyte_per_second>();
            test::<i::kilobyte, t::second, r::kilobyte_per_second>();
            test::<i::byte, t::second, r::byte_per_second>();

            test::<i::octet, t::second, r::octet_per_second>();

            fn test<I: i::Conversion<V>, T: t::Conversion<V>, R: r::Conversion<V>>() {
                Test::assert_approx_eq(&InformationRate::new::<R>(V::one()),
                    &(Information::new::<I>(V::one()) / Time::new::<T>(V::one())).into());
            }
        }
    }
}
