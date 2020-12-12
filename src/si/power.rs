//! Power (base unit watt, m² · kg · s⁻³).

quantity! {
    /// Power (base unit watt, m² · kg · s⁻³).
    quantity: Power; "power";
    /// Dimension of power, L²MT⁻³ (base unit watt, m² · kg · s⁻³).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N3,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottawatt: prefix!(yotta); "YW", "yottawatt", "yottawatts";
        @zettawatt: prefix!(zetta); "ZW", "zettawatt", "zettawatts";
        @exawatt: prefix!(exa); "EW", "exawatt", "exawatts";
        @petawatt: prefix!(peta); "PW", "petawatt", "petawatts";
        @terawatt: prefix!(tera); "TW", "terawatt", "terawatts";
        @gigawatt: prefix!(giga); "GW", "gigawatt", "gigawatts";
        @megawatt: prefix!(mega); "MW", "megawatt", "megawatts";
        @kilowatt: prefix!(kilo); "kW", "kilowatt", "kilowatts";
        @hectowatt: prefix!(hecto); "hW", "hectowatt", "hectowatts";
        @decawatt: prefix!(deca); "daW", "decawatt", "decawatts";
        /// Derived unit of power.
        @watt: prefix!(none); "W", "watt", "watts";
        @deciwatt: prefix!(deci); "dW", "deciwatt", "deciwatts";
        @centiwatt: prefix!(centi); "cW", "centiwatt", "centiwatts";
        @milliwatt: prefix!(milli); "mW", "milliwatt", "milliwatts";
        @microwatt: prefix!(micro); "µW", "microwatt", "microwatts";
        @nanowatt: prefix!(nano); "nW", "nanowatt", "nanowatts";
        @picowatt: prefix!(pico); "pW", "picowatt", "picowatts";
        @femtowatt: prefix!(femto); "fW", "femtowatt", "femtowatts";
        @attowatt: prefix!(atto); "aW", "attowatt", "attowatts";
        @zeptowatt: prefix!(zepto); "zW", "zeptowatt", "zeptowatts";
        @yoctowatt: prefix!(yocto); "yW", "yoctowatt", "yoctowatts";

        @decibel_watt: prefix!(none), 10.0, 10.0; "dBW", "decibel-watt", "decibel-watts";
        @decibel_milliwatt: prefix!(milli), 10.0, 10.0; "dBm", "decibel-milliwatt", "decibel-milliwatts";// dBm is more common than dBmW
        @decibel_microwatt: prefix!(micro), 10.0, 10.0; "dBµW", "decibel-microwatt", "decibel-microwatts";

        @erg_per_second: 1.0_E-7; "erg/s", "erg per second", "ergs per second";
        @foot_pound_per_hour: 3.766_161_111_111_111_E-4; "ft · lbf/h", "foot pound-force per hour",
            "foot pounds-force per hour";
        @foot_pound_per_minute: 2.259_696_666_666_666_6_E-2; "ft · lbf/min",
            "foot pound-force per minute", "foot pounds-force per minute";
        @foot_pound_per_second: 1.355_818; "ft · lbf/s", "foot pound-force per second",
            "foot pounds-force per second";
        @horsepower: 7.456_999_E2; "hp", "horsepower", "horsepower";
        @horsepower_boiler: 9.809_50_E3; "hp (S)", "horsepower (boiler)",
            "horsepower (boiler)";
        @horsepower_electric: 7.46_E2; "hp (E)", "horsepower (electric)",
            "horsepower (electric)";
        @horsepower_metric: 7.354_988_E2; "hp (M)", "metric horsepower", "metric horsepower";
        @horsepower_imperial: 7.457_0_E2; "hp (I)", "horsepower (Imperial)",
            "horsepower (Imperial)";
        @hydraulic_horsepower: 7.460_43_E2; "hp (hydraulic)", "hydraulic horsepower",
            "hydraulic horsepower";
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_dbm() {
        use crate::si::power as p;
        use crate::si::quantities::*;
        use crate::tests::Test;

        let x = Power::new::<p::decibel_milliwatt>(0.0);
        println!("{:?}", x.get::<p::watt>());
        println!("{:?}", x.get::<p::decibel_watt>());
    }

    storage_types! {
        use crate::num::One;
        use crate::si::energy as e;
        use crate::si::power as p;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: Power<V> =  Energy::new::<e::joule>(V::one())
                / Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<e::zettajoule, t::second, p::zettawatt>();
            test::<e::exajoule, t::second, p::exawatt>();
            test::<e::petajoule, t::second, p::petawatt>();
            test::<e::terajoule, t::second, p::terawatt>();
            test::<e::gigajoule, t::second, p::gigawatt>();
            test::<e::megajoule, t::second, p::megawatt>();
            test::<e::kilojoule, t::second, p::kilowatt>();
            test::<e::joule, t::second, p::watt>();
            test::<e::centijoule, t::second, p::centiwatt>();
            test::<e::millijoule, t::second, p::milliwatt>();
            test::<e::microjoule, t::second, p::microwatt>();
            test::<e::nanojoule, t::second, p::nanowatt>();
            test::<e::picojoule, t::second, p::picowatt>();
            test::<e::femtojoule, t::second, p::femtowatt>();
            test::<e::attojoule, t::second, p::attowatt>();
            test::<e::zeptojoule, t::second, p::zeptowatt>();
            test::<e::yoctojoule, t::second, p::yoctowatt>();

            test::<e::erg, t::second, p::erg_per_second>();
            test::<e::foot_pound, t::hour, p::foot_pound_per_hour>();
            test::<e::foot_pound, t::minute, p::foot_pound_per_minute>();
            test::<e::foot_pound, t::second, p::foot_pound_per_second>();

            fn test<E: e::Conversion<V>, T: t::Conversion<V>, P: p::Conversion<V>>() {
                Test::assert_approx_eq(&Power::new::<P>(V::one()),
                    &(Energy::new::<E>(V::one()) / Time::new::<T>(V::one())));
            }
        }
    }
}
