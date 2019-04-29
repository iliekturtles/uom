//! Electrical conductance (base unit siemens, m⁻² · kg⁻¹ · s³ · A²).

quantity! {
    /// Electrical conductance (base unit siemens, m⁻² · kg⁻¹ · s³ · A²).
    quantity: ElectricalConductance; "electrical conductance";
    /// Dimension of electrical conductance, L⁻²M⁻¹T³I² (base unit siemens, m⁻² · kg⁻¹ · s³ · A²).
    dimension: ISQ<
        N2,     // length
        N1,     // mass
        P3,     // time
        P2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottasiemens: prefix!(yotta); "YS", "yottasiemens", "yottasiemens";
        @zettasiemens: prefix!(zetta); "ZS", "zettasiemens", "zettasiemens";
        @exasiemens: prefix!(exa); "ES", "exasiemens", "exasiemens";
        @petasiemens: prefix!(peta); "PS", "petasiemens", "petasiemens";
        @terasiemens: prefix!(tera); "TS", "terasiemens", "terasiemens";
        @gigasiemens: prefix!(giga); "GS", "gigasiemens", "gigasiemens";
        @megasiemens: prefix!(mega); "MS", "megasiemens", "megasiemens";
        @kilosiemens: prefix!(kilo); "kS", "kilosiemens", "kilosiemens";
        @hectosiemens: prefix!(hecto); "hS", "hectosiemens", "hectosiemens";
        @decasiemens: prefix!(deca); "daS", "decasiemens", "decasiemens";
        /// Derived unit of electrical conductance.
        @siemens: prefix!(none); "S", "siemens", "siemens";
        @mho: prefix!(none); "℧", "mho", "mhos";
        @decisiemens: prefix!(deci); "dS", "decisiemens", "decisiemens";
        @centisiemens: prefix!(centi); "cS", "centisiemens", "centisiemens";
        @millisiemens: prefix!(milli); "mS", "millisiemens", "millisiemens";
        @microsiemens: prefix!(micro); "µS", "microsiemens", "microsiemens";
        @nanosiemens: prefix!(nano); "nS", "nanosiemens", "nanosiemens";
        @picosiemens: prefix!(pico); "pS", "picosiemens", "picosiemens";
        @femtosiemens: prefix!(femto); "fS", "femtosiemens", "femtosiemens";
        @attosiemens: prefix!(atto); "aS", "attosiemens", "attosiemens";
        @zeptosiemens: prefix!(zepto); "zS", "zeptosiemens", "zeptosiemens";
        @yoctosiemens: prefix!(yocto); "yS", "yoctosiemens", "yoctosiemens";

        @abmho: 1.0_E9; "abmho", "abmho", "abmhos";
        @absiemens: 1.0_E9; "abS", "abmsiemens", "abmsiemens";
        @statsiemens: 1.112_650_E-12; "statS", "statsiemens", "statsiemens";
        @statmho: 1.112_650_E-12; "statmho", "statmho", "statmhos";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::electrical_resistance as r;
        use si::electrical_conductance as g;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricalConductance<V> = V::one()
                / ElectricalResistance::new::<r::ohm>(V::one());
        }

        #[test]
        fn check_units() {
            test::<r::yoctoohm, g::yottasiemens>();
            test::<r::zeptoohm, g::zettasiemens>();
            test::<r::attoohm, g::exasiemens>();
            test::<r::femtoohm, g::petasiemens>();
            test::<r::picoohm, g::terasiemens>();
            test::<r::nanoohm, g::gigasiemens>();
            test::<r::microohm, g::megasiemens>();
            test::<r::milliohm, g::kilosiemens>();
            test::<r::centiohm, g::hectosiemens>();
            test::<r::deciohm, g::decasiemens>();
            test::<r::ohm, g::siemens>();
            test::<r::decaohm, g::decisiemens>();
            test::<r::hectoohm, g::centisiemens>();
            test::<r::kiloohm, g::millisiemens>();
            test::<r::megaohm, g::microsiemens>();
            test::<r::gigaohm, g::nanosiemens>();
            test::<r::teraohm, g::picosiemens>();
            test::<r::petaohm, g::femtosiemens>();
            test::<r::exaohm, g::attosiemens>();
            test::<r::zettaohm, g::zeptosiemens>();
            test::<r::yottaohm, g::yoctosiemens>();

            test::<r::abohm, g::absiemens>();
            test::<r::statohm, g::statsiemens>();
            test::<r::abohm, g::abmho>();
            test::<r::statohm, g::statmho>();

            fn test<R: r::Conversion<V>, G: g::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricalConductance::new::<G>(V::one()),
                    &(V::one() / ElectricalResistance::new::<R>(V::one())));
            }
        }
    }
}
