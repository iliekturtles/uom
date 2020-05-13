//! Length (base unit meter, m).

quantity! {
    /// Length (base unit meter, m).
    quantity: Length; "length";
    /// Dimension of length, L (base unit meter, m).
    dimension: ISQ<
        P1,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottameter: prefix!(yotta); "Ym", "yottameter", "yottameters";
        @zettameter: prefix!(zetta); "Zm", "zettameter", "zettameters";
        @exameter: prefix!(exa); "Em", "exameter", "exameters";
        @petameter: prefix!(peta); "Pm", "petameter", "petameters";
        @terameter: prefix!(tera); "Tm", "terameter", "terameters";
        @gigameter: prefix!(giga); "Gm", "gigameter", "gigameters";
        @megameter: prefix!(mega); "Mm", "megameter", "megameters";
        @kilometer: prefix!(kilo); "km", "kilometer", "kilometers";
        @hectometer: prefix!(hecto); "hm", "hectometer", "hectometers";
        @decameter: prefix!(deca); "dam", "decameter", "decameters";
        /// The meter is the SI unit of length. It is defined by taking the fixed numerical value
        /// of the speed of light in vacuum *c* to be 299 792 458 when expressed in the unit m s⁻¹,
        /// where the second is defined in terms of the caesium frequency ∆*ν*<sub>Cs</sub>.
        @meter: prefix!(none); "m", "meter", "meters";
        @decimeter: prefix!(deci); "dm", "decimeter", "decimeters";
        @centimeter: prefix!(centi); "cm", "centimeter", "centimeters";
        @millimeter: prefix!(milli); "mm", "millimeter", "millimeters";
        @micrometer: prefix!(micro); "µm", "micrometer", "micrometers";
        @nanometer: prefix!(nano); "nm", "nanometer", "nanometers";
        @picometer: prefix!(pico); "pm", "picometer", "picometers";
        @femtometer: prefix!(femto); "fm", "femtometer", "femtometers";
        @attometer: prefix!(atto); "am", "attometer", "attometers";
        @zeptometer: prefix!(zepto); "zm", "zeptometer", "zeptometers";
        @yoctometer: prefix!(yocto); "ym", "yoctometer", "yoctometers";

        @angstrom: 1.0_E-10; "Å", "ångström", "ångströms";
        @astronomical_unit: 1.495_979_E11; "ua", "astronomical unit", "astronomical units";
        @chain: 2.011_684_E1; "ch", "chain", "chains";
        @fathom: 1.828_804_E0; "fathom", "fathom", "fathoms";
        @fermi: 1.0_E-15; "fermi", "fermi", "fermis";
        @foot: 3.048_E-1; "ft", "foot", "feet";
        @foot_survey: 3.048_006_E-1; "ft (U.S. survey)", "foot (U.S. survey)", "feet (U.S. survey)";
        @inch: 2.54_E-2; "in", "inch", "inches";
        @light_year: 9.460_73_E15; "l. y.", "light year", "light years";
        @microinch: 2.54_E-8; "μin", "microinch", "microinches";
        @micron: 1.0_E-6; "μ", "micron", "microns";
        @mil: 2.54_E-5; "0.001 in", "mil", "mils";
        @mile: 1.609_344_E3; "mi", "mile", "miles";
        @mile_survey: 1.609_347_E3; "mi (U.S. survey)", "mile (U.S. survey)", "miles (U.S. survey)";
        @nautical_mile: 1.852_E3; "M", "nautical mile", "nautical miles";
        @parsec: 3.085_678_E16; "pc", "parsec", "parsecs";
        @pica_computer: 4.233_333_333_333_333_E-3; "1/6 in (computer)", "pica (computer)",
            "picas (computer)";
        @pica_printers: 4.217_518_E-3; "1/6 in", "pica (printer's)", "picas (printer's)";
        @point_computer: 3.527_778_E-4; "1/72 in (computer)", "point (computer)",
            "points (computer)";
        @point_printers: 3.514_598_E-4; "1/72 in", "point (printer's)", "points (printer's)";
        @rod: 5.029_21_E0; "rd", "rod", "rods";
        @yard: 9.144_E-1; "yd", "yard", "yards";
    }
}
