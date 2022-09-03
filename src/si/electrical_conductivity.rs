//! Electrical conductivity (base unit siemens per meter, m⁻³ · kg⁻¹ · s³ · A²).

quantity! {
    /// Electrical conductivity (base unit siemens per meter, m⁻³ · kg⁻¹ · s³ · A²).
    quantity: ElectricalConductivity; "electrical conductivity";
    /// Dimension of electrical conductivity, L⁻³M⁻¹T³I² (base unit siemens per meter,
    /// m⁻³ · kg⁻¹ · s³ · A²).
    dimension: ISQ<
        N3,     // length
        N1,     // mass
        P3,     // time
        P2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @siemens_per_meter: prefix!(none); "S/m", "siemens per meter", "siemens per meter";
        @siemens_per_centimeter: prefix!(none) / prefix!(centi); "S/cm", "siemens per centimeter",
            "siemens per centimeter";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::electrical_conductance as g;
        use crate::si::length as l;
        use crate::si::electrical_conductivity as ec;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricalConductivity<V> = ElectricalConductance::new::<g::siemens>(V::one())
                / Length::new::<l::meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<ec::siemens_per_meter, g::siemens, l::meter>();
            test::<ec::siemens_per_centimeter, g::siemens, l::centimeter>();

            fn test<EC: ec::Conversion<V>, G: g::Conversion<V>, L: l::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricalConductivity::new::<EC>(V::one()),
                    &(ElectricalConductance::new::<G>(V::one()) / Length::new::<L>(V::one())));
            }
        }
    }
}
