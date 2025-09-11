//! Areal mass density (base unit kilogram per square meter, m⁻² · kg).

quantity! {
    /// Areal mass density (base unit kilogram per square meter, m⁻² · kg).
    quantity: ArealMassDensity; "areal mass density";
    /// Dimension of areal mass density, L⁻²M (base unit kilogram per square meter, m⁻² · kg).
    dimension: ISQ<
        N2,     // length
        P1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @kilogram_per_square_meter: prefix!(none); "kg/m²", "kilogram per square meter",
            "kilograms per square meter";
        @gram_per_square_meter: prefix!(milli); "g/m²", "gram per square meter",
            "grams per square meter";
        @gram_per_square_centimeter: prefix!(milli) / prefix!(centi) / prefix!(centi); "g/cm²",
            "gram per square centimeter", "grams per square centimeter";

        @ounce_per_square_foot: 2.834_952_E-2 / 9.290_304_E-2; "oz/ft²", "ounce per square foot",
            "ounces per square foot";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::mass as m;
        use crate::si::areal_mass_density as d;
        use crate::si::quantities::*;
        use crate::si::area as a;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ArealMassDensity<V> = Mass::new::<m::kilogram>(V::one())
                / Area::new::<a::square_meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<m::kilogram, a::square_meter, d::kilogram_per_square_meter>();
            test::<m::gram, a::square_meter, d::gram_per_square_meter>();
            test::<m::gram, a::square_centimeter, d::gram_per_square_centimeter>();

            test::<m::ounce, a::square_foot, d::ounce_per_square_foot>();

            fn test<M: m::Conversion<V>, A: a::Conversion<V>, D: d::Conversion<V>>() {
                Test::assert_approx_eq(&ArealMassDensity::new::<D>(V::one()),
                    &(Mass::new::<M>(V::one()) / Area::new::<A>(V::one())));
            }
        }
    }
}
