//! Gyromagnetic ratio, γ (base unit hertz per tesla, kg⁻¹ · s · A).

quantity! {
    /// Gyromagnetic ratio (base unit hertz per tesla, kg⁻¹ · s · A).
    quantity: GyromagneticRatio; "gyromagnetic ratio";
    /// Dimension of gyromagnetic ratio, M⁻¹TI (base unit hertz per tesla, kg⁻¹ · s · A).
    dimension: ISQ<
        Z0,     // length
        N1,     // mass
        P1,     // time
        P1,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottahertz_per_tesla: prefix!(yotta); "YHz/T", "yottahertz per tesla",
            "yottahertz per tesla";
        @zettahertz_per_tesla: prefix!(zetta); "ZHz/T", "zettahertz per tesla",
            "zettahertz per tesla";
        @exahertz_per_tesla: prefix!(exa); "EHz/T", "exahertz per tesla", "exahertz per tesla";
        @petahertz_per_tesla: prefix!(peta); "PHz/T", "petahertz per tesla", "petahertz per tesla";
        @terahertz_per_tesla: prefix!(tera); "THz/T", "terahertz per tesla", "terahertz per tesla";
        @gigahertz_per_tesla: prefix!(giga); "GHz/T", "gigahertz per tesla", "gigahertz per tesla";
        @megahertz_per_tesla: prefix!(mega); "MHz/T", "megahertz per tesla", "megahertz per tesla";
        @kilohertz_per_tesla: prefix!(kilo); "kHz/T", "kilohertz per tesla", "kilohertz per tesla";
        @hectohertz_per_tesla: prefix!(hecto); "hHz/T", "hectohertz per tesla",
            "hectohertz per tesla";
        @decahertz_per_tesla: prefix!(deca); "daHz/T", "decahertz per tesla", "decahertz per tesla";
        /// Derived unit of gyromagnetic ratio.
        @hertz_per_tesla: prefix!(none); "Hz/T", "hertz per tesla", "hertz per tesla", base_unit();
        @decihertz_per_tesla: prefix!(deci); "dHz/T", "decihertz per tesla", "decihertz per tesla";
        @centihertz_per_tesla: prefix!(centi); "cHz/T", "centihertz per tesla",
            "centihertz per tesla";
        @millihertz_per_tesla: prefix!(milli); "mHz/T", "millihertz per tesla",
            "millihertz per tesla";
        @microhertz_per_tesla: prefix!(micro); "µHz/T", "microhertz per tesla",
            "microhertz per tesla";
        @nanohertz_per_tesla: prefix!(nano); "nHz/T", "nanohertz per tesla", "nanohertz per tesla";
        @picohertz_per_tesla: prefix!(pico); "pHz/T", "picohertz per tesla", "picohertz per tesla";
        @femtohertz_per_tesla: prefix!(femto); "fHz/T", "femtohertz per tesla",
            "femtohertz per tesla";
        @attohertz_per_tesla: prefix!(atto); "aHz/T", "attohertz per tesla", "attohertz per tesla";
        @zeptohertz_per_tesla: prefix!(zepto); "zHz/T", "zeptohertz per tesla",
            "zeptohertz per tesla";
        @yoctohertz_per_tesla: prefix!(yocto); "yHz/T", "yoctohertz per tesla",
            "yoctohertz per tesla";

        @radian_per_second_tesla: 1.0_E0; "rad/(s · T)", "radian per second tesla",
            "radians per second tesla";

        // Gyromagnetic ratio, γ
        // γ = (magnetic dipole movement * nuclear magneton) / (spin * hbar)
        // Magnetic dipole movement and spin https://nds.iaea.org/records/x2773-kj397
        @proton_gyromagnetic_ratio: 2.675_221_870_8_E8; "γₚ", "proton gyromagnetic ratio",
            "proton gyromagnetic ratio";
        @neutron_gyromagnetic_ratio: 1.832_471_74_E8; "γₙ", "neutron gyromagnetic ratio",
            "neutron gyromagnetic ratio";
        @electron_gyromagnetic_ratio: 1.760_859_627_84_E11; "γₑ", "electron gyromagnetic ratio",
            "electron gyromagnetic ratio";
        @shielded_proton_gyromagnetic_ratio: 2.675_153_194_E8; "γ'ₚ",
            "shielded proton gyromagnetic ratio", "shielded proton gyromagnetic ratio";
        @shielded_helion_gyromagnetic_ratio: 2.037_894_607_8_E8; "γ'ₕ",
            "shielded helion gyromagnetic ratio", "shielded helion gyromagnetic ratio";
        @deuteron_gyromagnetic_ratio: 4.106_628_88_E7; "γ₂", "deuteron gyromagnetic ratio",
            "deuteron gyromagnetic ratio";
        @triton_gyromagnetic_ratio: 2.853_498_44_E8; "γ₃", "triton gyromagnetic ratio",
            "triton gyromagnetic ratio";
        @helium3_gyromagnetic_ratio: -2.038_016_78_E8; "γ₃", "helium3 gyromagnetic ratio",
            "helium3 gyromagnetic ratio";
        @lithium7_gyromagnetic_ratio: 1.039_752_64_E8; "γ₇", "lithium7 gyromagnetic ratio",
            "lithium7 gyromagnetic ratio";
        @boron11_gyromagnetic_ratio: 8.583_841_43_E7; "γ₁₁", "boron11 gyromagnetic ratio",
            "boron11 gyromagnetic ratio";
        @carbon13_gyromagnetic_ratio: 6.727_875_46_E7; "γ₁₃", "carbon13 gyromagnetic ratio",
            "carbon13 gyromagnetic ratio";
        @nitrogen14_gyromagnetic_ratio: 1.932_879_22_E7; "γ₁₄", "nitrogen14 gyromagnetic ratio",
            "nitrogen14 gyromagnetic ratio";
        @nitrogen15_gyromagnetic_ratio: -2.711_354_82_E7; "γ₁₅", "nitrogen15 gyromagnetic ratio",
            "nitrogen15 gyromagnetic ratio";
        @oxygen17_gyromagnetic_ratio: -3.627_586_49_E7; "γ₁₇", "oxygen17 gyromagnetic ratio",
            "oxygen17 gyromagnetic ratio";
        @flourine19_gyromagnetic_ratio: 2.517_624_83_E8; "γ₁₉", "flourine19 gyromagnetic ratio",
            "flourine19 gyromagnetic ratio";
        @sodium23_gyromagnetic_ratio: 7.080_354_17_E7; "γ₂₃", "sodium23 gyromagnetic ratio",
            "sodium23 gyromagnetic ratio";
        @phosphorus31_gyromagnetic_ratio: 1.083_294_19_E8; "γ₃₁", "phosphorus31 gyromagnetic ratio",
            "phosphorus31 gyromagnetic ratio";

        // Reduced gyromagnetic ratios, ̅γ.
        // ̅γ = γ / (2 · PI)
        @proton_reduced_gyromagnetic_ratio: 4.257_747_846_1_E7; "̅γₚ",
            "proton reduced gyromagnetic ratio", "proton reduced gyromagnetic ratio";
        @neutron_reduced_gyromagnetic_ratio: 2.916_469_35_E7; "̅γₙ",
            "neutron reduced gyromagnetic ratio", "neutron reduced gyromagnetic ratio";
        @electron_reduced_gyromagnetic_ratio: 2.802_495_138_61_E10; "̅γₑ",
            "electron reduced gyromagnetic ratio", "electron reduced gyromagnetic ratio";
        @shielded_proton_reduced_gyromagnetic_ratio: 4.257_638_543_E7; "γ'ₚ",
            "shielded proton reduced gyromagnetic ratio",
            "shielded proton reduced gyromagnetic ratio";
        @shielded_helion_reduced_gyromagnetic_ratio: 3.243_410_003_3_E7; "γ'ₕ",
            "shielded helion reduced gyromagnetic ratio",
            "shielded helion reduced gyromagnetic ratio";
        @deuteron_reduced_gyromagnetic_ratio: 6.535_902_85_E6; "̅γ₂",
            "deuteron reduced gyromagnetic ratio", "deuteron reduced gyromagnetic ratio";
        @triton_reduced_gyromagnetic_ratio: 4.541_483_81_E7; "̅γ₃",
            "triton reduced gyromagnetic ratio", "triton reduced gyromagnetic ratio";
        @helium3_reduced_gyromagnetic_ratio: -3.243_604_45_E7; "̅γ₃",
            "helium3 reduced gyromagnetic ratio", "helium3 reduced gyromagnetic ratio";
        @lithium7_reduced_gyromagnetic_ratio: 1.654_817_73_E7; "̅γ₇",
            "lithium7 reduced gyromagnetic ratio", "lithium7 reduced gyromagnetic ratio";
        @boron11_reduced_gyromagnetic_ratio: 1.366_160_80_E7; "̅γ₁₁",
            "boron11 reduced gyromagnetic ratio", "boron11 reduced gyromagnetic ratio";
        @carbon13_reduced_gyromagnetic_ratio: 1.070_774_64_E7; "̅γ₁₃",
            "carbon13 reduced gyromagnetic ratio", "carbon13 reduced gyromagnetic ratio";
        @nitrogen14_reduced_gyromagnetic_ratio: 3.076_272_81_E6; "̅γ₁₄",
            "nitrogen14 reduced gyromagnetic ratio", "nitrogen14 reduced gyromagnetic ratio";
        @nitrogen15_reduced_gyromagnetic_ratio: -4.315_255_22_E6; "̅γ₁₅",
            "nitrogen15 reduced gyromagnetic ratio", "nitrogen15 reduced gyromagnetic ratio";
        @oxygen17_reduced_gyromagnetic_ratio: -5.773_483_22_E6; "̅γ₁₇",
            "oxygen17 reduced gyromagnetic ratio", "oxygen17 reduced gyromagnetic ratio";
        @flourine19_reduced_gyromagnetic_ratio: 4.006_924_37_E7; "̅γ₁₉",
            "flourine19 reduced gyromagnetic ratio", "flourine19 reduced gyromagnetic ratio";
        @sodium23_reduced_gyromagnetic_ratio: 1.126_873_36_E7; "̅γ₂₃",
            "sodium23 reduced gyromagnetic ratio", "sodium23 reduced gyromagnetic ratio";
        @phosphorus31_reduced_gyromagnetic_ratio: 1.724_116_25_E7; "̅γ₃₁",
            "phosphorus31 reduced gyromagnetic ratio", "phosphorus31 reduced gyromagnetic ratio";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        // Everything BUT complex
        types: PrimInt, Ratio, Float, BigInt, BigUint;

        use crate::num::{FromPrimitive, One};
        use crate::si::angle as a;
        use crate::si::frequency as f;
        use crate::si::gyromagnetic_ratio as g;
        use crate::si::magnetic_flux_density as mfd;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: GyromagneticRatio<V> = Frequency::new::<f::hertz>(V::one())
                / MagneticFluxDensity::new::<mfd::tesla>(V::one());
        }

        #[test]
        fn check_units() {
            test::<f::yottahertz, g::yottahertz_per_tesla>();
            test::<f::zettahertz, g::zettahertz_per_tesla>();
            test::<f::exahertz, g::exahertz_per_tesla>();
            test::<f::petahertz, g::petahertz_per_tesla>();
            test::<f::terahertz, g::terahertz_per_tesla>();
            test::<f::gigahertz, g::gigahertz_per_tesla>();
            test::<f::megahertz, g::megahertz_per_tesla>();
            test::<f::kilohertz, g::kilohertz_per_tesla>();
            test::<f::hectohertz, g::hectohertz_per_tesla>();
            test::<f::decahertz, g::decahertz_per_tesla>();
            test::<f::hertz, g::hertz_per_tesla>();
            test::<f::decihertz, g::decihertz_per_tesla>();
            test::<f::centihertz, g::centihertz_per_tesla>();
            test::<f::millihertz, g::millihertz_per_tesla>();
            test::<f::microhertz, g::microhertz_per_tesla>();
            test::<f::nanohertz, g::nanohertz_per_tesla>();
            test::<f::picohertz, g::picohertz_per_tesla>();
            test::<f::femtohertz, g::femtohertz_per_tesla>();
            test::<f::attohertz, g::attohertz_per_tesla>();
            test::<f::zeptohertz, g::zeptohertz_per_tesla>();
            test::<f::yoctohertz, g::yoctohertz_per_tesla>();

            fn test<F: f::Conversion<V>, G: g::Conversion<V>>() {
                Test::assert_approx_eq(&GyromagneticRatio::new::<G>(V::one()),
                    &(Frequency::new::<F>(V::one())
                        / MagneticFluxDensity::new::<mfd::tesla>(V::one())));
            }
        }

        #[test]
        fn check_angle_time_mfd() {
            test::<a::radian, t::second, mfd::tesla, g::radian_per_second_tesla>();

            fn test<A: a::Conversion<V>, T: t::Conversion<V>, MFD: mfd::Conversion<V>,
                G: g::Conversion<V>>()
            {
                Test::assert_approx_eq(&GyromagneticRatio::new::<G>(V::one()),
                    &(Angle::new::<A>(V::one())
                        / (Time::new::<T>(V::one()) * MagneticFluxDensity::new::<MFD>(V::one()))));
            }
        }

        #[test]
        fn check_reduced_gyromagnetic_ratio() {
            test::<g::proton_gyromagnetic_ratio, g::proton_reduced_gyromagnetic_ratio>(0.001);
            test::<g::neutron_gyromagnetic_ratio, g::neutron_reduced_gyromagnetic_ratio>(2.0);
            test::<g::electron_gyromagnetic_ratio, g::electron_reduced_gyromagnetic_ratio>(0.1);
            test::<g::shielded_proton_gyromagnetic_ratio, g::shielded_proton_reduced_gyromagnetic_ratio>(0.01);
            test::<g::shielded_helion_gyromagnetic_ratio, g::shielded_helion_reduced_gyromagnetic_ratio>(2.0);
            test::<g::deuteron_gyromagnetic_ratio, g::deuteron_reduced_gyromagnetic_ratio>(1.0);
            test::<g::triton_gyromagnetic_ratio, g::triton_reduced_gyromagnetic_ratio>(0.1);
            test::<g::helium3_gyromagnetic_ratio, g::helium3_reduced_gyromagnetic_ratio>(0.1);
            test::<g::lithium7_gyromagnetic_ratio, g::lithium7_reduced_gyromagnetic_ratio>(0.1);
            test::<g::boron11_gyromagnetic_ratio, g::boron11_reduced_gyromagnetic_ratio>(0.1);
            test::<g::carbon13_gyromagnetic_ratio, g::carbon13_reduced_gyromagnetic_ratio>(0.1);
            test::<g::nitrogen14_gyromagnetic_ratio, g::nitrogen14_reduced_gyromagnetic_ratio>(0.1);
            test::<g::nitrogen15_gyromagnetic_ratio, g::nitrogen15_reduced_gyromagnetic_ratio>(0.001);
            test::<g::oxygen17_gyromagnetic_ratio, g::oxygen17_reduced_gyromagnetic_ratio>(0.01);
            test::<g::flourine19_gyromagnetic_ratio, g::flourine19_reduced_gyromagnetic_ratio>(0.1);
            test::<g::sodium23_gyromagnetic_ratio, g::sodium23_reduced_gyromagnetic_ratio>(0.1);
            test::<g::phosphorus31_gyromagnetic_ratio, g::phosphorus31_reduced_gyromagnetic_ratio>(0.1);

            fn test<G: g::Conversion<V>, RG: g::Conversion<V>>(error: f64) {
                let g = GyromagneticRatio::new::<RG>(V::one());
                let rg = GyromagneticRatio::new::<G>(V::one())
                    / Angle::new::<a::revolution>(V::one());
                let delta = if g > rg { g - rg } else { rg - g };
                let error = V::from_f64(error).unwrap();

                assert!(&delta <= &GyromagneticRatio::new::<g::hertz_per_tesla>(error));
            }
        }
    }
}
