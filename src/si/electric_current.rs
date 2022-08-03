//! Electric current (base unit ampere, A).

quantity! {
    /// Electric current (base unit ampere, A).
    quantity: ElectricCurrent; "electric current";
    /// Dimension of electric current, I (base unit ampere, A).
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
        /// The ampere is the SI unit of electric current. It is defined by taking the fixed
        /// numerical value of the elementary charge *e* to be 1.602 176 634 × 10⁻¹⁹ when expressed
        /// in the unit C, which is equal to A s, where the second is defined in terms of
        /// ∆*ν*<sub>Cs</sub>.
        @ampere: prefix!(none); "A", "ampere", "amperes";
        @deciampere: prefix!(deci); "dA", "deciampere", "deciamperes";
        @centiampere: prefix!(centi); "cA", "centiampere", "centiamperes";
        @milliampere: prefix!(milli); "mA", "millampere", "millamperes";
        @microampere: prefix!(micro); "µA", "microampere", "microamperes";
        @nanoampere: prefix!(nano); "nA", "nanoampere", "nanoamperes";
        @picoampere: prefix!(pico); "pA", "picoampere", "picoamperes";
        @femtoampere: prefix!(femto); "fA", "femtoampere", "femtoamperes";
        @attoampere: prefix!(atto); "aA", "attoampere", "attoamperes";
        @zeptoampere: prefix!(zepto); "zA", "zeptoampere", "zeptoamperes";
        @yoctoampere: prefix!(yocto); "yA", "yoctoampere", "yoctoamperes";

        /// Atomic unit of charge (electric charge carried by a single proton) per second.
        @elementary_charge_per_second: 1.602_176_634_E-19; "e/s", "elementary charge per second",
            "elementary charges per second";
        @atomic_unit_of_charge_per_second: 1.602_176_634_E-19; "a.u. of charge/s",
            "atomic unit of charge per second", "atomic units of charge per second";
        @abampere: 1.0_E1; "abA", "abampere", "abamperes";
        @gilbert: 7.957_747_E-1; "Gi", "gilbert", "gilberts";
        @statampere: 3.335_641_E-10; "statA", "statampere", "statamperes";
    }
}
