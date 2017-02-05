//! Mass (base unit kilogram, kg^(1)).

use si::ISQ;

quantity! {
    /// Mass (base unit kilogram, kg^(1)).
    quantity: Mass;
    /// Mass dimension, kg^(1).
    dimension: ISQ<
        ::typenum::Z0,  // length
        ::typenum::P1,  // mass
        ::typenum::Z0,  // time
        ::typenum::Z0,  // electric current
        ::typenum::Z0,  // thermodynamic temperature
        ::typenum::Z0,  // amount of substance
        ::typenum::Z0>; // luminous intensity
    units {
        yottagram: prefix!(yotta) / prefix!(kilo);
        zettagram: prefix!(zetta) / prefix!(kilo);
        exagram: prefix!(exa) / prefix!(kilo);
        petagram: prefix!(peta) / prefix!(kilo);
        teragram: prefix!(tera) / prefix!(kilo);
        megagram: prefix!(mega) / prefix!(kilo);
        kilogram: prefix!(kilo) / prefix!(kilo);
        hectogram: prefix!(hecto) / prefix!(kilo);
        decagram: prefix!(deca) / prefix!(kilo);
        gram: prefix!(none) / prefix!(kilo);
        decigram: prefix!(deci) / prefix!(kilo);
        centigram: prefix!(centi) / prefix!(kilo);
        milligram: prefix!(milli) / prefix!(kilo);
        microgram: prefix!(micro) / prefix!(kilo);
        nanogram: prefix!(nano) / prefix!(kilo);
        picogram: prefix!(pico) / prefix!(kilo);
        femtogram: prefix!(femto) / prefix!(kilo);
        attogram: prefix!(atto) / prefix!(kilo);
        zeptogram: prefix!(zepto) / prefix!(kilo);
        yoctogram: prefix!(yocto) / prefix!(kilo);
    }
}
