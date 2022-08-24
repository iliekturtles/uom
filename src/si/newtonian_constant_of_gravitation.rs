//! Newtonian constant of gravitation (base unit cubic meter per kilogram square second, m³ · kg⁻¹ · s⁻²).

quantity! {
    /// Newtonian constant of gravitation (base unit cubic meter per kilogram square second, m³ · kg⁻¹ · s⁻²).
    quantity: NewtonianConstantOfGravitation; "newtonian constant of gravitation";
    /// Dimension of newtonian constant of gravitation, L³M⁻¹T⁻² (base unit cubic meter per kilogram square second, m³ · kg⁻¹ · s⁻²).
    dimension: ISQ<
        P3,     // length
        N1,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @cubic_meter_pre_kilogram_square_second: prefix!(none); "m³·kg⁻¹·s⁻²",
            "cubic meter per kilogram square second", "cubic meters per kilogram square second";
        @newtonian_constant_of_gravitation: 6.674_30_E-11; "G",
            "newtonian constant of gravitation", "newtonian constants of gravitation";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::mass as m;
        use crate::si::newtonian_constant_of_gravitation as g;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::length as l;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: NewtonianConstantOfGravitation<V> = Length::new::<l::meter>(V::one())
                * Length::new::<l::meter>(V::one())
                * Length::new::<l::meter>(V::one())
                / Mass::new::<m::kilogram>(V::one())
                / Time::new::<t::second>(V::one())
                / Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<l::meter, m::kilogram, t::second, g::cubic_meter_pre_kilogram_square_second>();

            fn test<L: l::Conversion<V>, M: m::Conversion<V>, T: t::Conversion<V>, G: g::Conversion<V>>() {
                Test::assert_approx_eq(&NewtonianConstantOfGravitation::new::<G>(V::one()),
                    &(Length::new::<L>(V::one())
                    * Length::new::<L>(V::one())
                    * Length::new::<L>(V::one())
                    / Mass::new::<M>(V::one())
                    / Time::new::<T>(V::one())
                    / Time::new::<T>(V::one())));
            }
        }
    }
}
