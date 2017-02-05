//! Amount of substance (base unit mole, mol^(1)).

quantity! {
    /// Amount of substance (base unit mole, mol^(1)).
    quantity: AmountOfSubstance;
    /// Amount of substance dimension, mol^(1).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        P1,     // amount of substance
        Z0>;    // luminous intensity
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
