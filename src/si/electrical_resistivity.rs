//! Electrical resistivity (base unit ohm meter, m³ · kg · s⁻³ · A⁻²).

quantity! {
    /// Electrical resistivity (base unit ohm meter, m³ · kg · s⁻³ · A⁻²).
    quantity: ElectricalResistivity; "electrical resistivity";
    /// Dimension of electrical resistivity, L³MT⁻³I⁻² (base unit ohm meter, m³ · kg · s⁻³ · A⁻²).
    dimension: ISQ<
        P3,     // length
        P1,     // mass
        N3,     // time
        N2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottaohm_meter: prefix!(yotta); "YΩ · m", "yottaohm meter", "yottaohm meters";
        @zettaohm_meter: prefix!(zetta); "ZΩ · m", "zettaohm meter", "zettaohm meters";
        @exaohm_meter: prefix!(exa); "EΩ · m", "exaohm meter", "exaohm meters";
        @petaohm_meter: prefix!(peta); "PΩ · m", "petaohm meter", "petaohm meters";
        @teraohm_meter: prefix!(tera); "TΩ · m", "teraohm meter", "teraohm meters";
        @gigaohm_meter: prefix!(giga); "GΩ · m", "gigaohm meter", "gigaohm meters";
        @megaohm_meter: prefix!(mega); "MΩ · m", "megaohm meter", "megaohm meters";
        @kiloohm_meter: prefix!(kilo); "kΩ · m", "kiloohm meter", "kiloohm meters";
        @hectoohm_meter: prefix!(hecto); "hΩ · m", "hectoohm meter", "hectoohm meters";
        @decaohm_meter: prefix!(deca); "daΩ · m", "decaohm meter", "decaohm meters";
        /// Derived unit of electrical resistance.
        @ohm_meter: prefix!(none); "Ω · m", "ohm meter", "ohm meters";
        @deciohm_meter: prefix!(deci); "dΩ · m", "deciohm meter", "deciohm meters";
        @centiohm_meter: prefix!(centi); "cΩ · m", "centiohm meter", "centiohm meters";
        @milliohm_meter: prefix!(milli); "mΩ · m", "milliohm meter", "milliohm meters";
        @microohm_meter: prefix!(micro); "µΩ · m", "microohm meter", "microohm meters";
        @nanoohm_meter: prefix!(nano); "nΩ · m", "nanoohm meter", "nanoohm meters";
        @picoohm_meter: prefix!(pico); "pΩ · m", "picoohm meter", "picoohm meters";
        @femtoohm_meter: prefix!(femto); "fΩ · m", "femtoohm meter", "femtoohm meters";
        @attoohm_meter: prefix!(atto); "aΩ · m", "attoohm meter", "attoohm meters";
        @zeptoohm_meter: prefix!(zepto); "zΩ · m", "zeptoohm meter", "zeptoohm meters";
        @yoctoohm_meter: prefix!(yocto); "yΩ · m", "yoctoohm meter", "yoctoohm meters";

        @abohm_meter: 1.0_E-9; "abΩ · m", "abohm meter", "abohm meters";
        @statohm_meter: 8.987_552_917_115_481_E11; "statΩ · m", "statohm meter", "statohm meters";

        @ohm_centimeter: prefix!(none) * prefix!(centi); "Ω · cm", "ohm centimeter",
            "ohm centimeters";
        @abohm_centimeter: 1.0_E-9 * prefix!(centi); "abΩ · cm", "abohm centimeter",
            "abohm centimeters";
        @statohm_centimeter: 8.987_552_917_115_481_E11 * prefix!(centi); "statΩ · cm",
            "statohm centimeter", "statohm centimeters";

        @ohm_inch: 2.54_E-2; "Ω · in", "ohm inch", "ohm inches";
        @ohm_foot: 3.048_E-1; "Ω · ft", "ohm foot", "ohm feet";
        @ohm_yard: 9.144_E-1; "Ω · yd", "ohm yard", "ohm yards";

        @ohm_square_millimeter_per_meter: prefix!(milli) * prefix!(milli); "Ω · mm²/m",
            "ohm square millimeter per meter", "ohm square millimeters per meter";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::electrical_resistance as r;
        use crate::si::electrical_resistivity as er;
        use crate::si::length as l;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricalResistivity<V> = ElectricalResistance::new::<r::ohm>(V::one())
                * Length::new::<l::meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<er::yottaohm_meter, r::yottaohm, l::meter>();
            test::<er::zettaohm_meter, r::zettaohm, l::meter>();
            test::<er::petaohm_meter, r::petaohm, l::meter>();
            test::<er::teraohm_meter, r::teraohm, l::meter>();
            test::<er::gigaohm_meter, r::gigaohm, l::meter>();
            test::<er::megaohm_meter, r::megaohm, l::meter>();
            test::<er::kiloohm_meter, r::kiloohm, l::meter>();
            test::<er::hectoohm_meter, r::hectoohm, l::meter>();
            test::<er::decaohm_meter, r::decaohm, l::meter>();
            test::<er::ohm_meter, r::ohm, l::meter>();
            test::<er::deciohm_meter, r::deciohm, l::meter>();
            test::<er::centiohm_meter, r::centiohm, l::meter>();
            test::<er::milliohm_meter, r::milliohm, l::meter>();
            test::<er::microohm_meter, r::microohm, l::meter>();
            test::<er::nanoohm_meter, r::nanoohm, l::meter>();
            test::<er::picoohm_meter, r::picoohm, l::meter>();
            test::<er::femtoohm_meter, r::femtoohm, l::meter>();
            test::<er::attoohm_meter, r::attoohm, l::meter>();
            test::<er::zeptoohm_meter, r::zeptoohm, l::meter>();
            test::<er::yoctoohm_meter, r::yoctoohm, l::meter>();

            test::<er::abohm_meter, r::abohm, l::meter>();
            test::<er::statohm_meter, r::statohm, l::meter>();

            test::<er::ohm_centimeter, r::ohm, l::centimeter>();
            test::<er::abohm_centimeter, r::abohm, l::centimeter>();
            test::<er::statohm_centimeter, r::statohm, l::centimeter>();

            test::<er::ohm_inch, r::ohm, l::inch>();
            test::<er::ohm_foot, r::ohm, l::foot>();
            test::<er::ohm_yard, r::ohm, l::yard>();

            test::<er::ohm_square_millimeter_per_meter, r::microohm, l::meter>();

            fn test<ER: er::Conversion<V>, R: r::Conversion<V>, L: l::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricalResistivity::new::<ER>(V::one()),
                    &(ElectricalResistance::new::<R>(V::one())
                        * Length::new::<L>(V::one())));
            }
        }
    }
}
