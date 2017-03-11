//! Electric current (base unit ampere, A^(1)).

quantity! {
    /// Electric current (base unit ampere, A^(1)).
    quantity: ElectricCurrent;
    /// Amount of substance dimension, mol^(1).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottaampere: prefix!(yotta);
        @zettaampere: prefix!(zetta);
        @exaampere: prefix!(exa);
        @petaampere: prefix!(peta);
        @teraampere: prefix!(tera);
        @megaampere: prefix!(mega);
        @kiloampere: prefix!(kilo);
        @hectoampere: prefix!(hecto);
        @decaampere: prefix!(deca);
        /// The ampere is the constant current which, if maintained in two straight parallel
        /// conductors of infinite length, of negligible circular cross-section, and placed 1 meter
        /// apart in vacuum, would produce between these conductors a force equal to 2E-7 newton per
        /// meter of length.
        @ampere: prefix!(none);
        @deciampere: prefix!(deci);
        @centiampere: prefix!(centi);
        @milliampere: prefix!(milli);
        @microampere: prefix!(micro);
        @nanoampere: prefix!(nano);
        @picoampere: prefix!(pico);
        @femtoampere: prefix!(femto);
        @attoampere: prefix!(atto);
        @zeptoampere: prefix!(zepto);
        @yoctoampere: prefix!(yocto);
    }
}
