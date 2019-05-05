//! Magnetic flux density (base unit tesla, kg · s⁻² · A⁻¹).

quantity! {
    /// Magnetic flux density (base unit tesla, kg · s⁻² · A⁻¹).
    quantity: MagneticFluxDensity; "magnetic flux density";
    /// Dimension of magnetic flux density, MT⁻²I⁻¹ (base unit tesla, kg · s⁻² · A⁻¹).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        N2,     // time
        N1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottatesla: prefix!(yotta); "YT", "yottatesla", "yottateslas";
        @zettatesla: prefix!(zetta); "ZT", "zettatesla", "zettateslas";
        @exatesla: prefix!(exa); "ET", "exatesla", "exateslas";
        @petatesla: prefix!(peta); "PT", "petatesla", "petateslas";
        @teratesla: prefix!(tera); "TT", "teratesla", "terateslas";
        @gigatesla: prefix!(giga); "GT", "gigatesla", "gigateslas";
        @megatesla: prefix!(mega); "MT", "megatesla", "megateslas";
        @kilotesla: prefix!(kilo); "kT", "kilotesla", "kiloteslas";
        @hectotesla: prefix!(hecto); "hT", "hectotesla", "hectoteslas";
        @decatesla: prefix!(deca); "daT", "decatesla", "decateslas";
        /// Derived unit of magnetic flux density.
        @tesla: prefix!(none); "T", "tesla", "teslas";
        @decitesla: prefix!(deci); "dT", "decitesla", "deciteslas";
        @centitesla: prefix!(centi); "cT", "centitesla", "centiteslas";
        @millitesla: prefix!(milli); "mT", "millitesla", "milliteslas";
        @microtesla: prefix!(micro); "µT", "microtesla", "microteslas";
        @nanotesla: prefix!(nano); "nT", "nanotesla", "nanoteslas";
        @picotesla: prefix!(pico); "pT", "picotesla", "picoteslas";
        @femtotesla: prefix!(femto); "fT", "femtotesla", "femtoteslas";
        @attotesla: prefix!(atto); "aT", "attotesla", "attoteslas";
        @zeptotesla: prefix!(zepto); "zT", "zeptotesla", "zeptoteslas";
        @yoctotesla: prefix!(yocto); "yT", "yoctotesla", "yoctoteslas";

        @gamma: 1.0_E-9; "γ", "gamma", "gammas";
        @gauss: 1.0_E-4; "G", "gauss", "gauss";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::area as a;
        use si::magnetic_flux as f;
        use si::magnetic_flux_density as b;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: MagneticFluxDensity<V> = MagneticFlux::new::<f::weber>(V::one())
                / Area::new::<a::square_meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<f::yottaweber, b::yottatesla>();
            test::<f::zettaweber, b::zettatesla>();
            test::<f::exaweber, b::exatesla>();
            test::<f::petaweber, b::petatesla>();
            test::<f::teraweber, b::teratesla>();
            test::<f::gigaweber, b::gigatesla>();
            test::<f::megaweber, b::megatesla>();
            test::<f::kiloweber, b::kilotesla>();
            test::<f::hectoweber, b::hectotesla>();
            test::<f::decaweber, b::decatesla>();
            test::<f::weber, b::tesla>();
            test::<f::deciweber, b::decitesla>();
            test::<f::centiweber, b::centitesla>();
            test::<f::milliweber, b::millitesla>();
            test::<f::microweber, b::microtesla>();
            test::<f::nanoweber, b::nanotesla>();
            test::<f::picoweber, b::picotesla>();
            test::<f::femtoweber, b::femtotesla>();
            test::<f::attoweber, b::attotesla>();
            test::<f::zeptoweber, b::zeptotesla>();
            test::<f::yoctoweber, b::yoctotesla>();

            test::<f::nanoweber, b::gamma>();

            fn test<F: f::Conversion<V>, B: b::Conversion<V>>() {
                Test::assert_approx_eq(&MagneticFluxDensity::new::<B>(V::one()),
                    &(MagneticFlux::new::<F>(V::one())
                        / Area::new::<a::square_meter>(V::one())));
            }
        }
    }
}
