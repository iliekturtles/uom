//! Luminous intensity (base unit candela, cd^(1)).

use si::ISQ;

quantity! {
    /// Luminous intensity (base unit candela, cd^(1)).
    quantity: LuminousIntensity;
    /// Luminous intensity dimension, cd^(1).
    dimension: ISQ<
        ::typenum::Z0,  // length
        ::typenum::Z0,  // mass
        ::typenum::Z0,  // time
        ::typenum::Z0,  // electric current
        ::typenum::Z0,  // thermodynamic temperature
        ::typenum::Z0,  // amount of substance
        ::typenum::P1>; // luminous intensity
    units {
        yottacandela: prefix!(yotta);
        zettacandela: prefix!(zetta);
        exacandela: prefix!(exa);
        petacandela: prefix!(peta);
        teracandela: prefix!(tera);
        megacandela: prefix!(mega);
        kilocandela: prefix!(kilo);
        hectocandela: prefix!(hecto);
        decacandela: prefix!(deca);
        candela: prefix!(none);
        decicandela: prefix!(deci);
        centicandela: prefix!(centi);
        millicandela: prefix!(milli);
        microcandela: prefix!(micro);
        nanocandela: prefix!(nano);
        picocandela: prefix!(pico);
        femtocandela: prefix!(femto);
        attocandela: prefix!(atto);
        zeptocandela: prefix!(zepto);
        yoctocandela: prefix!(yocto);
    }
}
