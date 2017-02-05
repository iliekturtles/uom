//! Amount of substance (base unit mole, mol^(1)).

use si::ISQ;

quantity! {
    /// Amount of substance (base unit mole, mol^(1)).
    quantity: AmountOfSubstance;
    /// Amount of substance dimension, mol^(1).
    dimension: ISQ<
        ::typenum::Z0,  // length
        ::typenum::Z0,  // mass
        ::typenum::Z0,  // time
        ::typenum::Z0,  // electric current
        ::typenum::Z0,  // thermodynamic temperature
        ::typenum::P1,  // amount of substance
        ::typenum::Z0>; // luminous intensity
    units {
        yottamole: prefix!(yotta);
        zettamole: prefix!(zetta);
        examole: prefix!(exa);
        petamole: prefix!(peta);
        teramole: prefix!(tera);
        megamole: prefix!(mega);
        kilomole: prefix!(kilo);
        hectomole: prefix!(hecto);
        decamole: prefix!(deca);
        mole: prefix!(none);
        decimole: prefix!(deci);
        centimole: prefix!(centi);
        millimole: prefix!(milli);
        micromole: prefix!(micro);
        nanomole: prefix!(nano);
        picomole: prefix!(pico);
        femtomole: prefix!(femto);
        attomole: prefix!(atto);
        zeptomole: prefix!(zepto);
        yoctomole: prefix!(yocto);
    }
}
