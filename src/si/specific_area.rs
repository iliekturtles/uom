//! Specific area (base unit square meter per kilogram, m² · kg⁻¹).

quantity! {
    /// Specific area (base unit square meter per kilogram, m² · kg⁻¹).
    quantity: SpecificArea; "specific area";
    /// Dimension of specific area, L²M⁻¹ (base unit square meter per kilogram, m² · kg⁻¹).
    dimension: ISQ<
        P2,     // length
        N1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @square_meter_per_kilogram: prefix!(none); "m²/kg", "square meter per kilogram",
            "square meters per kilogram";
        @square_centimeter_per_kilogram: prefix!(centi) * prefix!(centi); "cm²/kg",
            "square centimeter per kilogram", "square centimeters per kilogram";

        @square_meter_per_gram: prefix!(none) / prefix!(milli); "m²/g", "square meter per gram",
            "square meters per gram";
        @square_centimeter_per_gram: prefix!(centi) * prefix!(centi) / prefix!(milli); "cm²/g",
            "square centimeter per gram", "square centimeters per gram";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::area as a;
        use crate::si::mass as m;
        use crate::si::specific_area as sa;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: SpecificArea<V> = Area::new::<a::square_meter>(V::one()) / Mass::new::<m::kilogram>(V::one());
        }

        #[test]
        fn check_units() {
            test::<a::square_meter, m::kilogram, sa::square_meter_per_kilogram>();
            test::<a::square_centimeter, m::kilogram, sa::square_centimeter_per_kilogram>();

            test::<a::square_meter, m::gram, sa::square_meter_per_gram>();
            test::<a::square_centimeter, m::gram, sa::square_centimeter_per_gram>();

            fn test<A: a::Conversion<V>, M: m::Conversion<V>, SA: sa::Conversion<V>, >() {
                Test::assert_eq(&SpecificArea::new::<SA>(V::one()),
                    &(Area::new::<A>(V::one()) / Mass::new::<M>(V::one())));
            }
        }
    }
}
