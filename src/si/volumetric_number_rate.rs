//! Volumetric number rate (base unit 1 per cubic meter second, m⁻³ · s⁻¹).

quantity! {
    /// Volumetric number rate (base unit 1 per cubic meter second, m⁻³ · s⁻¹).
    quantity: VolumetricNumberRate; "volumetric number rate";
    /// Dimension of volumetric number rate, L⁻³T⁻¹ (base unit 1 per cubic meter second, m⁻³ · s⁻¹).
    dimension: ISQ<
        N3,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn crate::si::marker::ConstituentConcentrationKind;
    units {
        @per_cubic_meter_second: prefix!(none); "m⁻³ · s⁻¹",
            "per cubic meter second", "per cubic meter second";
        @per_cubic_centimeter_second:
            prefix!(none) / prefix!(centi) / prefix!(centi) / prefix!(centi); "cm⁻³ · s⁻¹",
            "per cubic centimeter second", "per cubic centimeter second";
        @per_cubic_millimeter_second:
            prefix!(none) / prefix!(milli) / prefix!(milli) / prefix!(milli); "mm⁻³ · s⁻¹",
            "per cubic millimeter second", "per cubic millimeter second";

        @per_cubic_foot_second: prefix!(none) / 2.831_685_E-2; "ft⁻³ · s⁻¹",
            "per cubic foot second", "per cubic foot second";
        @per_cubic_inch_second: prefix!(none) / 1.638_706_E-5; "in⁻³ · s⁻¹",
            "per cubic inch second", "per cubic inch second";
        @per_cubic_mile_second: prefix!(none) / 4.168_182_E9; "mi⁻³ · s⁻¹",
            "per cubic mile second", "per cubic mile second";
        @per_cubic_yard_second: prefix!(none) / 7.645_549_E-1; "yd⁻³ · s⁻¹",
            "per cubic yard second", "per cubic yard second";
        @per_fluid_ounce_second: prefix!(none) / 2.957_353_E-5; "fl oz⁻¹ · s⁻¹",
            "per fluid ounce second", "per fluid ounce second";
        @per_fluid_ounce_imperial_second: prefix!(none) / 2.841_306_E-5; "fl oz⁻¹ (UK) · s⁻¹",
            "per Imperial fluid ounce second", "per Imperial fluid ounce second";
        @per_gallon_imperial_second: prefix!(none) / 4.546_09_E-3; "gal⁻¹ (UK) · s⁻¹",
            "per Imperial gallon second", "per Imperial gallon second";
        @per_gallon_second: prefix!(none) / 3.785_412_E-3; "gal⁻¹ · s⁻¹", "per gallon second",
            "per gallon second";
        @per_liter_second: prefix!(none) / prefix!(milli); "L⁻¹ · s⁻¹", "per liter second",
            "per liter second";
        @per_milliliter_second: prefix!(none) / prefix!(milli) / prefix!(milli); "mL⁻¹ · s⁻¹",
            "per milliliter second", "per milliliter second";

        @becquerel_per_cubic_meter: prefix!(none); "Bq/m³", "becquerel per cubic meter",
            "becquerels per cubic meter";

        @curie_per_cubic_meter: 3.7_E10; "Ci/m³", "curie per cubic meter", "curies per cubic meter";

        @disintegrations_per_minute_per_cubic_meter: 1.0 / 6.0_E1; "dpm/m³",
            "disintegration per minute per cubic meter",
            "disintegrations per minute per cubic meter";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::volumetric_number_rate as vnr;
        use crate::si::radioactivity as rad;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::volume as vol;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: VolumetricNumberRate<V> = (V::one()
                / Time::new::<t::second>(V::one())
                / Volume::new::<vol::cubic_meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<vnr::per_cubic_meter_second, vol::cubic_meter, t::second>();
            test::<vnr::per_cubic_centimeter_second, vol::cubic_centimeter, t::second>();
            test::<vnr::per_cubic_millimeter_second, vol::cubic_millimeter, t::second>();

            test::<vnr::per_cubic_foot_second, vol::cubic_foot, t::second>();
            test::<vnr::per_cubic_inch_second, vol::cubic_inch, t::second>();
            test::<vnr::per_cubic_mile_second, vol::cubic_mile, t::second>();
            test::<vnr::per_cubic_yard_second, vol::cubic_yard, t::second>();
            test::<vnr::per_fluid_ounce_second, vol::fluid_ounce, t::second>();
            test::<vnr::per_fluid_ounce_imperial_second, vol::fluid_ounce_imperial, t::second>();
            test::<vnr::per_gallon_imperial_second, vol::gallon_imperial, t::second>();
            test::<vnr::per_gallon_second, vol::gallon, t::second>();
            test::<vnr::per_liter_second, vol::liter, t::second>();
            test::<vnr::per_milliliter_second, vol::milliliter, t::second>();

            fn test<VNR: vnr::Conversion<V>, VOL: vol::Conversion<V>, T: t::Conversion<V>>() {
                Test::assert_approx_eq(&VolumetricNumberRate::new::<VNR>(V::one()),
                    &(V::one()
                        / Time::new::<T>(V::one())
                        / Volume::new::<VOL>(V::one())).into());
            }
        }

        #[test]
        fn check_units_volumetric_radioactivity() {
            test::<rad::becquerel, vol::cubic_meter, vnr::becquerel_per_cubic_meter>();
            test::<rad::curie, vol::cubic_meter, vnr::curie_per_cubic_meter>();
            test::<rad::disintegrations_per_minute, vol::cubic_meter,
                vnr::disintegrations_per_minute_per_cubic_meter>();

            fn test<RAD: rad::Conversion<V>, VOL: vol::Conversion<V>, VNR: vnr::Conversion<V>>() {
                Test::assert_approx_eq(&VolumetricNumberRate::new::<VNR>(V::one()),
                    &(Radioactivity::new::<RAD>(V::one()) / Volume::new::<VOL>(V::one())).into());
            }
        }
    }
}
