//! Amount of substance (base unit mole, mol).

quantity! {
    /// Amount of substance (base unit mole, mol).
    quantity: AmountOfSubstance; "amount of substance";
    /// Dimension of amount of substance, N (base unit mole, mol).
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
        /// 1. The mole is the SI unit of amount of substance. One mole contains exactly
        ///    6.022 140 76 × 10²³ elementary entities. This number is the fixed numerical value of
        ///    the Avogadro constant, *N*<sub>A</sub>, when expressed in the unit mol⁻¹ and is
        ///    called the Avogadro number.
        /// 2. The amount of substance, symbol *n*, of a system is a measure of the number of
        ///    specified elementary entities. An elementary entity may be an atom, a molecule, an
        ///    ion, an electron, any other particle or specified group of particles.
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
