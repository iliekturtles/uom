//! Heat flux density (base unit watt per square meter, kg · s⁻³).

quantity! {
    /// Heat flux density (base unit watt per square meter, kg · s⁻³).
    quantity: HeatFluxDensity; "heat flux density";
    /// Dimension of heat flux density, MT⁻³ (base unit watt per square meter, kg · s⁻³).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        N3,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottawatt_per_square_meter: prefix!(yotta); "YW/m²",
            "yottawatt per square meter", "yottawatts per square meter";
        @zettawatt_per_square_meter: prefix!(zetta); "ZW/m²",
            "zettawatt per square meter", "zettawatts per square meter";
        @exawatt_per_square_meter: prefix!(exa); "EW/m²",
            "exawatt per square meter", "exawatts per square meter";
        @petawatt_per_square_meter: prefix!(peta); "PW/m²",
            "petawatt per square meter", "petawatts per square meter";
        @terawatt_per_square_meter: prefix!(tera); "TW/m²",
            "terawatt per square meter", "terawatts per square meter";
        @gigawatt_per_square_meter: prefix!(giga); "GW/m²",
            "gigawatt per square meter", "gigawatts per square meter";
        @megawatt_per_square_meter: prefix!(mega); "MW/m²",
            "megawatt per square meter", "megawatts per square meter";
        /// Derived unit of density.
        @kilowatt_per_square_meter: prefix!(kilo); "kW/m²",
            "kilowatt per square meter", "kilowatts per square meter";
        @hectowatt_per_square_meter: prefix!(hecto); "hW/m²",
            "hectowatt per square meter", "hectowatts per square meter";
        @decawatt_per_square_meter: prefix!(deca); "daW/m²",
            "decawatt per square meter", "decawatts per square meter";
        @watt_per_square_meter: prefix!(none); "W/m²",
            "watt per square meter", "watts per square meter";
        @deciwatt_per_square_meter: prefix!(deci); "dW/m²",
            "deciwatt per square meter", "deciwatts per square meter";
        @centiwatt_per_square_meter: prefix!(centi); "cW/m²",
            "centiwatt per square meter", "centiwatts per square meter";
        @milliwatt_per_square_meter: prefix!(milli); "mW/m²",
            "milliwatt per square meter", "milliwatts per square meter";
        @microwatt_per_square_meter: prefix!(micro); "µW/m²",
            "microwatt per square meter", "microwatts per square meter";
        @nanowatt_per_square_meter: prefix!(nano); "nW/m²",
            "nanowatt per square meter", "nanowatts per square meter";
        @picowatt_per_square_meter: prefix!(pico); "pW/m²",
            "picowatt per square meter", "picowatts per square meter";
        @femtowatt_per_square_meter: prefix!(femto); "fW/m²",
            "femtowatt per square meter", "femtowatts per square meter";
        @attowatt_per_square_meter: prefix!(atto); "aW/m²",
            "attowatt per square meter", "attowatts per square meter";
        @zeptowatt_per_square_meter: prefix!(zepto); "zW/m²",
            "zeptowatt per square meter", "zeptowatts per square meter";
        @yoctowatt_per_square_meter: prefix!(yocto); "yW/m²",
            "yoctowatt per square meter", "yoctowatts per square meter";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::power as p;
        use si::length as l;
        use si::heat_flux_density as d;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: HeatFluxDensity<V> =
                Power::new::<p::kilowatt>(V::one())
                / (Length::new::<l::meter>(V::one())
                   * Length::new::<l::meter>(V::one()));
        }

        #[test]
        fn check_units() {
            test::<p::yottawatt, d::yottawatt_per_square_meter>();
            test::<p::zettawatt, d::zettawatt_per_square_meter>();
            test::<p::exawatt, d::exawatt_per_square_meter>();
            test::<p::petawatt, d::petawatt_per_square_meter>();
            test::<p::terawatt, d::terawatt_per_square_meter>();
            test::<p::gigawatt, d::gigawatt_per_square_meter>();
            test::<p::megawatt, d::megawatt_per_square_meter>();
            test::<p::kilowatt, d::kilowatt_per_square_meter>();
            test::<p::hectowatt, d::hectowatt_per_square_meter>();
            test::<p::decawatt, d::decawatt_per_square_meter>();
            test::<p::watt, d::watt_per_square_meter>();
            test::<p::deciwatt, d::deciwatt_per_square_meter>();
            test::<p::centiwatt, d::centiwatt_per_square_meter>();
            test::<p::milliwatt, d::milliwatt_per_square_meter>();
            test::<p::microwatt, d::microwatt_per_square_meter>();
            test::<p::nanowatt, d::nanowatt_per_square_meter>();
            test::<p::picowatt, d::picowatt_per_square_meter>();
            test::<p::femtowatt, d::femtowatt_per_square_meter>();
            test::<p::attowatt, d::attowatt_per_square_meter>();
            test::<p::zeptowatt, d::zeptowatt_per_square_meter>();
            test::<p::yoctowatt, d::yoctowatt_per_square_meter>();

            fn test<P: p::Conversion<V>, D: d::Conversion<V>>() {
                Test::assert_approx_eq(
                    &HeatFluxDensity::new::<D>(V::one()),
                    &(Power::new::<P>(V::one())
                      / (Length::new::<l::meter>(V::one())
                         * Length::new::<l::meter>(V::one()))));
            }
        }
    }
}
