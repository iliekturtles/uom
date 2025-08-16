//! Surface electric current density (base unit ampere per meter, ). 

quantity! {
    /// Surface electric current density (base unit ampere per meter, m⁻¹ · A).
    quantity: SurfaceElectricCurrentDensity; "surface electric current density";
    /// Dimension of surface electric current density, IL⁻¹ (base unit ampere per meter, m⁻¹ · A).
    dimension: ISQ<
        N1,     // length
        Z0,     // mass
        Z0,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ConstituentConcentrationKind;
    units {
        @ampere_per_meter: prefix!(none); "A/m", "ampere per meter", "amperes per meter";
        @ampere_per_centimeter: prefix!(none) / prefix!(centi) ; "A/cm", "ampere per centimeter",
            "amperes per centimeter";
        @ampere_per_millimeter: prefix!(none) / prefix!(milli) ; "A/mm", "ampere per millimeter",
            "amperes per millimeter";
        @ampere_per_micrometer: prefix!(none) / prefix!(micro) ; "A/μm", "ampere per micrometer",
            "amperes per micrometer";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::length as l;
        use crate::si::electric_current as i;
        use crate::si::surface_electric_current_density as ecd;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: SurfaceElectricCurrentDensity<V> = (ElectricCurrent::new::<i::ampere>(V::one())
                / Length::new::<l::meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<i::ampere, l::meter, ecd::ampere_per_meter>();
            test::<i::ampere, l::centimeter, ecd::ampere_per_centimeter>();
            test::<i::ampere, l::millimeter, ecd::ampere_per_millimeter>();
            test::<i::ampere, l::micrometer, ecd::ampere_per_micrometer>();

            fn test<I: i::Conversion<V>, L: l::Conversion<V>, ECD: ecd::Conversion<V>>() {
                Test::assert_approx_eq(&SurfaceElectricCurrentDensity::new::<ECD>(V::one()),
                    &(ElectricCurrent::new::<I>(V::one()) / Length::new::<L>(V::one())).into());
            }
        }
    }
}
