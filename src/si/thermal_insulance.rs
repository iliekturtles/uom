//! Thermal insulance (base unit square meter kelvin per watt, kg⁻¹ ⋅ m⁻² ⋅ s³ ⋅ K)

quantity! {
    /// Thermal insulance (base unit square meter kelvin per watt, kg⁻¹ ⋅ m⁻² ⋅ s³ ⋅ K).
    /// Also called R-value.  Measures a materials resistance to the heat
    /// current.
    quantity: ThermalInsulance; "thermal insulance";
    /// Dimension of thermal insulance, L⁻²M⁻¹T³Th (base unit square meter
    /// kelvin per watt, kg⁻¹ ⋅ m⁻² ⋅ s³ ⋅ K).
    dimension: ISQ<
        Z0,     // length
        N1,     // mass
        P3,     // time
        Z0,     // electric current
        P1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @square_meter_kelvin_per_yottawatt: prefix!(none) / prefix!(yotta); "m²K/YW",
        "square meter kelvin per yottawatt", "square meter kelvins per yottawatt";
        @square_meter_kelvin_per_zettawatt: prefix!(none) / prefix!(zetta); "m²K/ZW",
        "square meter kelvin per zettawatt", "square meter kelvins per zettawatt";
        @square_meter_kelvin_per_exawatt: prefix!(none) / prefix!(exa); "m²K/EW",
        "square meter kelvin per exawatt", "square meter kelvins per exawatt";
        @square_meter_kelvin_per_petawatt: prefix!(none) / prefix!(peta); "m²K/PW",
        "square meter kelvin per petawatt", "square meter kelvins per petawatt";
        @square_meter_kelvin_per_terawatt: prefix!(none) / prefix!(tera); "m²K/TW",
        "square meter kelvin per terawatt", "square meter kelvins per terawatt";
        @square_meter_kelvin_per_gigawatt: prefix!(none) / prefix!(giga); "m²K/GW",
        "square meter kelvin per gigawatt", "square meter kelvins per gigawatt";
        @square_meter_kelvin_per_megawatt: prefix!(none) / prefix!(mega); "m²K/MW",
        "square meter kelvin per megawatt", "square meter kelvins per megawatt";
        @square_meter_kelvin_per_kilowatt: prefix!(none) / prefix!(kilo); "m²K/kW",
        "square meter kelvin per kilowatt", "square meter kelvins per kilowatt";
        @square_meter_kelvin_per_hectowatt: prefix!(none) / prefix!(hecto); "m²K/hW",
        "square meter kelvin per hectowatt", "square meter kelvins per hectowatt";
        @square_meter_kelvin_per_decawatt: prefix!(none) / prefix!(deca); "m²K/daW",
        "square meter kelvin per decawatt", "square meter kelvins per decawatt";
        @square_meter_kelvin_per_watt: prefix!(none) / prefix!(none); "m²K/W",
        "square meter kelvin per watt", "square meter kelvins per watt";
        @square_meter_kelvin_per_deciwatt: prefix!(none) / prefix!(deci); "m²K/dW",
        "square meter kelvin per deciwatt", "square meter kelvins per deciwatt";
        @square_meter_kelvin_per_centiwatt: prefix!(none) / prefix!(centi); "m²K/cW",
        "square meter kelvin per centiwatt", "square meter kelvins per centiwatt";
        @square_meter_kelvin_per_milliwatt: prefix!(none) / prefix!(milli); "m²K/mW",
        "square meter kelvin per milliwatt", "square meter kelvins per milliwatt";
        @square_meter_kelvin_per_microwatt: prefix!(none) / prefix!(micro); "m²K/μW",
        "square meter kelvin per microwatt", "square meter kelvins per microwatt";
        @square_meter_kelvin_per_nanowatt: prefix!(none) / prefix!(nano); "m²K/nW",
        "square meter kelvin per nanowatt", "square meter kelvins per nanowatt";
        @square_meter_kelvin_per_picowatt: prefix!(none) / prefix!(pico); "m²K/pW",
        "square meter kelvin per picowatt", "square meter kelvins per picowatt";
        @square_meter_kelvin_per_femtowatt: prefix!(none) / prefix!(femto); "m²K/fW",
        "square meter kelvin per femtowatt", "square meter kelvins per femtowatt";
        @square_meter_kelvin_per_attowatt: prefix!(none) / prefix!(atto); "m²K/aW",
        "square meter kelvin per attowatt", "square meter kelvins per attowatt";
        @square_meter_kelvin_per_zeptowatt: prefix!(none) / prefix!(zepto); "m²K/zW",
        "square meter kelvin per zeptowatt", "square meter kelvins per zeptowatt";
        @square_meter_kelvin_per_yoctowatt: prefix!(none) / prefix!(yocto); "m²K/yW",
        "square meter kelvin per yoctowatt", "square meter kelvins per yoctowatt";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::area as a;
        use crate::si::power as p;
        use crate::si::quantities::*;
        use crate::si::thermodynamic_temperature as t;
        use crate::si::thermal_insulance as ti;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ThermalInsulance<V> = Area::new::<a::square_meter>(V::one())
            * ThermodynamicTemperature::new::<t::kelvin>(V::one()) /
                Power::new::<p::watt>(V::one());
        }

        #[test]
        fn check_power_ti_units() {
            test::<a::square_meter, t::kelvin, p::yottawatt, ti::square_meter_kelvin_per_yottawatt>();
            test::<a::square_meter, t::kelvin, p::zettawatt, ti::square_meter_kelvin_per_zettawatt>();
            test::<a::square_meter, t::kelvin, p::exawatt,   ti::square_meter_kelvin_per_exawatt>();
            test::<a::square_meter, t::kelvin, p::petawatt,  ti::square_meter_kelvin_per_petawatt>();
            test::<a::square_meter, t::kelvin, p::terawatt,  ti::square_meter_kelvin_per_terawatt>();
            test::<a::square_meter, t::kelvin, p::gigawatt,  ti::square_meter_kelvin_per_gigawatt>();
            test::<a::square_meter, t::kelvin, p::megawatt,  ti::square_meter_kelvin_per_megawatt>();
            test::<a::square_meter, t::kelvin, p::kilowatt,  ti::square_meter_kelvin_per_kilowatt>();
            test::<a::square_meter, t::kelvin, p::hectowatt, ti::square_meter_kelvin_per_hectowatt>();
            test::<a::square_meter, t::kelvin, p::decawatt,  ti::square_meter_kelvin_per_decawatt>();
            test::<a::square_meter, t::kelvin, p::watt,      ti::square_meter_kelvin_per_watt>();
            test::<a::square_meter, t::kelvin, p::deciwatt,  ti::square_meter_kelvin_per_deciwatt>();
            test::<a::square_meter, t::kelvin, p::centiwatt, ti::square_meter_kelvin_per_centiwatt>();
            test::<a::square_meter, t::kelvin, p::milliwatt, ti::square_meter_kelvin_per_milliwatt>();
            test::<a::square_meter, t::kelvin, p::microwatt, ti::square_meter_kelvin_per_microwatt>();
            test::<a::square_meter, t::kelvin, p::nanowatt,  ti::square_meter_kelvin_per_nanowatt>();
            test::<a::square_meter, t::kelvin, p::picowatt,  ti::square_meter_kelvin_per_picowatt>();
            test::<a::square_meter, t::kelvin, p::femtowatt, ti::square_meter_kelvin_per_femtowatt>();
            test::<a::square_meter, t::kelvin, p::attowatt,  ti::square_meter_kelvin_per_attowatt>();
            test::<a::square_meter, t::kelvin, p::zeptowatt, ti::square_meter_kelvin_per_zeptowatt>();
            test::<a::square_meter, t::kelvin, p::yoctowatt, ti::square_meter_kelvin_per_yoctowatt>();

            fn test<
                A: a::Conversion<V>,
                T: t::Conversion<V>,
                P: p::Conversion<V>,
                TI: ti::Conversion<V>>()
            {
                Test::assert_approx_eq(&ThermalInsulance::new::<TI>(V::one()),
                    &(Area::new::<A>(V::one()) * ThermodynamicTemperature::new::<T>(V::one())
                        / Power::new::<P>(V::one())));
            }
        }
    }
}
