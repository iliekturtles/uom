//! Temperature gradient (base unit kelvin per meter, m⁻¹ · K).

quantity! {
    /// Temperature gradient (base unit kelvin per meter, m⁻¹ · K).
    quantity: TemperatureGradient; "temperature gradient";
    /// Dimension of temperature gradient, L⁻¹Th (base unit kelvin per meter, K · m⁻¹).
    dimension: ISQ<
        N1,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        P1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @kelvin_per_kilometer: prefix!(none) / prefix!(kilo); "K/km", "kelvin per kilometer",
            "kelvins per kilometer";
        @kelvin_per_hectometer: prefix!(none) / prefix!(hecto); "K/hm", "kelvin per hectometer",
            "kelvins per hectometer";
        @kelvin_per_meter: prefix!(none); "K/m", "kelvin per meter", "kelvins per meter";
        @kelvin_per_centimeter: prefix!(none) / prefix!(centi); "K/cm", "kelvin per centimeter",
            "kelvins per centimeter";
        @kelvin_per_millimeter: prefix!(none) / prefix!(milli); "K/mm", "kelvin per millimeter",
            "kelvins per millimeter";
        @kelvin_per_micrometer: prefix!(none) / prefix!(micro); "K/µm", "kelvin per micrometer",
            "kelvins per micrometer";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::temperature_gradient as tg;
        use crate::si::temperature_interval as ti;
        use crate::si::quantities::*;
        use crate::si::length as l;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: TemperatureGradient<V> = TemperatureInterval::new::<ti::kelvin>(V::one())
                / Length::new::<l::meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<ti::kelvin, l::meter, tg::kelvin_per_meter>();
            test::<ti::kelvin, l::centimeter, tg::kelvin_per_centimeter>();
            test::<ti::kelvin, l::millimeter, tg::kelvin_per_millimeter>();
            test::<ti::kelvin, l::micrometer, tg::kelvin_per_micrometer>();
            test::<ti::kelvin, l::hectometer, tg::kelvin_per_hectometer>();
            test::<ti::kelvin, l::kilometer, tg::kelvin_per_kilometer>();

            fn test<TI: ti::Conversion<V>, L: l::Conversion<V>, TG: tg::Conversion<V>>() {
                Test::assert_approx_eq(&TemperatureGradient::new::<TG>(V::one()),
                    &(TemperatureInterval::new::<TI>(V::one())
                        / Length::new::<L>(V::one())));
            }
        }
    }
}
