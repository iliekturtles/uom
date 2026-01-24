//! Thermal resistance (base unit kelvin per watt, kg⁻¹ · m⁻² · s³ · K).

quantity! {
    /// Thermal resistance (base unit kelvin per watt, kg⁻¹ · m⁻² · s³ · K).
    quantity: ThermalResistance; "thermal resistance";
    /// Dimension of thermal resistance, L⁻²M⁻¹T³Th (base unit kelvin per watt,
    /// kg⁻¹ · m⁻² · s³ · K).
    dimension: ISQ<
        N2,     // length
        N1,     // mass
        P3,     // time
        Z0,     // electric current
        P1,     // thermodynamic temperature
        Z0,     // ammount of substance
        Z0>;    // luminous intensity
    units {
        @kelvin_per_yottawatt: prefix!(none) / prefix!(yotta); "K/YW", "kelvin per yottawatt",
            "kelvins per yottawatt";
        @kelvin_per_zettawatt: prefix!(none) / prefix!(zetta); "K/ZW", "kelvin per zettawatt",
            "kelvins per zettawatt";
        @kelvin_per_exawatt: prefix!(none) / prefix!(exa); "K/EW", "kelvin per exawatt",
            "kelvins per exawatt";
        @kelvin_per_petawatt: prefix!(none) / prefix!(peta); "K/PW", "kelvin per petawatt",
            "kelvins per petawatt";
        @kelvin_per_terawatt: prefix!(none) / prefix!(tera); "K/TW", "kelvin per terawatt",
            "kelvins per terawatt";
        @kelvin_per_gigawatt: prefix!(none) / prefix!(giga); "K/GW", "kelvin per gigawatt",
            "kelvins per gigawatt";
        @kelvin_per_megawatt: prefix!(none) / prefix!(mega); "K/MW", "kelvin per megawatt",
            "kelvins per megawatt";
        @kelvin_per_kilowatt: prefix!(none) / prefix!(kilo); "K/kW", "kelvin per kilowatt",
            "kelvins per kilowatt";
        @kelvin_per_hectowatt: prefix!(none) / prefix!(hecto); "K/hW", "kelvin per hectowatt",
            "kelvins per hectowatt";
        @kelvin_per_decawatt: prefix!(none) / prefix!(deca); "K/daW", "kelvin per decawatt",
            "kelvins per decawatt";
        @kelvin_per_watt: prefix!(none) / prefix!(none); "K/W", "kelvin per watt",
            "kelvins per watt";
        @kelvin_per_deciwatt: prefix!(none) / prefix!(deci); "K/dW", "kelvin per deciwatt",
            "kelvins per deciwatt";
        @kelvin_per_centiwatt: prefix!(none) / prefix!(centi); "K/cW", "kelvin per centiwatt",
            "kelvins per centiwatt";
        @kelvin_per_milliwatt: prefix!(none) / prefix!(milli); "K/mW", "kelvin per milliwatt",
            "kelvins per milliwatt";
        @kelvin_per_microwatt: prefix!(none) / prefix!(micro); "K/µW", "kelvin per microwatt",
            "kelvins per microwatt";
        @kelvin_per_nanowatt: prefix!(none) / prefix!(nano); "K/nW", "kelvin per nanowatt",
            "kelvins per nanowatt";
        @kelvin_per_picowatt: prefix!(none) / prefix!(pico); "K/pW", "kelvin per picowatt",
            "kelvins per picowatt";
        @kelvin_per_femtowatt: prefix!(none) / prefix!(femto); "K/fW", "kelvin per femtowatt",
            "kelvins per femtowatt";
        @kelvin_per_attowatt: prefix!(none) / prefix!(atto); "K/aW", "kelvin per attowatt",
            "kelvins per attowatt";
        @kelvin_per_zeptowatt: prefix!(none) / prefix!(zepto); "K/zW", "kelvin per zeptowatt",
            "kelvins per zeptowatt";
        @kelvin_per_yoctowatt: prefix!(none) / prefix!(yocto); "K/yW", "kelvin per yoctowatt",
            "kelvins per yoctowatt";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::power as p;
        use crate::si::quantities::*;
        use crate::si::temperature_interval as ti;
        use crate::si::thermal_resistance as tr;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ThermalResistance<V> = TemperatureInterval::new::<ti::kelvin>(V::one())
                / Power::new::<p::watt>(V::one());
        }

        #[test]
        fn check_power_ti_units() {
            test::<ti::kelvin, p::yottawatt, tr::kelvin_per_yottawatt>();
            test::<ti::kelvin, p::zettawatt, tr::kelvin_per_zettawatt>();
            test::<ti::kelvin, p::exawatt, tr::kelvin_per_exawatt>();
            test::<ti::kelvin, p::petawatt, tr::kelvin_per_petawatt>();
            test::<ti::kelvin, p::terawatt, tr::kelvin_per_terawatt>();
            test::<ti::kelvin, p::gigawatt, tr::kelvin_per_gigawatt>();
            test::<ti::kelvin, p::megawatt, tr::kelvin_per_megawatt>();
            test::<ti::kelvin, p::kilowatt, tr::kelvin_per_kilowatt>();
            test::<ti::kelvin, p::hectowatt, tr::kelvin_per_hectowatt>();
            test::<ti::kelvin, p::decawatt, tr::kelvin_per_decawatt>();
            test::<ti::kelvin, p::watt, tr::kelvin_per_watt>();
            test::<ti::kelvin, p::deciwatt, tr::kelvin_per_deciwatt>();
            test::<ti::kelvin, p::centiwatt, tr::kelvin_per_centiwatt>();
            test::<ti::kelvin, p::milliwatt, tr::kelvin_per_milliwatt>();
            test::<ti::kelvin, p::microwatt, tr::kelvin_per_microwatt>();
            test::<ti::kelvin, p::nanowatt, tr::kelvin_per_nanowatt>();
            test::<ti::kelvin, p::picowatt, tr::kelvin_per_picowatt>();
            test::<ti::kelvin, p::femtowatt, tr::kelvin_per_femtowatt>();
            test::<ti::kelvin, p::attowatt, tr::kelvin_per_attowatt>();
            test::<ti::kelvin, p::zeptowatt, tr::kelvin_per_zeptowatt>();
            test::<ti::kelvin, p::yoctowatt, tr::kelvin_per_yoctowatt>();

            fn test<
                TI: ti::Conversion<V>,
                P: p::Conversion<V>,
                TR: tr::Conversion<V>>()
            {
                Test::assert_approx_eq(&ThermalResistance::new::<TR>(V::one()),
                    &(TemperatureInterval::new::<TI>(V::one())
                        / Power::new::<P>(V::one())));
            }
        }
    }
}
