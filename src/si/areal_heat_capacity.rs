//! Areal heat capacity (base unit joule per square meter kelvin, kg · s⁻² · K⁻¹).

quantity! {
    /// Areal heat capacity (base unit joule per square meter kelvin, kg · s⁻² · K⁻¹).
    quantity: ArealHeatCapacity; "areal heat capacity";
    /// Dimension of areal heat capacity, MT⁻²Th⁻¹(base unit joule per square meter kelvin,
    /// kg · s⁻² · K⁻¹).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        N1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @joule_per_square_meter_kelvin: prefix!(none); "J/(m² · K)",
            "joule per square meter kelvin", "joules per square meter kelvin";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::heat_capacity as hc;
        use crate::si::area as a;
        use crate::si::quantities::*;
        use crate::si::areal_heat_capacity as ahc;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ArealHeatCapacity<V> = HeatCapacity::new::<hc::joule_per_kelvin>(V::one())
                / Area::new::<a::square_meter>(V::one());
        }

        #[test]
        fn check_heat_capacity_area_units() {
            test::<hc::joule_per_kelvin, a::square_meter, ahc::joule_per_square_meter_kelvin>();

            fn test<HC: hc::Conversion<V>, A: a::Conversion<V>, AHC: ahc::Conversion<V>>() {
                Test::assert_approx_eq(&ArealHeatCapacity::new::<AHC>(V::one()),
                    &(HeatCapacity::new::<HC>(V::one()) / (Area::new::<A>(V::one()))));
            }
        }
    }
}
