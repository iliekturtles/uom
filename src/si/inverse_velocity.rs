//! Inverse velocity (base unit second per meter, s · m⁻¹).

quantity! {
    /// Inverse velocity (base unit second per meter, s · m⁻¹).
    quantity: InverseVelocity; "inverse velocity";
    /// Dimension of inverse velocity, TL⁻¹ (base unit second per meter, s · m⁻¹).
    dimension: ISQ<
        N1,     // length
        Z0,     // mass
        P1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottasecond_per_meter: prefix!(yotta); "Ys/m", "yottasecond per meter",
            "yottaseconds per meter";
        @zettasecond_per_meter: prefix!(zetta); "Zs/m", "zettasecond per meter",
            "zettaseconds per meter";
        @exasecond_per_meter: prefix!(exa); "Es/m", "exasecond per meter", "exaseconds per meter";
        @petasecond_per_meter: prefix!(peta); "Ps/m", "petasecond per meter",
            "petaseconds per meter";
        @terasecond_per_meter: prefix!(tera); "Ts/m", "terasecond per meter",
            "teraseconds per meter";
        @gigasecond_per_meter: prefix!(giga); "Gs/m", "gigasecond per meter",
            "gigaseconds per meter";
        @megasecond_per_meter: prefix!(mega); "Ms/m", "megasecond per meter",
            "megaseconds per meter";
        @kilosecond_per_meter: prefix!(kilo); "ks/m", "kilosecond per meter",
            "kiloseconds per meter";
        @hectosecond_per_meter: prefix!(hecto); "hs/m", "hectosecond per meter",
            "hectoseconds per meter";
        @decasecond_per_meter: prefix!(deca); "das/m", "decasecond per meter",
            "decaseconds per meter";
        @second_per_meter: prefix!(none); "s/m", "second per meter", "seconds per meter";
        @decisecond_per_meter: prefix!(deci); "ds/m", "decisecond per meter",
            "deciseconds per meter";
        @centisecond_per_meter: prefix!(centi); "cs/m", "centisecond per meter",
            "centiseconds per meter";
        @millisecond_per_meter: prefix!(milli); "ms/m", "millisecond per meter",
            "milliseconds per meter";
        @microsecond_per_meter: prefix!(micro); "µs/m", "microsecond per meter",
            "microseconds per meter";
        @nanosecond_per_meter: prefix!(nano); "ns/m", "nanosecond per meter",
            "nanoseconds per meter";
        @picosecond_per_meter: prefix!(pico); "ps/m", "picosecond per meter",
            "picoseconds per meter";
        @femtosecond_per_meter: prefix!(femto); "fs/m", "femtosecond per meter",
            "femtoseconds per meter";
        @attosecond_per_meter: prefix!(atto); "as/m", "attosecond per meter",
            "attoseconds per meter";
        @zeptosecond_per_meter: prefix!(zepto); "zs/m", "zeptosecond per meter",
            "zeptoseconds per meter";
        @yoctosecond_per_meter: prefix!(yocto); "ys/m", "yoctosecond per meter",
            "yoctoseconds per meter";

        @hour_per_foot: 1.181_102_362_204_724_3_E4; "h/ft", "hour per foot", "hours per foot";
        @minute_per_foot: 1.968_503_937_007_873_8_E2; "min/ft", "minute per foot",
            "minutes per foot";
        @second_per_foot: 3.280_839_895_013_123; "s/ft", "second per foot", "seconds per foot";
        @second_per_inch: 3.937_007_874_015_748_E1; "s/in", "second per inch", "seconds per inch";
        @minute_per_kilometer: 6_E-2; "min/km", "minute per kilometer", "minutes per kilometer";
        @hour_per_kilometer: 3.6; "h/km", "hour per kilometer", "hours per kilometer";
        @hour_per_mile: 2.236_936_292_054_402; "h/mi", "hour per mile", "hours per mile";
        @minute_per_mile: 3.728_227_153_E-2; "min/mi", "minute per mile", "minutes per mile";
        @second_per_mile: 6.213_711_922_373_339_E-4; "s/mi", "second per mile", "seconds per mile";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::length as l;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::inverse_velocity as v;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: InverseVelocity<V> = Time::new::<t::second>(V::one())
                / Length::new::<l::meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<l::meter, t::yottasecond, v::yottasecond_per_meter>();
            test::<l::meter, t::zettasecond, v::zettasecond_per_meter>();
            test::<l::meter, t::exasecond, v::exasecond_per_meter>();
            test::<l::meter, t::petasecond, v::petasecond_per_meter>();
            test::<l::meter, t::terasecond, v::terasecond_per_meter>();
            test::<l::meter, t::gigasecond, v::gigasecond_per_meter>();
            test::<l::meter, t::megasecond, v::megasecond_per_meter>();
            test::<l::meter, t::kilosecond, v::kilosecond_per_meter>();
            test::<l::meter, t::hectosecond, v::hectosecond_per_meter>();
            test::<l::meter, t::decasecond, v::decasecond_per_meter>();
            test::<l::meter, t::second, v::second_per_meter>();
            test::<l::meter, t::decisecond, v::decisecond_per_meter>();
            test::<l::meter, t::centisecond, v::centisecond_per_meter>();
            test::<l::meter, t::millisecond, v::millisecond_per_meter>();
            test::<l::meter, t::microsecond, v::microsecond_per_meter>();
            test::<l::meter, t::nanosecond, v::nanosecond_per_meter>();
            test::<l::meter, t::picosecond, v::picosecond_per_meter>();
            test::<l::meter, t::femtosecond, v::femtosecond_per_meter>();
            test::<l::meter, t::attosecond, v::attosecond_per_meter>();
            test::<l::meter, t::zeptosecond, v::zeptosecond_per_meter>();
            test::<l::meter, t::yoctosecond, v::yoctosecond_per_meter>();

            test::<l::foot, t::hour, v::hour_per_foot>();
            test::<l::foot, t::minute, v::minute_per_foot>();
            test::<l::foot, t::second, v::second_per_foot>();
            test::<l::inch, t::second, v::second_per_inch>();
            test::<l::kilometer, t::minute, v::minute_per_kilometer>();
            test::<l::kilometer, t::hour, v::hour_per_kilometer>();
            test::<l::mile, t::hour, v::hour_per_mile>();
            test::<l::mile, t::second, v::second_per_mile>();

            fn test<L: l::Conversion<V>, T: t::Conversion<V>, E: v::Conversion<V>>() {
                Test::assert_eq(&InverseVelocity::new::<E>(V::one()),
                    &(Time::new::<T>(V::one()) / Length::new::<L>(V::one())));
            }
        }
    }
}
