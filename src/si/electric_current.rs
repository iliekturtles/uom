//! Electric current (base unit ampere, A<sup>1</sup>).

quantity! {
    /// Electric current (base unit ampere, A<sup>1</sup>).
    quantity: ElectricCurrent; "electric current";
    /// Electric current dimension, A<sup>1</sup>.
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottaampere: prefix!(yotta); "YA", "yottaampere", "yottaamperes";
        @zettaampere: prefix!(zetta); "ZA", "zettaampere", "zettaamperes";
        @exaampere: prefix!(exa); "EA", "exaampere", "exaamperes";
        @petaampere: prefix!(peta); "PA", "petaampere", "petaamperes";
        @teraampere: prefix!(tera); "TA", "teraampere", "teraamperes";
        @gigaampere: prefix!(giga); "GA", "gigaampere", "gigaamperes";
        @megaampere: prefix!(mega); "MA", "megaampere", "megaamperes";
        @kiloampere: prefix!(kilo); "kA", "kiloampere", "kiloamperes";
        @hectoampere: prefix!(hecto); "hA", "hectoampere", "hectoamperes";
        @decaampere: prefix!(deca); "daA", "decaampere", "decaamperes";
        /// The ampere is the constant current which, if maintained in two straight parallel
        /// conductors of infinite length, of negligible circular cross-section, and placed 1 meter
        /// apart in vacuum, would produce between these conductors a force equal to 2E-7 newton per
        /// meter of length.
        @ampere: prefix!(none); "A", "ampere", "amperes";
        @deciampere: prefix!(deci); "dA", "deciampere", "deciamperes";
        @centiampere: prefix!(centi); "cA", "centiampere", "centiamperes";
        @milliampere: prefix!(milli); "mA", "millampere", "millamperes";
        @microampere: prefix!(micro); "A", "ampere", "amperes";
        @nanoampere: prefix!(nano); "A", "ampere", "amperes";
        @picoampere: prefix!(pico); "A", "ampere", "amperes";
        @femtoampere: prefix!(femto); "A", "ampere", "amperes";
        @attoampere: prefix!(atto); "A", "ampere", "amperes";
        @zeptoampere: prefix!(zepto); "A", "ampere", "amperes";
        @yoctoampere: prefix!(yocto); "A", "ampere", "amperes";
    }
}
