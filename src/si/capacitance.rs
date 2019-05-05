//! Capacitance (base unit farad, m⁻² · kg⁻¹ · s⁴ · A²).

quantity! {
    /// Capacitance (base unit farad, m⁻² · kg⁻¹ · s⁴ · A²).
    quantity: Capacitance; "capacitance";
    /// Dimension of capacitance, L⁻²M⁻¹T⁴I² (base unit farad, m⁻² · kg⁻¹ · s⁴ · A²).
    dimension: ISQ<
        N2,     // length
        N1,     // mass
        P4,     // time
        P2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottafarad: prefix!(yotta); "YF", "yottafarad", "yottafarads";
        @zettafarad: prefix!(zetta); "ZF", "zettafarad", "zettafarads";
        @exafarad: prefix!(exa); "EF", "exafarad", "exafarads";
        @petafarad: prefix!(peta); "PF", "petafarad", "petafarads";
        @terafarad: prefix!(tera); "TF", "terafarad", "terafarads";
        @gigafarad: prefix!(giga); "GF", "gigafarad", "gigafarads";
        @megafarad: prefix!(mega); "MF", "megafarad", "megafarads";
        @kilofarad: prefix!(kilo); "kF", "kilofarad", "kilofarads";
        @hectofarad: prefix!(hecto); "hF", "hectofarad", "hectofarads";
        @decafarad: prefix!(deca); "daF", "decafarad", "decafarads";
        /// Derived unit of capacitance.
        @farad: prefix!(none); "F", "farad", "farads";
        @decifarad: prefix!(deci); "dF", "decifarad", "decifarads";
        @centifarad: prefix!(centi); "cF", "centifarad", "centifarads";
        @millifarad: prefix!(milli); "mF", "millifarad", "millifarads";
        @microfarad: prefix!(micro); "µF", "microfarad", "microfarads";
        @nanofarad: prefix!(nano); "nF", "nanofarad", "nanofarads";
        @picofarad: prefix!(pico); "pF", "picofarad", "picofarads";
        @femtofarad: prefix!(femto); "fF", "femtofarad", "femtofarads";
        @attofarad: prefix!(atto); "aF", "attofarad", "attofarads";
        @zeptofarad: prefix!(zepto); "zF", "zeptofarad", "zeptofarads";
        @yoctofarad: prefix!(yocto); "yF", "yoctofarad", "yoctofarads";

        @abfarad: 1.0_E9; "abF", "abfarad", "abfarads";
        @statfarad: 1.112_650_E-12; "statF", "statfarad", "statfarads";
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
        use si::capacitance as c;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Capacitance<V> = ElectricCurrent::new::<i::ampere>(V::one())
                * Time::new::<t::second>(V::one())
                / ElectricPotential::new::<v::volt>(V::one());
        }

        #[test]
        fn check_units() {
            test::<i::yottaampere, v::volt, c::yottafarad>();
            test::<i::zettaampere, v::volt, c::zettafarad>();
            test::<i::exaampere, v::volt, c::exafarad>();
            test::<i::petaampere, v::volt, c::petafarad>();
            test::<i::teraampere, v::volt, c::terafarad>();
            test::<i::gigaampere, v::volt, c::gigafarad>();
            test::<i::megaampere, v::volt, c::megafarad>();
            test::<i::kiloampere, v::volt, c::kilofarad>();
            test::<i::hectoampere, v::volt, c::hectofarad>();
            test::<i::decaampere, v::volt, c::decafarad>();
            test::<i::ampere, v::volt, c::farad>();
            test::<i::deciampere, v::volt, c::decifarad>();
            test::<i::centiampere, v::volt, c::centifarad>();
            test::<i::milliampere, v::volt, c::millifarad>();
            test::<i::microampere, v::volt, c::microfarad>();
            test::<i::nanoampere, v::volt, c::nanofarad>();
            test::<i::picoampere, v::volt, c::picofarad>();
            test::<i::femtoampere, v::volt, c::femtofarad>();
            test::<i::attoampere, v::volt, c::attofarad>();
            test::<i::zeptoampere, v::volt, c::zeptofarad>();
            test::<i::yoctoampere, v::volt, c::yoctofarad>();

            test::<i::statampere, v::statvolt, c::statfarad>();
            test::<i::abampere, v::abvolt, c::abfarad>();

            fn test<I: i::Conversion<V>, U: v::Conversion<V>, C: c::Conversion<V>>() {
                Test::assert_approx_eq(&Capacitance::new::<C>(V::one()),
                    &(ElectricCurrent::new::<I>(V::one())
                        * Time::new::<t::second>(V::one())
                        / ElectricPotential::new::<U>(V::one())));
            }
        }
    }
}
