//! Electric potential (base unit volt, m² · kg · s⁻³ · A⁻¹).

quantity! {
    /// Electric potential (base unit volt, m² · kg · s⁻³ · A⁻¹).
    quantity: ElectricPotential; "electric potential";
    /// Dimension of electric potential, L²MT⁻³I⁻¹ (base unit volt, m² · kg · s⁻³ · A⁻¹).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N3,     // time
        N1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottavolt: prefix!(yotta); "YV", "yottavolt", "yottavolts";
        @zettavolt: prefix!(zetta); "ZV", "zettavolt", "zettavolts";
        @exavolt: prefix!(exa); "EV", "exavolt", "exavolts";
        @petavolt: prefix!(peta); "PV", "petavolt", "petavolts";
        @teravolt: prefix!(tera); "TV", "teravolt", "teravolts";
        @gigavolt: prefix!(giga); "GV", "gigavolt", "gigavolts";
        @megavolt: prefix!(mega); "MV", "megavolt", "megavolts";
        @kilovolt: prefix!(kilo); "kV", "kilovolt", "kilovolts";
        @hectovolt: prefix!(hecto); "hV", "hectovolt", "hectovolts";
        @decavolt: prefix!(deca); "daV", "decavolt", "decavolts";
        /// Derived unit of electric potential.
        @volt: prefix!(none); "V", "volt", "volts";
        @decivolt: prefix!(deci); "dV", "decivolt", "decivolts";
        @centivolt: prefix!(centi); "cV", "centivolt", "centivolts";
        @millivolt: prefix!(milli); "mV", "millivolt", "millivolts";
        @microvolt: prefix!(micro); "µV", "microvolt", "microvolts";
        @nanovolt: prefix!(nano); "nV", "nanovolt", "nanovolts";
        @picovolt: prefix!(pico); "pV", "picovolt", "picovolts";
        @femtovolt: prefix!(femto); "fV", "femtovolt", "femtovolts";
        @attovolt: prefix!(atto); "aV", "attovolt", "attovolts";
        @zeptovolt: prefix!(zepto); "zV", "zeptovolt", "zeptovolts";
        @yoctovolt: prefix!(yocto); "yV", "yoctovolt", "yoctovolts";

        @abvolt: 1.0_E-8; "abV", "abvolt", "abvolts";
        @statvolt: 2.997_925_E2; "statV", "statvolt", "statvolts";

        @decibel_volt: prefix!(none), 10.0, 20.0; "dBV", "decibel-volt", "decibel-volts";
        @decibel_millivolt: prefix!(milli), 10.0, 20.0; "dBmV", "decibel-millivolt", "decibel-millivolts";
        @decibel_microvolt: prefix!(micro), 10.0, 20.0; "dBµV", "decibel-microvolt", "decibel-microvolts";
        @decibel_unit: 0.7746E0, 10.0, 20.0; "dBu", "decibel-unit", "decibel-units";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::area as a;
        use crate::si::electric_current as i;
        use crate::si::electric_potential as v;
        use crate::si::mass as m;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricPotential<V> = Area::new::<a::square_meter>(V::one())
                * Mass::new::<m::kilogram>(V::one())
                / (ElectricCurrent::new::<i::ampere>(V::one())
                    * (Time::new::<t::second>(V::one()) * Time::new::<t::second>(V::one())
                        * Time::new::<t::second>(V::one())));
        }

        #[test]
        fn check_units() {
            test::<i::yottaampere, v::yoctovolt>();
            test::<i::zettaampere, v::zeptovolt>();
            test::<i::exaampere, v::attovolt>();
            test::<i::petaampere, v::femtovolt>();
            test::<i::teraampere, v::picovolt>();
            test::<i::gigaampere, v::nanovolt>();
            test::<i::megaampere, v::microvolt>();
            test::<i::kiloampere, v::millivolt>();
            test::<i::hectoampere, v::centivolt>();
            test::<i::decaampere, v::decivolt>();
            test::<i::ampere, v::volt>();
            test::<i::deciampere, v::decavolt>();
            test::<i::centiampere, v::hectovolt>();
            test::<i::milliampere, v::kilovolt>();
            test::<i::microampere, v::megavolt>();
            test::<i::nanoampere, v::gigavolt>();
            test::<i::picoampere, v::teravolt>();
            test::<i::femtoampere, v::petavolt>();
            test::<i::attoampere, v::exavolt>();
            test::<i::zeptoampere, v::zettavolt>();
            test::<i::yoctoampere, v::yottavolt>();

            fn test<I: i::Conversion<V>, E: v::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricPotential::new::<E>(V::one()),
                    &(Area::new::<a::square_meter>(V::one()) * Mass::new::<m::kilogram>(V::one())
                        / (ElectricCurrent::new::<I>(V::one())
                            * (Time::new::<t::second>(V::one()) * Time::new::<t::second>(V::one())
                                * Time::new::<t::second>(V::one())))));
            }
        }
    }
}
