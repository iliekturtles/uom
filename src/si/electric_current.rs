//! Electric current (base unit ampere, A^(1)).

use si::ISQ;

quantity! {
    /// Electric current (base unit ampere, A^(1)).
    quantity: ElectricCurrent;
    /// Amount of substance dimension, mol^(1).
    dimension: ISQ<
        ::typenum::Z0,  // length
        ::typenum::Z0,  // mass
        ::typenum::Z0,  // time
        ::typenum::P1,  // electric current
        ::typenum::Z0,  // thermodynamic temperature
        ::typenum::Z0,  // amount of substance
        ::typenum::Z0>; // luminous intensity
    units {
        yottaampere: prefix!(yotta);
        zettaampere: prefix!(zetta);
        exaampere: prefix!(exa);
        petaampere: prefix!(peta);
        teraampere: prefix!(tera);
        megaampere: prefix!(mega);
        kiloampere: prefix!(kilo);
        hectoampere: prefix!(hecto);
        decaampere: prefix!(deca);
        ampere: prefix!(none);
        deciampere: prefix!(deci);
        centiampere: prefix!(centi);
        milliampere: prefix!(milli);
        microampere: prefix!(micro);
        nanoampere: prefix!(nano);
        picoampere: prefix!(pico);
        femtoampere: prefix!(femto);
        attoampere: prefix!(atto);
        zeptoampere: prefix!(zepto);
        yoctoampere: prefix!(yocto);
    }
}
