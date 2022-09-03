//! Linear power density (base unit watt per meter, m · kg · s⁻³).

quantity! {
    /// Linear power density (base unit watt per meter, m · kg · s⁻³).
    quantity: LinearPowerDensity; "linear power density";
    /// Dimension of linear power density, LMT⁻³ (base unit watt per meter, m · kg · s⁻³).
    dimension: ISQ<
        P1,     // length
        P1,     // mass
        N3,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @watt_per_meter: prefix!(none); "W/m", "watt per meter", "watts per meter";
        @watt_per_centimeter: prefix!(none) / prefix!(centi); "W/cm", "watt per centimeter",
            "watts per centimeter";
        @watt_per_millimeter: prefix!(none) / prefix!(milli); "W/mm", "watt per millimeter",
            "watts per millimeter";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::power as p;
        use crate::si::quantities::*;
        use crate::si::linear_power_density as lpd;
        use crate::si::length as l;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: LinearPowerDensity<V> =  Power::new::<p::watt>(V::one())
                / Length::new::<l::meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<p::watt, l::meter, lpd::watt_per_meter>();
            test::<p::watt, l::centimeter, lpd::watt_per_centimeter>();
            test::<p::watt, l::millimeter, lpd::watt_per_millimeter>();

            fn test<P: p::Conversion<V>, L: l::Conversion<V>, LPD: lpd::Conversion<V>>() {
                Test::assert_approx_eq(&LinearPowerDensity::new::<LPD>(V::one()),
                    &(Power::new::<P>(V::one()) / Length::new::<L>(V::one())));
            }
        }
    }
}
