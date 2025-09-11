//! Electric Flux (base unit volt meter, m³ ⋅ kg ⋅ s⁻³ ⋅ A⁻¹).

quantity! {
    /// Electric Flux (base unit volt meter, m³ ⋅ kg ⋅ s⁻³ ⋅ A⁻¹).
    quantity: ElectricFlux; "electric flux";
    /// Dimension of electric flux, L³MT⁻³I⁻¹ (base unit volt meter, m³ ⋅ kg ⋅ s⁻³ ⋅ A⁻¹).
    dimension: ISQ<
        P3,     // length
        P1,     // mass
        N3,     // time
        N1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @volt_meter: prefix!(none); "V ⋅ m", "volt meter", "volt meters";
        @volt_centimeter: prefix!(none) * prefix!(centi); "V ⋅ cm", "volt centimeter",
            "volt centimeters";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::electric_flux as ef;
        use crate::si::quantities::*;
        use crate::si::electric_potential as ep;
        use crate::si::length as l;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ElectricFlux<V> = ElectricPotential::new::<ep::volt>(V::one())
                * Length::new::<l::meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<ef::volt_meter, l::meter, ep::volt>();
            test::<ef::volt_centimeter, l::centimeter, ep::volt>();

            fn test<EF: ef::Conversion<V>, L: l::Conversion<V>, EP: ep::Conversion<V>>() {
                Test::assert_approx_eq(&ElectricFlux::new::<EF>(V::one()),
                    &(ElectricPotential::new::<EP>(V::one())
                        * Length::new::<L>(V::one())));
            }
        }
    }
}
