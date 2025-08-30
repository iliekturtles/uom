//! Gyromagnetic ratio (base unit hertz per tesla or coulomb per kg, kg⁻¹ · s · A).

quantity! {
    /// Gyromagnetic ratio (base unit hertz per tesla or coulomb per kg, kg⁻¹ · s · A).
    quantity: GyromagneticRatio; "gyromagnetic ratio";
    /// Dimension of gyromagnetic ratio, M⁻¹TI (base unit hertz per tesla or coulomb per kg, kg⁻¹ · s · A).
    dimension: ISQ<
        Z0,     // length
        N1,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottahertz_per_tesla: prefix!(yotta); "YHz/T", "yottahertz per tesla", "yottahertz per tesla";
        @zettahertz_per_tesla: prefix!(zetta); "ZHz/T", "zettahertz per tesla", "zettahertz per tesla";
        @exahertz_per_tesla: prefix!(exa); "EHz/T", "exahertz per tesla", "exahertz per tesla";
        @petahertz_per_tesla: prefix!(peta); "PHz/T", "petahertz per tesla", "petahertz per tesla";
        @terahertz_per_tesla: prefix!(tera); "THz/T", "terahertz per tesla", "terahertz per tesla";
        @gigahertz_per_tesla: prefix!(giga); "GHz/T", "gigahertz per tesla", "gigahertz per tesla";
        @megahertz_per_tesla: prefix!(mega); "MHz/T", "megahertz per tesla", "megahertz per tesla";
        @kilohertz_per_tesla: prefix!(kilo); "kHz/T", "kilohertz per tesla", "kilohertz per tesla";
        @hectohertz_per_tesla: prefix!(hecto); "hHz/T", "hectohertz per tesla", "hectohertz per tesla";
        @decahertz_per_tesla: prefix!(deca); "daHz/T", "decahertz per tesla", "decahertz per tesla";
        /// Derived unit of gyromagnetic ratio.
        @hertz_per_tesla: prefix!(none); "Hz/T", "hertz per tesla", "hertz per tesla";
        @decihertz_per_tesla: prefix!(deci); "dHz/T", "decihertz per tesla", "decihertz per tesla";
        @centihertz_per_tesla: prefix!(centi); "cHz/T", "centihertz per tesla", "centihertz per tesla";
        @millihertz_per_tesla: prefix!(milli); "mHz/T", "millihertz per tesla", "millihertz per tesla";
        @microhertz_per_tesla: prefix!(micro); "µHz/T", "microhertz per tesla", "microhertz per tesla";
        @nanohertz_per_tesla: prefix!(nano); "nHz/T", "nanohertz per tesla", "nanohertz per tesla";
        @picohertz_per_tesla: prefix!(pico); "pHz/T", "picohertz per tesla", "picohertz per tesla";
        @femtohertz_per_tesla: prefix!(femto); "fHz/T", "femtohertz per tesla", "femtohertz per tesla";
        @attohertz_per_tesla: prefix!(atto); "aHz/T", "attohertz per tesla", "attohertz per tesla";
        @zeptohertz_per_tesla: prefix!(zepto); "zHz/T", "zeptohertz per tesla", "zeptohertz per tesla";
        @yoctohertz_per_tesla: prefix!(yocto); "yHz/T", "yoctohertz per tesla", "yoctohertz per tesla";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::frequency as f;
        use crate::si::gyromagnetic_ratio as g;
        use crate::si::magnetic_flux_density as b;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: GyromagneticRatio<V> = Frequency::new::<f::hertz>(V::one())
                / MagneticFluxDensity::new::<b::tesla>(V::one());
        }

        #[test]
        fn check_units() {
            test::<f::yottahertz, g::yottahertz_per_tesla>();
            test::<f::zettahertz, g::zettahertz_per_tesla>();
            test::<f::exahertz, g::exahertz_per_tesla>();
            test::<f::petahertz, g::petahertz_per_tesla>();
            test::<f::terahertz, g::terahertz_per_tesla>();
            test::<f::gigahertz, g::gigahertz_per_tesla>();
            test::<f::megahertz, g::megahertz_per_tesla>();
            test::<f::kilohertz, g::kilohertz_per_tesla>();
            test::<f::hectohertz, g::hectohertz_per_tesla>();
            test::<f::decahertz, g::decahertz_per_tesla>();
            test::<f::hertz, g::hertz_per_tesla>();
            test::<f::decihertz, g::decihertz_per_tesla>();
            test::<f::centihertz, g::centihertz_per_tesla>();
            test::<f::millihertz, g::millihertz_per_tesla>();
            test::<f::microhertz, g::microhertz_per_tesla>();
            test::<f::nanohertz, g::nanohertz_per_tesla>();
            test::<f::picohertz, g::picohertz_per_tesla>();
            test::<f::femtohertz, g::femtohertz_per_tesla>();
            test::<f::attohertz, g::attohertz_per_tesla>();
            test::<f::zeptohertz, g::zeptohertz_per_tesla>();
            test::<f::yoctohertz, g::yoctohertz_per_tesla>();

            fn test<F: f::Conversion<V>, G: g::Conversion<V>>() {
                Test::assert_approx_eq(
                    &GyromagneticRatio::new::<G>(V::one()),
                    &(Frequency::new::<F>(V::one()) / MagneticFluxDensity::new::<b::tesla>(V::one())),
                );
            }
        }
    }
}
