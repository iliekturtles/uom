//! Specific volume (base unit cubic meter per kilogram, m³ · kg⁻¹).

quantity! {
    /// Specific volume (base unit cubic meter per kilogram, m³ · kg⁻¹).
    quantity: SpecificVolume; "specific volume";
    /// Dimension of specific volume, L³M⁻¹ (base unit cubic meter per kilogram, m³ · kg⁻¹).
    dimension: ISQ<
        P3,     // length
        N1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @cubic_meter_per_kilogram: prefix!(none); "m³/kg", "cubic meter per kilogram",
            "cubic meters per kilogram";
        @cubic_centimeter_per_kilogram: prefix!(centi) * prefix!(centi) * prefix!(centi); "cm³/kg",
            "cubic centimeter per kilogram", "cubic centimeters per kilogram";

        @cubic_meter_per_gram: prefix!(none) / prefix!(milli); "m³/g", "cubic meter per gram",
            "cubic meters per gram";
        @cubic_centimeter_per_gram:
            prefix!(centi) * prefix!(centi) * prefix!(centi) / prefix!(milli); "cm³/g",
            "cubic centimeter per gram", "cubic centimeters per gram";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::volume as a;
        use crate::si::mass as m;
        use crate::si::specific_volume as sv;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: SpecificVolume<V> = Volume::new::<a::cubic_meter>(V::one())
                / Mass::new::<m::kilogram>(V::one());
        }

        #[test]
        fn check_units() {
            test::<a::cubic_meter, m::kilogram, sv::cubic_meter_per_kilogram>();
            test::<a::cubic_centimeter, m::kilogram, sv::cubic_centimeter_per_kilogram>();
            test::<a::cubic_meter, m::gram, sv::cubic_meter_per_gram>();
            test::<a::cubic_centimeter, m::gram, sv::cubic_centimeter_per_gram>();

            fn test<A: a::Conversion<V>, M: m::Conversion<V>, SV: sv::Conversion<V>>() {
                Test::assert_eq(&SpecificVolume::new::<SV>(V::one()),
                    &(Volume::new::<A>(V::one()) / Mass::new::<M>(V::one())));
            }
        }
    }
}
