//! Diffusion coefficient (base unit square meter per second, m² · s⁻¹).

quantity! {
    /// Diffusion coefficient (base unit square meter per second, m² · s⁻¹).
    quantity: DiffusionCoefficient; "diffusion coefficient";
    /// Dimension of diffusion coefficient, L²T⁻¹ (base unit square meter per second, m² · s⁻¹).
    dimension: ISQ<
        P2,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @square_meter_per_second: prefix!(none); "m²/s", "square meter per second",
            "square meters per second";
        @square_centimeter_per_second: prefix!(centi) * prefix!(centi); "cm²/s",
            "square centimeter per second", "square centimeters per second";
        @square_millimeter_per_second: prefix!(milli) * prefix!(milli); "mm²/s",
            "square millimeter per second", "square millimeters per second";
        @square_micrometer_per_second: prefix!(micro) * prefix!(micro); "µm²/s",
            "square micrometer per second", "square micrometers per second";
        @square_nanometer_per_second: prefix!(nano) * prefix!(nano); "nm²/s",
            "square nanometer per second", "square nanometers per second";
        @stokes: prefix!(centi) * prefix!(centi); "St", "Stokes", "Stokes";
        @centistokes: prefix!(centi) * prefix!(centi) * prefix!(centi); "cSt", "centistokes",
            "centistokes";
        }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::area as area;
        use crate::si::diffusion_coefficient as dc;

        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: DiffusionCoefficient<V> = Area::new::<area::square_meter>(V::one())
                / Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<area::square_meter, t::second, dc::square_meter_per_second>();
            test::<area::square_centimeter, t::second, dc::square_centimeter_per_second>();
            test::<area::square_millimeter, t::second, dc::square_millimeter_per_second>();
            test::<area::square_micrometer, t::second, dc::square_micrometer_per_second>();
            test::<area::square_nanometer, t::second, dc::square_nanometer_per_second>();
            test::<area::square_centimeter, t::second, dc::stokes>();
            test::<area::square_millimeter, t::second, dc::centistokes>();

            fn test<A: area::Conversion<V>, T: t::Conversion<V>, DC: dc::Conversion<V>>() {
                Test::assert_approx_eq(&DiffusionCoefficient::new::<DC>(V::one()),
                    &(Area::new::<A>(V::one())
                        / Time::new::<T>(V::one())));
            }
        }
    }
}
