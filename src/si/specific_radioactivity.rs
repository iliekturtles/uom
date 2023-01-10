//! Specific radioactivity (base unit becquerel per kilogram, kg⁻¹ · s⁻¹).

quantity! {
    /// Specific radioactivity (base unit becquerel per kilogram, kg⁻¹ · s⁻¹).
    quantity: SpecificRadioactivity; "specific radioactivity";
    /// Dimension of specific radioactivity, M⁻¹T⁻¹ (base unit becquerel per kilogram, kg⁻¹ · s⁻¹).
    dimension: ISQ<
        Z0,     // length
        N1,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::ConstituentConcentrationKind);
    units {
        @becquerel_per_kilogram: prefix!(none); "Bq/kg", "becquerel per kilogram",
            "becquerels per kilogram";

        @curie_per_kilogram: 3.7_E10; "Ci/kg", "curie per kilogram", "curie per kilogram";

        @disintegrations_per_minute_per_kilogram: 1.0 / 6.0_E1; "dpm/kg",
            "disintegration per minute per kilogram", "disintegrations per minute per kilogram";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::radioactivity as rad;
        use crate::si::specific_radioactivity as srad;
        use crate::si::quantities::*;
        use crate::si::mass as m;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: SpecificRadioactivity<V> = (Radioactivity::new::<rad::becquerel>(V::one())
                / Mass::new::<m::kilogram>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<rad::becquerel, m::kilogram, srad::becquerel_per_kilogram>();
            test::<rad::curie, m::kilogram, srad::curie_per_kilogram>();
            test::<rad::disintegrations_per_minute, m::kilogram,
                srad::disintegrations_per_minute_per_kilogram>();

            fn test<RAD: rad::Conversion<V>, M: m::Conversion<V>, SRAD: srad::Conversion<V>>() {
                Test::assert_approx_eq(&SpecificRadioactivity::new::<SRAD>(V::one()),
                    &(Radioactivity::new::<RAD>(V::one()) / Mass::new::<M>(V::one())).into());
            }
        }
    }
}
