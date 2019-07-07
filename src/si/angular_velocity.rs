//! Angular velocity (base unit radian per second, s⁻¹).

quantity! {
    /// Angular velocity (base unit radian per second, s⁻¹).
    quantity: AngularVelocity; "angular velocity";
    /// Dimension of angular velocity, T⁻¹ (base unit radian per second, s⁻¹).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (::si::marker::AngleKind);
    units {
        /// Derived unit of angular velocity.
        @radian_per_second: 1.0_E0; "rad/s", "radian per second", "radians per second";
        @degree_per_second: 1.745_329_251_994_329_5_E-2; "°/s", "degree per second",
            "degrees per second";
        @revolution_per_second: 6.283_185_307_179_586_E0; "rps", "revolution per second",
            "revolutions per second";
        @revolution_per_minute: 1.047_197_551_196_597_7_E-1; "rpm", "revolution per minute",
            "revolutions per minute";
        @revolution_per_hour: 1.745_329_251_994_329_6_E-3; "rph", "revolution per hour",
            "revolutions per hour";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::angle as a;
        use si::angular_velocity as v;
        use si::quantities::*;
        use si::time as t;
        use tests::Test;

        #[test]
        fn check_units() {
            test::<a::radian, t::second, v::radian_per_second>();
            test::<a::degree, t::second, v::degree_per_second>();
            test::<a::revolution, t::second, v::revolution_per_second>();
            test::<a::revolution, t::minute, v::revolution_per_minute>();
            test::<a::revolution, t::hour, v::revolution_per_hour>();

            fn test<A: a::Conversion<V>, T: t::Conversion<V>, R: v::Conversion<V>>() {
                Test::assert_approx_eq(&AngularVelocity::new::<R>(V::one()),
                    &(Angle::new::<A>(V::one()) / Time::new::<T>(V::one())).into());
            }
        }
    }
}
