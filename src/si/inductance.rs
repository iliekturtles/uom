//! Inductance (base unit henry, m² · kg · s⁻² · A⁻²).

quantity! {
    /// Inductance (base unit henry, m² · kg · s⁻² · A⁻²).
    quantity: Inductance; "inductance";
    /// Dimension of inductance, L²MT⁻²I⁻² (base unit henry, m² · kg · s⁻² · A⁻²).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N2,     // time
        N2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottahenry: prefix!(yotta); "YH", "yottahenry", "yottahenries";
        @zettahenry: prefix!(zetta); "ZH", "zettahenry", "zettahenries";
        @exahenry: prefix!(exa); "EH", "exahenry", "exahenries";
        @petahenry: prefix!(peta); "PH", "petahenry", "petahenries";
        @terahenry: prefix!(tera); "TH", "terahenry", "terahenries";
        @gigahenry: prefix!(giga); "GH", "gigahenry", "gigahenries";
        @megahenry: prefix!(mega); "MH", "megahenry", "megahenries";
        @kilohenry: prefix!(kilo); "kH", "kilohenry", "kilohenries";
        @hectohenry: prefix!(hecto); "hH", "hectohenry", "hectohenries";
        @decahenry: prefix!(deca); "daH", "decahenry", "decahenries";
        /// Derived unit of inductance.
        @henry: prefix!(none); "H", "henry", "henries";
        @decihenry: prefix!(deci); "dH", "decihenry", "decihenries";
        @centihenry: prefix!(centi); "cH", "centihenry", "centihenries";
        @millihenry: prefix!(milli); "mH", "millihenry", "millihenries";
        @microhenry: prefix!(micro); "µH", "microhenry", "microhenries";
        @nanohenry: prefix!(nano); "nH", "nanohenry", "nanohenries";
        @picohenry: prefix!(pico); "pH", "picohenry", "picohenries";
        @femtohenry: prefix!(femto); "fH", "femtohenry", "femtohenries";
        @attohenry: prefix!(atto); "aH", "attohenry", "attohenries";
        @zeptohenry: prefix!(zepto); "zH", "zeptohenry", "zeptohenries";
        @yoctohenry: prefix!(yocto); "yH", "yoctohenry", "yoctohenries";

        @abhenry: 1.0_E-9; "abH", "abhenry", "abhenries";
        @stathenry: 8.987_552_917_115_481_E11; "statH", "stathenry", "stathenries";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::electric_current as i;
        use si::time as t;
        use si::electric_potential as v;
        use si::inductance as l;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Inductance<V> = ElectricPotential::new::<v::volt>(V::one())
                * Time::new::<t::second>(V::one())
                / ElectricCurrent::new::<i::ampere>(V::one());
        }

        #[test]
        fn check_units() {
            test::<i::ampere, v::yottavolt, l::yottahenry>();
            test::<i::ampere, v::zettavolt, l::zettahenry>();
            test::<i::ampere, v::exavolt, l::exahenry>();
            test::<i::ampere, v::petavolt, l::petahenry>();
            test::<i::ampere, v::teravolt, l::terahenry>();
            test::<i::ampere, v::gigavolt, l::gigahenry>();
            test::<i::ampere, v::megavolt, l::megahenry>();
            test::<i::ampere, v::kilovolt, l::kilohenry>();
            test::<i::ampere, v::hectovolt, l::hectohenry>();
            test::<i::ampere, v::decavolt, l::decahenry>();
            test::<i::ampere, v::volt, l::henry>();
            test::<i::ampere, v::decivolt, l::decihenry>();
            test::<i::ampere, v::centivolt, l::centihenry>();
            test::<i::ampere, v::millivolt, l::millihenry>();
            test::<i::ampere, v::microvolt, l::microhenry>();
            test::<i::ampere, v::nanovolt, l::nanohenry>();
            test::<i::ampere, v::picovolt, l::picohenry>();
            test::<i::ampere, v::femtovolt, l::femtohenry>();
            test::<i::ampere, v::attovolt, l::attohenry>();
            test::<i::ampere, v::zeptovolt, l::zeptohenry>();
            test::<i::ampere, v::yoctovolt, l::yoctohenry>();

            test::<i::statampere, v::statvolt, l::stathenry>();
            test::<i::abampere, v::abvolt, l::abhenry>();

            fn test<I: i::Conversion<V>, U: v::Conversion<V>, L: l::Conversion<V>>() {
                Test::assert_approx_eq(&Inductance::new::<L>(V::one()),
                    &(ElectricPotential::new::<U>(V::one())
                        * Time::new::<t::second>(V::one())
                        / ElectricCurrent::new::<I>(V::one())));
            }
        }
    }
}
