//! Angular momentum (base unit kilogram square meter per second, kg · m² · s⁻¹).

quantity! {
    /// Angular momentum (base unit kilogram square meter per second, kg · m² · s⁻¹).
    quantity: AngularMomentum; "angular momentum";
    /// Dimension of angular momentum, L²MT⁻¹ (base unit kilogram square meter per second, kg · m² · s⁻¹).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::AngleKind);
    units {
        /// Derived unit of angular velocity.
        @kilogram_square_meter_per_second: 1.0_E0; "kg m^2/s", "kilogram square meter per second", "kilogram square meters per second";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::moment_of_inertia as p;
        use crate::si::angular_velocity as v;
        use crate::si::quantities::*;
        use crate::si::angular_momentum as l;
        use crate::tests::Test;

        #[test]
        fn check_units() {
            test::<l::kilogram_square_meter_per_second, p::kilogram_square_meter, v::radian_per_second>();

            fn test<L: l::Conversion<V>, P: p::Conversion<V>, R: v::Conversion<V>>() {
                Test::assert_approx_eq(&AngularMomentum::new::<L>(V::one()),
                    &(MomentOfInertia::new::<P>(V::one()) * AngularVelocity::new::<R>(V::one())).into());
            }
        }
    }
}
