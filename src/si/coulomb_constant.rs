//! Coulomb constant (base unit kilogram cubic meter per second to the fourth power square ampere, kg · m³ · s⁻⁴ · A⁻²).

quantity! {
    /// Coulomb constant (base unit kilogram cubic meter per second to the fourth power square ampere, kg · m³ · s⁻⁴ · A⁻²).
    quantity: CoulombConstant; "coulomb constant";
    /// Dimension of coulomb constant, ML³T⁻⁴I⁻² (base unit kilogram cubic meter per second to the fourth power square ampere, kg · m³ · s⁻⁴ · A⁻²).
    dimension: ISQ<
        P3,     // length
        P1,     // mass
        N4,     // time
        N2,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @kilogram_cubic_meter_per_second_to_the_fourth_power_square_ampere: prefix!(none); "kg·m³·s⁻⁴·A⁻²",
            "kilogram cubic meter per second to the fourth power square ampere", "kilograms cubic meter per second to the fourth power square ampere";
        @meter_per_farad: prefix!(none); "m/F",
            "meter per farad", "meters_per_farad";
        @coulomb_constant: 8.987_551_792_261_172_E9 ; "kₑ", // Derived from CODATA as 1/(4πε₀)
            "coulomb constant", "coulomb constants";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::electric_current as i;
        use crate::si::electric_permittivity as epsilon;

        use crate::si::coulomb_constant as ke;
        use crate::si::quantities::*;
        use crate::si::mass as m;
        use crate::si::time as t;
        use crate::si::volume as v;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: CoulombConstant<V> = Mass::new::<m::kilogram>(V::one())
                * Volume::new::<v::cubic_meter>(V::one())
                / Time::new::<t::second>(V::one())
                / Time::new::<t::second>(V::one())
                / Time::new::<t::second>(V::one())
                / Time::new::<t::second>(V::one())
                / ElectricCurrent::new::<i::ampere>(V::one())
                / ElectricCurrent::new::<i::ampere>(V::one());
        }

        #[test]
        fn check_units() {
            test::<m::kilogram, v::cubic_meter, t::second, i::ampere, ke::kilogram_cubic_meter_per_second_to_the_fourth_power_square_ampere>();

            fn test<M: m::Conversion<V>, VOL: v::Conversion<V>, T: t::Conversion<V>, I: i::Conversion<V>, KE: ke::Conversion<V>>() {
                Test::assert_approx_eq(&CoulombConstant::new::<KE>(V::one()),
                    &(Mass::new::<M>(V::one())
                    * Volume::new::<VOL>(V::one())
                    / Time::new::<T>(V::one())
                    / Time::new::<T>(V::one())
                    / Time::new::<T>(V::one())
                    / Time::new::<T>(V::one())
                    / ElectricCurrent::new::<I>(V::one())
                    / ElectricCurrent::new::<I>(V::one())));
            }
        }

        #[test]
        fn check_units_farad_meter() {
            test::<epsilon::farad_per_meter, ke::kilogram_cubic_meter_per_second_to_the_fourth_power_square_ampere>();
            test::<epsilon::farad_per_meter, ke::meter_per_farad>();

            fn test<EPS: epsilon::Conversion<V>, KE: ke::Conversion<V>>() {
                Test::assert_approx_eq(&CoulombConstant::new::<KE>(V::one()),
                    &(V::one()
                    / ElectricPermittivity::new::<EPS>(V::one())
                   ));
            }
        }

        #[test]
        fn check_units_ke() {
            test::<epsilon::vacuum_electric_permittivity, ke::coulomb_constant>();

            fn test<EPS: epsilon::Conversion<V>, KE: ke::Conversion<V>>() {
                Test::assert_approx_eq(&CoulombConstant::new::<KE>(V::one()),
                     &(
                    
                    // 1 / (4 · π)
                    7.957_747_154_594_768_E-2 * V::one() / ElectricPermittivity::new::<EPS>(V::one())
                   ));
            }
        }

    }
}
