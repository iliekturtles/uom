//! Mass per energy (base unit kilogram per joule, m⁻² · s²).
//!
//! This quantity is typically used to express an emission intensity, also known as carbon
//! intensity. Emission intensity is a measure of how much mass of carbon dioxide (CO2) is emitted
//! per unit of energy.

quantity! {
    /// Mass per energy (base unit kilogram per joule, m⁻² · s²).
    quantity: MassPerEnergy; "mass per energy";
    /// Dimension of mass per energy, L⁻²T² (base unit kilogram per joule, m⁻² · s²).
    dimension: ISQ<
        N2,     // length
        Z0,     // mass
        P2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @teragram_per_joule: prefix!(tera) / prefix!(kilo); "Tg/J", "teragram per joule",
            "teragrams per joule";
        @gigagram_per_joule: prefix!(giga) / prefix!(kilo); "Gg/J", "gigagram per joule",
            "gigagrams per joule";
        @megagram_per_joule: prefix!(mega) / prefix!(kilo); "Mg/J", "megagram per joule",
            "megagrams per joule";
        /// Derived unit of Mass per energy.
        @kilogram_per_joule: prefix!(kilo) / prefix!(kilo); "kg/J", "kilogram per joule",
            "kilograms per joule";
        @hectogram_per_joule: prefix!(hecto) / prefix!(kilo); "hg/J", "hectogram per joule",
            "hectograms per joule";
        @decagram_per_joule: prefix!(deca) / prefix!(kilo); "dag/J", "decagram per joule",
            "decagrams per joule";
        @gram_per_joule: prefix!(none) / prefix!(kilo); "g/J", "gram per joule",
            "grams per joule";
        @decigram_per_joule: prefix!(deci) / prefix!(kilo); "dg/J", "decigram per joule",
            "decigrams per joule";
        @centigram_per_joule: prefix!(centi) / prefix!(kilo); "cg/J", "centigram per joule",
            "centigrams per joule";
        @milligram_per_joule: prefix!(milli) / prefix!(kilo); "mg/J", "milligram per joule",
            "milligrams per joule";
        @microgram_per_joule: prefix!(micro) / prefix!(kilo); "µg/J", "microgram per joule",
            "micrograms per joule";

        @pound_per_joule: 4.535_924_E-1; "lb/J", "pound per joule", "pounds per joule";
        @pound_per_gigawatt_hour: 4.535_924_E-1 / 3.6_E12; "lb/GWh", "pound per gigawatt hour",
            "pounds per gigawatt hour";
        @pound_per_megawatt_hour: 4.535_924_E-1 / 3.6_E9; "lb/MWh", "pound per megawatt hour",
            "pounds per megawatt hour";
        @pound_per_kilowatt_hour: 4.535_924_E-1 / 3.6_E6; "lb/kWh", "pound per kilowatt hour",
            "pounds per kilowatt hour";
        @pound_per_watt_hour: 4.535_924_E-1 / 3.6_E3; "lb/Wh", "pound per watt hour",
            "pounds per watt hour";

        @kilogram_per_gigawatt_hour: prefix!(none) / 3.6_E12; "g/GWh", "kilogram per gigawatt hour",
            "kilograms per gigawatt hour";
        @kilogram_per_megawatt_hour: prefix!(none) / 3.6_E9; "g/MWh", "kilogram per megawatt hour",
            "kilograms per megawatt hour";
        @kilogram_per_kilowatt_hour: prefix!(none) / 3.6_E6; "g/kWh", "kilogram per kilowatt hour",
            "kilograms per kilowatt hour";
        @kilogram_per_watt_hour: prefix!(none) / 3.6_E3; "g/Wh", "kilogram per watt hour",
            "kilograms per watt hour";

        @gram_per_gigawatt_hour: 1_E-3 / 3.6_E12; "g/GWh", "gram per gigawatt hour",
            "grams per gigawatt hour";
        @gram_per_megawatt_hour: 1_E-3 / 3.6_E9; "g/MWh", "gram per megawatt hour",
            "grams per megawatt hour";
        @gram_per_kilowatt_hour: 1_E-3 / 3.6_E6; "g/kWh", "gram per kilowatt hour",
            "grams per kilowatt hour";
        @gram_per_watt_hour: 1_E-3 / 3.6_E3; "g/Wh", "gram per watt hour", "grams per watt hour";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::mass_per_energy as v;
        use crate::si::energy as e;
        use crate::si::mass as m;
        use crate::si::quantities::*;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: MassPerEnergy<V> = Mass::new::<m::kilogram>(V::one())
                / Energy::new::<e::joule>(V::one());
        }

        #[test]
        fn check_units() {
            test::<m::teragram, e::joule, v::teragram_per_joule>();
            test::<m::gigagram, e::joule, v::gigagram_per_joule>();
            test::<m::megagram, e::joule, v::megagram_per_joule>();
            test::<m::kilogram, e::joule, v::kilogram_per_joule>();
            test::<m::hectogram, e::joule, v::hectogram_per_joule>();
            test::<m::decagram, e::joule, v::decagram_per_joule>();
            test::<m::gram, e::joule, v::gram_per_joule>();
            test::<m::decigram, e::joule, v::decigram_per_joule>();
            test::<m::centigram, e::joule, v::centigram_per_joule>();
            test::<m::milligram, e::joule, v::milligram_per_joule>();
            test::<m::microgram, e::joule, v::microgram_per_joule>();

            test::<m::pound, e::joule, v::pound_per_joule>();
            test::<m::pound, e::gigawatt_hour, v::pound_per_gigawatt_hour>();
            test::<m::pound, e::megawatt_hour, v::pound_per_megawatt_hour>();
            test::<m::pound, e::kilowatt_hour, v::pound_per_kilowatt_hour>();
            test::<m::pound, e::watt_hour, v::pound_per_watt_hour>();

            test::<m::kilogram, e::gigawatt_hour, v::kilogram_per_gigawatt_hour>();
            test::<m::kilogram, e::megawatt_hour, v::kilogram_per_megawatt_hour>();
            test::<m::kilogram, e::kilowatt_hour, v::kilogram_per_kilowatt_hour>();
            test::<m::kilogram, e::watt_hour, v::kilogram_per_watt_hour>();

            test::<m::gram, e::gigawatt_hour, v::gram_per_gigawatt_hour>();
            test::<m::gram, e::megawatt_hour, v::gram_per_megawatt_hour>();
            test::<m::gram, e::kilowatt_hour, v::gram_per_kilowatt_hour>();
            test::<m::gram, e::watt_hour, v::gram_per_watt_hour>();

            fn test<M: m::Conversion<V>, E: e::Conversion<V>, A: v::Conversion<V>>() {
                Test::assert_approx_eq(&MassPerEnergy::new::<A>(V::one()),
                    &(Mass::new::<M>(V::one()) / Energy::new::<E>(V::one())));
            }
        }
    }
}
