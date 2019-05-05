//! Electrical resistance (base unit ohm, m² · kg · s⁻³ · A⁻²).

quantity! {
    /// Electrical resistance (base unit ohm, m² · kg · s⁻³ · A⁻²).
    quantity: ElectricalResistance; "electrical resistance";
    /// Dimension of electrical resistance, L²MT⁻³I⁻² (base unit ohm, m² · kg · s⁻³ · A⁻²).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N3,     // time
        N2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottaohm: prefix!(yotta); "YΩ", "yottaohm", "yottaohms";
        @zettaohm: prefix!(zetta); "ZΩ", "zettaohm", "zettaohms";
        @exaohm: prefix!(exa); "EΩ", "exaohm", "exaohms";
        @petaohm: prefix!(peta); "PΩ", "petaohm", "petaohms";
        @teraohm: prefix!(tera); "TΩ", "teraohm", "teraohms";
        @gigaohm: prefix!(giga); "GΩ", "gigaohm", "gigaohms";
        @megaohm: prefix!(mega); "MΩ", "megaohm", "megaohms";
        @kiloohm: prefix!(kilo); "kΩ", "kiloohm", "kiloohms";
        @hectoohm: prefix!(hecto); "hΩ", "hectoohm", "hectoohms";
        @decaohm: prefix!(deca); "daΩ", "decaohm", "decaohms";
        /// Derived unit of electrical resistance.
        @ohm: prefix!(none); "Ω", "ohm", "ohms";
        @deciohm: prefix!(deci); "dΩ", "deciohm", "deciohms";
        @centiohm: prefix!(centi); "cΩ", "centiohm", "centiohms";
        @milliohm: prefix!(milli); "mΩ", "milliohm", "milliohms";
        @microohm: prefix!(micro); "µΩ", "microohm", "microohms";
        @nanoohm: prefix!(nano); "nΩ", "nanoohm", "nanoohms";
        @picoohm: prefix!(pico); "pΩ", "picoohm", "picoohms";
        @femtoohm: prefix!(femto); "fΩ", "femtoohm", "femtoohms";
        @attoohm: prefix!(atto); "aΩ", "attoohm", "attoohms";
        @zeptoohm: prefix!(zepto); "zΩ", "zeptoohm", "zeptoohms";
        @yoctoohm: prefix!(yocto); "yΩ", "yoctoohm", "yoctoohms";

        @abohm: 1.0_E-9; "abΩ", "abohm", "abohms";
        @statohm: 8.987_552_917_115_481_E11; "statΩ", "statohm", "statohms";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::electric_current as i;
        use si::electric_potential as v;
        use si::electrical_resistance as r;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricalResistance<V> = ElectricPotential::new::<v::volt>(V::one())
                / ElectricCurrent::new::<i::ampere>(V::one());
        }

        #[test]
        fn check_units() {
            test::<v::yottavolt, i::ampere, r::yottaohm>();
            test::<v::zettavolt, i::ampere, r::zettaohm>();
            test::<v::exavolt, i::ampere, r::exaohm>();
            test::<v::petavolt, i::ampere, r::petaohm>();
            test::<v::teravolt, i::ampere, r::teraohm>();
            test::<v::gigavolt, i::ampere, r::gigaohm>();
            test::<v::megavolt, i::ampere, r::megaohm>();
            test::<v::kilovolt, i::ampere, r::kiloohm>();
            test::<v::hectovolt, i::ampere, r::hectoohm>();
            test::<v::decavolt, i::ampere, r::decaohm>();
            test::<v::volt, i::ampere, r::ohm>();
            test::<v::decivolt, i::ampere, r::deciohm>();
            test::<v::centivolt, i::ampere, r::centiohm>();
            test::<v::millivolt, i::ampere, r::milliohm>();
            test::<v::microvolt, i::ampere, r::microohm>();
            test::<v::nanovolt, i::ampere, r::nanoohm>();
            test::<v::picovolt, i::ampere, r::picoohm>();
            test::<v::femtovolt, i::ampere, r::femtoohm>();
            test::<v::attovolt, i::ampere, r::attoohm>();
            test::<v::zeptovolt, i::ampere, r::zeptoohm>();
            test::<v::yoctovolt, i::ampere, r::yoctoohm>();

            test::<v::abvolt, i::abampere, r::abohm>();
            test::<v::statvolt, i::statampere, r::statohm>();

            fn test<U: v::Conversion<V>, I: i::Conversion<V>, R: r::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricalResistance::new::<R>(V::one()),
                    &(ElectricPotential::new::<U>(V::one())
                        / ElectricCurrent::new::<I>(V::one())));
            }
        }
    }
}
