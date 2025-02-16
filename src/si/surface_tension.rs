//! Surface tension (base unit newton per meter, kg · s⁻²).

quantity! {
    /// Surface tension (base unit newton per meter, kg · s⁻²).
    quantity: SurfaceTension; "surface tension";
    /// Dimension of surface tension, MT⁻² (base unit newton per meter, kg · s⁻²).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::SurfaceTensionKind);
    units {
        @yottanewton_per_meter: prefix!(yotta);
            "YN/m", "yottanewton per meter", "yottanewtons per meter";
        @zettanewton_per_meter: prefix!(zetta);
            "ZN/m", "zettanewton per meter", "zettanewtons per meter";
        @exanewton_per_meter: prefix!(exa);
            "EN/m", "exanewton per meter", "exanewtons per meter";
        @petanewton_per_meter: prefix!(peta);
            "PN/m", "petanewton per meter", "petanewtons per meter";
        @teranewton_per_meter: prefix!(tera);
            "TN/m", "teranewton per meter", "teranewtons per meter";
        @giganewton_per_meter: prefix!(giga);
            "GN/m", "giganewton per meter", "giganewtons per meter";
        @meganewton_per_meter: prefix!(mega);
            "MN/m", "meganewton per meter", "meganewtons per meter";
        @kilonewton_per_meter: prefix!(kilo);
            "kN/m", "kilonewton per meter", "kilonewtons per meter";
        @hectonewton_per_meter: prefix!(hecto);
            "hN/m", "hectonewton per meter", "hectonewtons per meter";
        @decanewton_per_meter: prefix!(deca);
            "daN/m", "decanewton per meter", "decanewtons per meter";
        /// Derived unit of surface tension.
        @newton_per_meter: prefix!(none);
            "N/m", "newton per meter", "newtons per meter";
        @decinewton_per_meter: prefix!(deci);
            "dN/m", "decinewton per meter", "decinewtons per meter";
        @centinewton_per_meter: prefix!(centi);
            "cN/m", "centinewton per meter", "centinewtons per meter";
        @millinewton_per_meter: prefix!(milli);
            "mN/m", "millinewton per meter", "millinewtons per meter";
        @micronewton_per_meter: prefix!(micro);
            "µN/m", "micronewton per meter", "micronewtons per meter";
        @nanonewton_per_meter: prefix!(nano);
            "nN/m", "nanonewton per meter", "nanonewtons per meter";
        @piconewton_per_meter: prefix!(pico);
            "pN/m", "piconewton per meter", "piconewtons per meter";
        @femtonewton_per_meter: prefix!(femto);
            "fN/m", "femtonewton per meter", "femtonewtons per meter";
        @attonewton_per_meter: prefix!(atto);
            "aN/m", "attonewton per meter", "attonewtons per meter";
        @zeptonewton_per_meter: prefix!(zepto);
            "zN/m", "zeptonewton per meter", "zeptonewtons per meter";
        @yoctonewton_per_meter: prefix!(yocto);
            "yN/m", "yoctonewton per meter", "yoctonewtons per meter";
        @dyne_per_centimeter: prefix!(none) / prefix!(centi) * 1.0_E-5;
            "dyn/cm", "dyne per centimeter", "dynes per centimeter";
        @kilogram_force_per_meter: 9.806_65_E0;
            "kgf/m", "kilogram-force per meter", "kilograms-force per meter";
        @gram_force_per_centimeter: prefix!(none) / prefix!(kilo) / prefix!(centi) * 9.806_65_E0;
            "gf/cm", "gram-force per centimeter", "grams-force per centimeter";
        @ounce_force_per_inch: 2.780_139_E-1 / 2.54_E-2;
            "ozf/in", "ounce-force per inch", "ounces-force per inch";
        @poundal_per_inch: 1.382_550_E-1 / 2.54_E-2;
            "pdl/in", "poundal per inch", "poundals per inch";
        @pound_force_per_inch: 4.448_222_E0 / 2.54_E-2;
            "lbf/in", "pound-force per inch", "pounds-force per inch";
        @joule_per_square_meter: prefix!(none);
            "J/m²", "joule per square meter", "joules per square meter";
        @joule_per_square_centimeter: prefix!(none) / (prefix!(centi) * prefix!(centi));
            "J/cm²", "joule per square centimeter", "joules per square centimeter";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::area as a;
        use crate::si::energy as e;
        use crate::si::force as f;
        use crate::si::length as l;
        use crate::si::quantities::*;
        use crate::si::surface_tension as st;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: SurfaceTension<V> = (Force::new::<f::newton>(V::one())
                / Length::new::<l::meter>(V::one())).into();
        }

        #[test]
        fn check_units_force() {
            test::<f::yottanewton, l::meter, st::yottanewton_per_meter>();
            test::<f::zettanewton, l::meter, st::zettanewton_per_meter>();
            test::<f::exanewton, l::meter, st::exanewton_per_meter>();
            test::<f::petanewton, l::meter, st::petanewton_per_meter>();
            test::<f::teranewton, l::meter, st::teranewton_per_meter>();
            test::<f::giganewton, l::meter, st::giganewton_per_meter>();
            test::<f::meganewton, l::meter, st::meganewton_per_meter>();
            test::<f::kilonewton, l::meter, st::kilonewton_per_meter>();
            test::<f::hectonewton, l::meter, st::hectonewton_per_meter>();
            test::<f::decanewton, l::meter, st::decanewton_per_meter>();
            test::<f::newton, l::meter, st::newton_per_meter>();
            test::<f::decinewton, l::meter, st::decinewton_per_meter>();
            test::<f::centinewton, l::meter, st::centinewton_per_meter>();
            test::<f::millinewton, l::meter, st::millinewton_per_meter>();
            test::<f::micronewton, l::meter, st::micronewton_per_meter>();
            test::<f::nanonewton, l::meter, st::nanonewton_per_meter>();
            test::<f::piconewton, l::meter, st::piconewton_per_meter>();
            test::<f::femtonewton, l::meter, st::femtonewton_per_meter>();
            test::<f::attonewton, l::meter, st::attonewton_per_meter>();
            test::<f::zeptonewton, l::meter, st::zeptonewton_per_meter>();
            test::<f::yoctonewton, l::meter, st::yoctonewton_per_meter>();
            test::<f::dyne, l::centimeter, st::dyne_per_centimeter>();
            test::<f::kilogram_force, l::meter, st::kilogram_force_per_meter>();
            test::<f::gram_force, l::centimeter, st::gram_force_per_centimeter>();
            test::<f::ounce_force, l::inch, st::ounce_force_per_inch>();
            test::<f::poundal, l::inch, st::poundal_per_inch>();
            test::<f::pound_force, l::inch, st::pound_force_per_inch>();

            fn test<
                F: f::Conversion<V>,
                L: l::Conversion<V>,
                ST: st::Conversion<V>
            >() {
                Test::assert_approx_eq(&SurfaceTension::new::<ST>(V::one()),
                    &(Force::new::<F>(V::one()) / Length::new::<L>(V::one())).into()
                );
            }
        }

        #[test]
        fn check_units_energy() {
            test::<e::joule, a::square_meter, st::joule_per_square_meter>();
            test::<e::joule, a::square_centimeter, st::joule_per_square_centimeter>();

            fn test<
                E: e::Conversion<V>,
                A: a::Conversion<V>,
                ST: st::Conversion<V>
            >() {
                Test::assert_approx_eq(&SurfaceTension::new::<ST>(V::one()),
                    &(Energy::new::<E>(V::one()) / Area::new::<A>(V::one())).into()
                );
            }
        }
    }
}
