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
        @kilojoule_per_square_meter_kelvin: prefix!(kilo); "kJ/(m² · K)",
            "kilojoule per square meter kelvin", "kilojoules per square meter kelvin";
        @megajoule_per_square_meter_kelvin: prefix!(mega); "MJ/(m² · K)",
            "megajoule per square meter kelvin", "megajoules per square meter kelvin";
        @gigajoule_per_square_meter_kelvin: prefix!(giga); "GJ/(m² · K)",
            "gigajoule per square meter kelvin", "gigajoules per square meter kelvin";
        @terajoule_per_square_meter_kelvin: prefix!(tera); "TJ/(m² · K)",
            "terajoule per square meter kelvin", "terajoules per square meter kelvin";
        @petajoule_per_square_meter_kelvin: prefix!(peta); "PJ/(m² · K)",
            "petajoule per square meter kelvin", "petajoules per square meter kelvin";
        @exajoule_per_square_meter_kelvin: prefix!(exa); "EJ/(m² · K)",
            "exajoule per square meter kelvin", "exajoules per square meter kelvin";
        @zettajoule_per_square_meter_kelvin: prefix!(zetta); "ZJ/(m² · K)",
            "zettajoule per square meter kelvin", "zettajoules per square meter kelvin";
        @yottajoule_per_square_meter_kelvin: prefix!(yotta); "YJ/(m² · K)",
            "yottajoule per square meter kelvin", "yottajoules per square meter kelvin";
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
            test::<hc::kilojoule_per_kelvin, a::square_meter, ahc::kilojoule_per_square_meter_kelvin>();
            test::<hc::megajoule_per_kelvin, a::square_meter, ahc::megajoule_per_square_meter_kelvin>();
            test::<hc::gigajoule_per_kelvin, a::square_meter, ahc::gigajoule_per_square_meter_kelvin>();
            test::<hc::terajoule_per_kelvin, a::square_meter, ahc::terajoule_per_square_meter_kelvin>();
            test::<hc::petajoule_per_kelvin, a::square_meter, ahc::petajoule_per_square_meter_kelvin>();
            test::<hc::exajoule_per_kelvin, a::square_meter, ahc::exajoule_per_square_meter_kelvin>();
            test::<hc::zettajoule_per_kelvin, a::square_meter, ahc::zettajoule_per_square_meter_kelvin>();
            test::<hc::yottajoule_per_kelvin, a::square_meter, ahc::yottajoule_per_square_meter_kelvin>();

            fn test<HC: hc::Conversion<V>, A: a::Conversion<V>, AHC: ahc::Conversion<V>>() {
                Test::assert_approx_eq(&ArealHeatCapacity::new::<AHC>(V::one()),
                    &(HeatCapacity::new::<HC>(V::one()) / (Area::new::<A>(V::one()))));
            }
        }
    }
}
