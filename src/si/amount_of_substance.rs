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
        @yottamole: prefix!(yotta);
        @zettamole: prefix!(zetta);
        @examole: prefix!(exa);
        @petamole: prefix!(peta);
        @teramole: prefix!(tera);
        @megamole: prefix!(mega);
        @kilomole: prefix!(kilo);
        @hectomole: prefix!(hecto);
        @decamole: prefix!(deca);
        /// 1. The mole is the amount of substance of a system which contains as many elementary
        ///    entities as there are atoms in 0.012 kilogram of carbon 12.
        /// 2. When the mole is used, the elementary entities must be specified and may be atmons,
        ///    molecules, ions, electrons, other particles, or specified groups of such particles.
        @mole: prefix!(none);
        @decimole: prefix!(deci);
        @centimole: prefix!(centi);
        @millimole: prefix!(milli);
        @micromole: prefix!(micro);
        @nanomole: prefix!(nano);
        @picomole: prefix!(pico);
        @femtomole: prefix!(femto);
        @attomole: prefix!(atto);
        @zeptomole: prefix!(zepto);
        @yoctomole: prefix!(yocto);
    }
}
