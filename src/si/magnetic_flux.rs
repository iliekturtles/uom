//! Magnetic flux (base unit weber, m² · kg · s⁻² · A⁻¹).

quantity! {
    /// Magnetic flux (base unit weber, m² · kg · s⁻² · A⁻¹).
    quantity: MagneticFlux; "magnetic flux";
    /// Dimension of magnetic flux, L²MT⁻²I⁻¹ (base unit weber, m² · kg · s⁻² · A⁻¹).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N2,     // time
        N1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottaweber: prefix!(yotta); "YWb", "yottaweber", "yottawebers";
        @zettaweber: prefix!(zetta); "ZWb", "zettaweber", "zettawebers";
        @exaweber: prefix!(exa); "EWb", "exaweber", "exawebers";
        @petaweber: prefix!(peta); "PWb", "petaweber", "petawebers";
        @teraweber: prefix!(tera); "TWb", "teraweber", "terawebers";
        @gigaweber: prefix!(giga); "GWb", "gigaweber", "gigawebers";
        @megaweber: prefix!(mega); "MWb", "megaweber", "megawebers";
        @kiloweber: prefix!(kilo); "kWb", "kiloweber", "kilowebers";
        @hectoweber: prefix!(hecto); "hWb", "hectoweber", "hectowebers";
        @decaweber: prefix!(deca); "daWb", "decaweber", "decawebers";
        /// Derived unit of magnetic flux.
        @weber: prefix!(none); "Wb", "weber", "webers";
        @deciweber: prefix!(deci); "dWb", "deciweber", "deciwebers";
        @centiweber: prefix!(centi); "cWb", "centiweber", "centiwebers";
        @milliweber: prefix!(milli); "mWb", "milliweber", "milliwebers";
        @microweber: prefix!(micro); "µWb", "microweber", "microwebers";
        @nanoweber: prefix!(nano); "nWb", "nanoweber", "nanowebers";
        @picoweber: prefix!(pico); "pWb", "picoweber", "picowebers";
        @femtoweber: prefix!(femto); "fWb", "femtoweber", "femtowebers";
        @attoweber: prefix!(atto); "aWb", "attoweber", "attowebers";
        @zeptoweber: prefix!(zepto); "zWb", "zeptoweber", "zeptowebers";
        @yoctoweber: prefix!(yocto); "yWb", "yoctoweber", "yoctowebers";

        @maxwell: 1.0_E-8; "Mx", "maxwell", "maxwells";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::time as t;
        use si::electric_potential as v;
        use si::magnetic_flux as f;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: MagneticFlux<V> = ElectricPotential::new::<v::volt>(V::one())
                * Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<v::yottavolt, f::yottaweber>();
            test::<v::zettavolt, f::zettaweber>();
            test::<v::exavolt, f::exaweber>();
            test::<v::petavolt, f::petaweber>();
            test::<v::teravolt, f::teraweber>();
            test::<v::gigavolt, f::gigaweber>();
            test::<v::megavolt, f::megaweber>();
            test::<v::kilovolt, f::kiloweber>();
            test::<v::hectovolt, f::hectoweber>();
            test::<v::decavolt, f::decaweber>();
            test::<v::volt, f::weber>();
            test::<v::decivolt, f::deciweber>();
            test::<v::centivolt, f::centiweber>();
            test::<v::millivolt, f::milliweber>();
            test::<v::microvolt, f::microweber>();
            test::<v::nanovolt, f::nanoweber>();
            test::<v::picovolt, f::picoweber>();
            test::<v::femtovolt, f::femtoweber>();
            test::<v::attovolt, f::attoweber>();
            test::<v::zeptovolt, f::zeptoweber>();
            test::<v::yoctovolt, f::yoctoweber>();

            fn test<U: v::Conversion<V>, F: f::Conversion<V>>() {
                Test::assert_approx_eq(&MagneticFlux::new::<F>(V::one()),
                    &(ElectricPotential::new::<U>(V::one())
                        * Time::new::<t::second>(V::one())));
            }
        }
    }
}
