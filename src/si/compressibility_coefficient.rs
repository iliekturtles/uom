//! Compressibility coefficient (base unit 1 / pascal, kg⁻¹ · m · s²).

quantity! {
    /// Compressibility coefficient (base unit 1 / pascal, kg⁻¹ · m · s²).
    quantity: CompressibilityCoefficient; "compressibility coefficient";
    /// Dimension of compressibility coefficient, LM⁻¹T² (base unit 1 / pascal, kg⁻¹ · m · s²).
    dimension: ISQ<
        P1,     // length
        N1,     // mass
        P2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @per_yottapascal: prefix!(none) / prefix!(yotta);
            "YPa⁻¹", "per yottapascal", "per yottapascal";
        @per_zettapascal: prefix!(none) / prefix!(zetta);
            "ZPa⁻¹", "per zettapascal", "per zettapascal";
        @per_exapascal: prefix!(none) / prefix!(exa);
            "EPa⁻¹", "per exapascal", "per exapascal";
        @per_petapascal: prefix!(none) / prefix!(peta);
            "PPa⁻¹", "per petapascal", "per petapascal";
        @per_terapascal: prefix!(none) / prefix!(tera);
            "TPa⁻¹", "per terapascal", "per terapascal";
        @per_gigapascal: prefix!(none) / prefix!(giga);
            "GPa⁻¹", "per gigapascal", "per gigapascal";
        @per_megapascal: prefix!(none) / prefix!(mega);
            "MPa⁻¹", "per megapascal", "per megapascal";
        @per_kilopascal: prefix!(none) / prefix!(kilo);
            "kPa⁻¹", "per kilopascal", "per kilopascal";
        @per_hectopascal: prefix!(none) / prefix!(hecto);
            "hPa⁻¹", "per hectopascal", "per hectopascal";
        @per_decapascal: prefix!(none) / prefix!(deca);
            "daPa⁻¹", "per decapascal", "per decapascal";
        /// Derived unit of compressibility coefficient.
        @per_pascal: prefix!(none); "Pa⁻¹", "per pascal", "per pascal";
        @per_decipascal: prefix!(none) / prefix!(deci);
            "dPa⁻¹", "per decipascal", "per decipascal";
        @per_centipascal: prefix!(none) / prefix!(centi);
            "cPa⁻¹", "per centipascal", "per centipascal";
        @per_millipascal: prefix!(none) / prefix!(milli);
            "mPa⁻¹", "per millipascal", "per millipascal";
        @per_micropascal: prefix!(none) / prefix!(micro);
            "µPa⁻¹", "per micropascal", "per micropascal";
        @per_nanopascal: prefix!(none) / prefix!(nano);
            "nPa⁻¹", "per nanopascal", "per nanopascal";
        @per_picopascal: prefix!(none) / prefix!(pico);
            "pPa⁻¹", "per picopascal", "per picopascal";
        @per_femtopascal: prefix!(none) / prefix!(femto);
            "fPa⁻¹", "per femtopascal", "per femtopascal";
        @per_attopascal: prefix!(none) / prefix!(atto);
            "aPa⁻¹", "per attopascal", "per attopascal";
        @per_zeptopascal: prefix!(none) / prefix!(zepto);
            "zPa⁻¹", "per zeptopascal", "per zeptopascal";
        @per_yoctopascal: prefix!(none) / prefix!(yocto);
            "yPa⁻¹", "per yoctopascal", "per yoctopascal";

        @per_atmosphere: prefix!(none) / 1.013_25_E5;
            "atm⁻¹", "per atmosphere", "per atmosphere";
        @per_atmosphere_technical: prefix!(none) / 9.806_65_E4;
            "at⁻¹", "per atmosphere (technical)", "per atmosphere (technical)";
        @per_bar: prefix!(none) / 1.0_E5; "bar⁻¹", "per bar", "per bar";
        @per_millibar: prefix!(none) / 1.0_E2; "mbar⁻¹", "per millibar", "per millibar";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::pressure as p;
        use crate::si::compressibility_coefficient as cc;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: CompressibilityCoefficient<V> = V::one()
                / Pressure::new::<p::pascal>(V::one());
        }

        #[test]
        fn check_units() {
            test::<p::yottapascal, cc::per_yottapascal>();
            test::<p::zettapascal, cc::per_zettapascal>();
            test::<p::exapascal, cc::per_exapascal>();
            test::<p::petapascal, cc::per_petapascal>();
            test::<p::terapascal, cc::per_terapascal>();
            test::<p::gigapascal, cc::per_gigapascal>();
            test::<p::megapascal, cc::per_megapascal>();
            test::<p::kilopascal, cc::per_kilopascal>();
            test::<p::hectopascal, cc::per_hectopascal>();
            test::<p::decapascal, cc::per_decapascal>();
            test::<p::pascal, cc::per_pascal>();
            test::<p::decipascal, cc::per_decipascal>();
            test::<p::centipascal, cc::per_centipascal>();
            test::<p::millipascal, cc::per_millipascal>();
            test::<p::micropascal, cc::per_micropascal>();
            test::<p::nanopascal, cc::per_nanopascal>();
            test::<p::picopascal, cc::per_picopascal>();
            test::<p::femtopascal, cc::per_femtopascal>();
            test::<p::attopascal, cc::per_attopascal>();
            test::<p::zeptopascal, cc::per_zeptopascal>();
            test::<p::yoctopascal, cc::per_yoctopascal>();

            test::<p::atmosphere, cc::per_atmosphere>();
            test::<p::atmosphere_technical, cc::per_atmosphere_technical>();
            test::<p::bar, cc::per_bar>();
            test::<p::millibar, cc::per_millibar>();

            fn test<P: p::Conversion<V>, CC: cc::Conversion<V>>() {
                Test::assert_approx_eq(&CompressibilityCoefficient::new::<CC>(V::one()),
                    &(V::one()
                        / Pressure::new::<P>(V::one())));
            }
        }
    }
}
