//! Amount of substance (base unit mole, mol<sup>1</sup>).

quantity! {
    /// Amount of substance (base unit mole, mol<sup>1</sup>).
    quantity: AmountOfSubstance; "amount of substance";
    /// Amount of substance dimension, mol<sup>1</sup>.
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        P1,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottamole: prefix!(yotta); "Ymol", "yottamole", "yottamoles";
        @zettamole: prefix!(zetta); "Zmol", "zettamole", "zettamoles";
        @examole: prefix!(exa); "Emol", "examole", "examoles";
        @petamole: prefix!(peta); "Pmol", "petamole", "petamoles";
        @teramole: prefix!(tera); "Tmol", "teramole", "teramoles";
        @gigamole: prefix!(giga); "Gmol", "gigamole", "gigamoles";
        @megamole: prefix!(mega); "Mmol", "megamole", "megamoles";
        @kilomole: prefix!(kilo); "kmol", "kilomole", "kilomoles";
        @hectomole: prefix!(hecto); "hmol", "hectomole", "hectomoles";
        @decamole: prefix!(deca); "damol", "decamole", "decamoles";
        /// 1. The mole is the amount of substance of a system which contains as many elementary
        ///    entities as there are atoms in 0.012 kilogram of carbon 12.
        /// 2. When the mole is used, the elementary entities must be specified and may be atmons,
        ///    molecules, ions, electrons, other particles, or specified groups of such particles.
        @mole: prefix!(none); "mol", "mole", "moles";
        @decimole: prefix!(deci); "dmol", "decimole", "decimoles";
        @centimole: prefix!(centi); "cmol", "centimole", "centimoles";
        @millimole: prefix!(milli); "mmol", "millimole", "millimoles";
        @micromole: prefix!(micro); "µmol", "micromole", "micromoles";
        @nanomole: prefix!(nano); "nmol", "nanomole", "nanomoles";
        @picomole: prefix!(pico); "pmol", "picomole", "picomoles";
        @femtomole: prefix!(femto); "fmol", "femtomole", "femtomoles";
        @attomole: prefix!(atto); "amol", "attomole", "attomoles";
        @zeptomole: prefix!(zepto); "zmol", "zeptomole", "zeptomoles";
        @yoctomole: prefix!(yocto); "ymol", "yoctomole", "yoctomoles";
    }
}
