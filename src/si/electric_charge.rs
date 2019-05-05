//! Electric charge (base unit coulomb, A · s).

quantity! {
    /// Electric charge (base unit coulomb, A · s).
    quantity: ElectricCharge; "electric charge";
    /// Dimension of electric charge, TI (base unit coulomb, A · s).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottacoulomb: prefix!(yotta); "YC", "yottacoulomb", "yottacoulombs";
        @zettacoulomb: prefix!(zetta); "ZC", "zettacoulomb", "zettacoulombs";
        @exacoulomb: prefix!(exa); "EC", "exacoulomb", "exacoulombs";
        @petacoulomb: prefix!(peta); "PC", "petacoulomb", "petacoulombs";
        @teracoulomb: prefix!(tera); "TC", "teracoulomb", "teracoulombs";
        @gigacoulomb: prefix!(giga); "GC", "gigacoulomb", "gigacoulombs";
        @megacoulomb: prefix!(mega); "MC", "megacoulomb", "megacoulombs";
        @kilocoulomb: prefix!(kilo); "kC", "kilocoulomb", "kilocoulombs";
        @hectocoulomb: prefix!(hecto); "hC", "hectocoulomb", "hectocoulombs";
        @decacoulomb: prefix!(deca); "daC", "decacoulomb", "decacoulombs";
        /// Derived unit of electric charge.
        @coulomb: prefix!(none); "C", "coulomb", "coulombs";
        @decicoulomb: prefix!(deci); "dC", "decicoulomb", "decicoulombs";
        @centicoulomb: prefix!(centi); "cC", "centicoulomb", "centicoulombs";
        @millicoulomb: prefix!(milli); "mC", "millcoulomb", "millcoulombs";
        @microcoulomb: prefix!(micro); "µC", "microcoulomb", "microcoulombs";
        @nanocoulomb: prefix!(nano); "nC", "nanocoulomb", "nanocoulombs";
        @picocoulomb: prefix!(pico); "pC", "picocoulomb", "picocoulombs";
        @femtocoulomb: prefix!(femto); "fC", "femtocoulomb", "femtocoulombs";
        @attocoulomb: prefix!(atto); "aC", "attocoulomb", "attocoulombs";
        @zeptocoulomb: prefix!(zepto); "zC", "zeptocoulomb", "zeptocoulombs";
        @yoctocoulomb: prefix!(yocto); "yC", "yoctocoulomb", "yoctocoulombs";

        @petaampere_hour: 3.6_E18; "PA · h", "petaampere hour", "petaampere hours";
        @teraampere_hour: 3.6_E15; "TA · h", "teraampere hour", "teraampere hours";
        @gigaampere_hour: 3.6_E12; "GA · h", "gigaampere hour", "gigaampere hours";
        @megaampere_hour: 3.6_E9; "MA · h", "megaampere hour", "megaampere hours";
        @kiloampere_hour: 3.6_E6; "kA · h", "kiloampere hour", "kiloampere hours";
        @hectoampere_hour: 3.6_E5; "hA · h", "hectoampere hour", "hectoampere hours";
        @decaampere_hour: 3.6_E4; "daA · h", "decaampere hour", "decaampere hours";
        @ampere_hour: 3.6_E3; "A · h", "ampere hour", "ampere hours";
        @milliampere_hour: 3.6_E0; "mA · h", "milliampere hour", "milliampere hours";
        @microampere_hour: 3.6_E-3; "µA · h", "microampere hour", "microampere hours";

        @abcoulomb: 1.0_E1; "abC", "abcoulomb", "abcoulombs";
        @faraday: 9.648_531_E4; "F", "faraday", "faradays";
        @franklin: 3.335_641_E-10; "Fr", "franklin", "franklins";
        @statcoulomb: 3.335_641_E-10; "statC", "statcoulomb", "statcoulombs";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::time as t;
        use si::electric_current as i;
        use si::electric_charge as q;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricCharge<V> = ElectricCurrent::new::<i::ampere>(V::one())
                * Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<i::yottaampere, t::second, q::yottacoulomb>();
            test::<i::zettaampere, t::second, q::zettacoulomb>();
            test::<i::exaampere, t::second, q::exacoulomb>();
            test::<i::petaampere, t::second, q::petacoulomb>();
            test::<i::teraampere, t::second, q::teracoulomb>();
            test::<i::gigaampere, t::second, q::gigacoulomb>();
            test::<i::megaampere, t::second, q::megacoulomb>();
            test::<i::kiloampere, t::second, q::kilocoulomb>();
            test::<i::hectoampere, t::second, q::hectocoulomb>();
            test::<i::decaampere, t::second, q::decacoulomb>();
            test::<i::ampere, t::second, q::coulomb>();
            test::<i::deciampere, t::second, q::decicoulomb>();
            test::<i::centiampere, t::second, q::centicoulomb>();
            test::<i::milliampere, t::second, q::millicoulomb>();
            test::<i::microampere, t::second, q::microcoulomb>();
            test::<i::nanoampere, t::second, q::nanocoulomb>();
            test::<i::picoampere, t::second, q::picocoulomb>();
            test::<i::femtoampere, t::second, q::femtocoulomb>();
            test::<i::attoampere, t::second, q::attocoulomb>();
            test::<i::zeptoampere, t::second, q::zeptocoulomb>();
            test::<i::yoctoampere, t::second, q::yoctocoulomb>();

            test::<i::ampere, t::hour, q::ampere_hour>();
            test::<i::abampere, t::second, q::abcoulomb>();
            test::<i::statampere, t::second, q::statcoulomb>();

            fn test<I: i::Conversion<V>, T: t::Conversion<V>, Q: q::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricCharge::new::<Q>(V::one()),
                    &(ElectricCurrent::new::<I>(V::one()) * Time::new::<T>(V::one())));
            }
        }
    }
}
