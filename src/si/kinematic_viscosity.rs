//! Kinematic viscosity (base unit square meter per second, m² · s⁻¹).

quantity! {
    /// Kinematic viscosity (base unit square meter per second, m² · s⁻¹).
    quantity: KinematicViscosity; "kinematic viscosity";
    /// Dimension of kinematic viscosity, L²T⁻¹ (base unit square meter per second, m² · s⁻¹).
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::KinematicViscosityKind);
    units {
        @square_meter_per_second: prefix!(none);
            "m²/s", "square meter per second", "square meters per second";
        @square_centimeter_per_second: prefix!(centi) * prefix!(centi);
            "cm²/s", "square centimeter per second", "square centimeters per second";
        @square_millimeter_per_second: prefix!(milli) * prefix!(milli);
            "mm²/s", "square millimeter per second", "square millimeters per second";
        @square_micrometer_per_second: prefix!(micro) * prefix!(micro);
            "µm²/s", "square micrometer per second", "square micrometers per second";
        @square_nanometer_per_second: prefix!(nano) * prefix!(nano);
            "nm²/s", "square nanometer per second", "square nanometers per second";
        @kilostokes: prefix!(kilo) * prefix!(centi) * prefix!(centi);
            "kSt", "kilostokes", "kilostokes";
        @stokes: prefix!(centi) * prefix!(centi); "St", "stokes", "stokes";
        @decistokes: prefix!(deci) * prefix!(centi) * prefix!(centi);
            "dSt", "decistokes", "decistokes";
        @centistokes: prefix!(centi) * prefix!(centi) * prefix!(centi);
            "cSt", "centistokes", "centistokes";
        @millistokes: prefix!(milli) * prefix!(centi) * prefix!(centi);
            "mSt", "millistokes", "millistokes";
        @microstokes: prefix!(micro) * prefix!(centi) * prefix!(centi);
            "µSt", "microstokes", "microstokes";
        @nanostokes: prefix!(nano) * prefix!(centi) * prefix!(centi);
            "nSt", "nanostokes", "nanostokes";
        }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::area as a;
        use crate::si::kinematic_viscosity as kv;
        use crate::si::quantities::*;
        use crate::si::time as t;

        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: KinematicViscosity<V> = (Area::new::<a::square_meter>(V::one())
                / Time::new::<t::second>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<a::square_meter, t::second, kv::square_meter_per_second>();
            test::<a::square_centimeter, t::second, kv::square_centimeter_per_second>();
            test::<a::square_millimeter, t::second, kv::square_millimeter_per_second>();
            test::<a::square_micrometer, t::second, kv::square_micrometer_per_second>();
            test::<a::square_nanometer, t::second, kv::square_nanometer_per_second>();
            test::<a::square_centimeter, t::millisecond, kv::kilostokes>();
            test::<a::square_centimeter, t::second, kv::stokes>();
            test::<a::square_centimeter, t::decasecond, kv::decistokes>();
            test::<a::square_centimeter, t::hectosecond, kv::centistokes>();
            test::<a::square_centimeter, t::kilosecond, kv::millistokes>();
            test::<a::square_centimeter, t::megasecond, kv::microstokes>();
            test::<a::square_centimeter, t::gigasecond, kv::nanostokes>();

            fn test<A: a::Conversion<V>, T: t::Conversion<V>, KV: kv::Conversion<V>>() {
                Test::assert_approx_eq(&KinematicViscosity::new::<KV>(V::one()),
                    &(Area::new::<A>(V::one()) / Time::new::<T>(V::one())).into());
            }
        }
    }
}
