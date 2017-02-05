//! Luminous intensity (base unit candela, cd^(1)).

quantity! {
    /// Luminous intensity (base unit candela, cd^(1)).
    quantity: LuminousIntensity;
    /// Luminous intensity dimension, cd^(1).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        P1>;    // luminous intensity
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
