//! Volume (base unit cubic meter, m<sup>3</sup>).

quantity! {
    /// Volume (base unit cubic meter, m<sup>3</sup>).
    quantity: Volume; "volume";
    /// Volume dimension, m<sup>3</sup>.
    dimension: ISQ<
        P3,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @cubic_yottameter: prefix!(yotta) * prefix!(yotta) * prefix!(yotta);
            "Ym³", "cubic yottameter", "cubic yottameters";
        @cubic_zettameter: prefix!(zetta) * prefix!(zetta) * prefix!(zetta);
            "Zm³", "cubic zettameter", "cubic zettameters";
        @cubic_exameter: prefix!(exa) * prefix!(exa) * prefix!(exa);
            "Em³", "cubic exameter", "cubic exameters";
        @cubic_petameter: prefix!(peta) * prefix!(peta) * prefix!(peta);
            "Pm³", "cubic petameter", "cubic petameters";
        @cubic_terameter: prefix!(tera) * prefix!(tera) * prefix!(tera);
            "Tm³", "cubic terameter", "cubic terameters";
        @cubic_gigameter: prefix!(giga) * prefix!(giga) * prefix!(giga);
            "Gm³", "cubic gigameter", "cubic gigameters";
        @cubic_megameter: prefix!(mega) * prefix!(mega) * prefix!(mega);
            "Mm³", "cubic megameter", "cubic megameters";
        @cubic_kilometer: prefix!(kilo) * prefix!(kilo) * prefix!(kilo);
            "km³", "cubic kilometer", "cubic kilometers";
        @cubic_hectometer: prefix!(hecto) * prefix!(hecto) * prefix!(hecto);
            "hm³", "cubic hectometer", "cubic hectometers";
        @cubic_decameter: prefix!(deca) * prefix!(deca) * prefix!(deca);
            "dam³", "cubic decameter", "cubic decameters";
        @cubic_meter: prefix!(none); "m³", "cubic meter", "cubic meters";
        @cubic_decimeter: prefix!(deci) * prefix!(deci) * prefix!(deci);
            "dm³", "cubic decimeter", "cubic decimeters";
        @cubic_centimeter: prefix!(centi) * prefix!(centi) * prefix!(centi);
            "cm³", "cubic centimeter", "cubic centimeters";
        @cubic_millimeter: prefix!(milli) * prefix!(milli) * prefix!(milli);
            "mm³", "cubic millimeter", "cubic millimeters";
        @cubic_micrometer: prefix!(micro) * prefix!(micro) * prefix!(micro);
            "µm³", "cubic micrometer", "cubic micrometers";
        @cubic_nanometer: prefix!(nano) * prefix!(nano) * prefix!(nano);
            "nm³", "cubic nanometer", "cubic nanometers";
        @cubic_picometer: prefix!(pico) * prefix!(pico) * prefix!(pico);
            "pm³", "cubic picometer", "cubic picometers";
        @cubic_femtometer: prefix!(femto) * prefix!(femto) * prefix!(femto);
            "fm³", "cubic femtometer", "cubic femtometers";
        @cubic_attometer: prefix!(atto) * prefix!(atto) * prefix!(atto);
            "am³", "cubic attometer", "cubic attometers";
        @cubic_zeptometer: prefix!(zepto) * prefix!(zepto) * prefix!(zepto);
            "zm³", "cubic zeptometer", "cubic zeptometers";
        @cubic_yoctometer: prefix!(yocto) * prefix!(yocto) * prefix!(yocto);
            "ym³", "cubic yoctometer", "cubic yoctometers";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use lib::any::TypeId;
        use num::One;
        use si::quantities::*;
        use si::area as a;
        use si::volume as v;
        use si::length as l;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Volume<V> = Length::new::<l::meter>(V::one())
                * Length::new::<l::meter>(V::one())
                * Length::new::<l::meter>(V::one());
            let _: Volume<V> = Length::new::<l::meter>(V::one())
                * Area::new::<a::square_meter>(V::one());
        }

        #[test]
        fn check_units() {
            // Values too large for f32.
            if TypeId::of::<f64>() == TypeId::of::<V>() {
                test(l::yottameter, v::cubic_yottameter);
                test(l::zettameter, v::cubic_zettameter);
                test(l::exameter, v::cubic_exameter);
                test(l::petameter, v::cubic_petameter);
            }

            test(l::terameter, v::cubic_terameter);
            test(l::gigameter, v::cubic_gigameter);
            test(l::megameter, v::cubic_megameter);
            test(l::kilometer, v::cubic_kilometer);
            test(l::hectometer, v::cubic_hectometer);
            test(l::decameter, v::cubic_decameter);
            test(l::meter, v::cubic_meter);
            test(l::decimeter, v::cubic_decimeter);
            test(l::centimeter, v::cubic_centimeter);
            test(l::millimeter, v::cubic_millimeter);
            test(l::micrometer, v::cubic_micrometer);
            test(l::nanometer, v::cubic_nanometer);
            test(l::picometer, v::cubic_picometer);
            test(l::femtometer, v::cubic_femtometer);

            // Values too small for f32.
            if TypeId::of::<f64>() == TypeId::of::<V>() {
                test(l::attometer, v::cubic_attometer);
                test(l::zeptometer, v::cubic_zeptometer);
                test(l::yoctometer, v::cubic_yoctometer);
            }

            // TODO #17 Convert to == once PartialEq is implemented.
            fn test<L: l::Conversion<V>, O: v::Conversion<V>>(_l: L, v: O) {
                Test::assert_eq(&V::one(),
                    &(Length::new::<L>(V::one())
                        * Length::new::<L>(V::one())
                        * Length::new::<L>(V::one())).get(v));
            }
        }
    }
}
