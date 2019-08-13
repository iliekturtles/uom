//! Information (dimensionless quantity).

quantity! {
    /// Information (dimensionless quantity).
    quantity: Information; "information";
    /// Dimension of information, 1 (dimensionless).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn(::si::marker::InformationKind);
    units {
        // Base-2.
        @yobibit: prefix!(yobi) / 8.0; "Yib", "yobibit", "yobibits";
        @yottabit: prefix!(yotta) / 8.0; "Yb", "yottabit", "yottabits";
        @zebibit: prefix!(zebi) / 8.0; "Zib", "zebibit", "zebibits";
        @zettabit: prefix!(zetta) / 8.0; "Zb", "zettabit", "zettabits";
        @exbibit: prefix!(exbi) / 8.0; "Eib", "exbibit", "exbibits";
        @exabit: prefix!(exa) / 8.0; "Eb", "exabit", "exabits";
        @pebibit: prefix!(pebi) / 8.0; "Pib", "pebibit", "pebibits";
        @petabit: prefix!(peta) / 8.0; "Pb", "petabit", "petabits";
        @tebibit: prefix!(tebi) / 8.0; "Tib", "tibibit", "tibibits";
        @terabit: prefix!(tera) / 8.0; "Tb", "terabit", "terabits";
        @gibibit: prefix!(gibi) / 8.0; "Gib", "gibibit", "gibibits";
        @gigabit: prefix!(giga) / 8.0; "Gb", "gigabit", "gigabits";
        @mebibit: prefix!(mebi) / 8.0; "Mib", "mebibit", "mebibits";
        @megabit: prefix!(mega) / 8.0; "Mb", "megabit", "megabits";
        @kibibit: prefix!(kibi) / 8.0; "Kib", "kibibit", "kibibits";
        @kilobit: prefix!(kilo) / 8.0; "kb", "kilobit", "kilobits";
        /// The bit is the basic unit of information.
        ///
        /// Although the bit is the base unit of information `uom` treats it as a derived unit. The
        /// byte is treated as the base unit in order to provide more accurate conversion factors
        /// for byte derived units which are more commonly used than bits.
        @bit: prefix!(none) / 8.0; "b", "bit", "bits";

        @yobibyte: prefix!(yobi); "YiB", "yobibyte", "yobibytes";
        @yottabyte: prefix!(yotta); "YB", "yottabyte", "yottabytes";
        @zebibyte: prefix!(zebi); "ZiB", "zebibyte", "zebibytes";
        @zettabyte: prefix!(zetta); "ZB", "zettabyte", "zettabytes";
        @exbibyte: prefix!(exbi); "EiB", "exbibyte", "exbibytes";
        @exabyte: prefix!(exa); "EB", "exabyte", "exabytes";
        @pebibyte: prefix!(pebi); "PiB", "pebibyte", "pebibytes";
        @petabyte: prefix!(peta); "PB", "petabyte", "petabytes";
        @tebibyte: prefix!(tebi); "TiB", "tibibyte", "tibibytes";
        @terabyte: prefix!(tera); "TB", "terabyte", "terabytes";
        @gibibyte: prefix!(gibi); "GiB", "gibibyte", "gibibytes";
        @gigabyte: prefix!(giga); "GB", "gigabyte", "gigabytes";
        @mebibyte: prefix!(mebi); "MiB", "mebibyte", "mebibytes";
        @megabyte: prefix!(mega); "MB", "megabyte", "megabytes";
        @kibibyte: prefix!(kibi); "KiB", "kibibyte", "kibibytes";
        @kilobyte: prefix!(kilo); "kB", "kilobyte", "kilobytes";
        /// The byte is a unit of information consisting of 8 bits.
        ///
        /// Although the bit is the base unit of information `uom` treats it as a derived unit. The
        /// byte is treated as the base unit in order to provide more accurate conversion factors
        /// for byte derived units which are more commonly used than bits.
        @byte: prefix!(none); "B", "byte", "bytes";

        @octet: prefix!(none); "o", "octet", "octets";
        @nibble: prefix!(none) / 2.0; "nibble", "nibble", "nibbles";
        @crumb: prefix!(none) / 4.0; "crumb", "crumb", "crumbs";
        @shannon: prefix!(none) / 8.0; "Sh", "shannon", "shannons";

        // Base-e. ln(2).
        @natural_unit_of_information: 1.442_695_040_888_963_E0 * prefix!(none) / 8.0;
            "nat", "natural unit of uniformation", "natural units of information";

        // Base-3. log2(3).
        @trit: 1.584_962_500_721_156_E0 * prefix!(none) / 8.0; "trit", "trit", "trits";

        // Base-10. log2(10).
        @hartley: 3.321_928_094_887_363_E0 * prefix!(none) / 8.0; "Hart", "hartley", "hartleys";
        @deciban: 3.321_928_094_887_363_E0 * prefix!(deci) / 8.0; "deciban", "deciban", "decibans";
    }
}
