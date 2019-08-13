//! Angular jerk (base unit radian per second cubed, s⁻³).

quantity! {
    /// Angular jerk (base unit radian per second cubed, s⁻³).
    quantity: AngularJerk; "angular jerk";
    /// Dimension of angular jerk, T⁻³ (base unit radian per second cubed, s⁻³).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N3,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (::si::marker::AngleKind);
    units {
        /// Derived unit of angular jerk.
        @radian_per_second_cubed: 1.0; "rad/s³", "radian per second cubed",
            "radians per second cubed";
        @degree_per_second_cubed: 1.745_329_251_994_329_5_E-2; "°/s³",
            "degree per second cubed", "degrees per second cubed";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::angle as a;
        use si::angular_jerk as aj;
        use si::quantities::*;
        use si::time as t;
        use tests::Test;

        #[test]
        fn check_units() {
            test::<a::radian, t::second, aj::radian_per_second_cubed>();
            test::<a::degree, t::second, aj::degree_per_second_cubed>();

            fn test<A: a::Conversion<V>, T: t::Conversion<V>, R: aj::Conversion<V>>() {
                let cubic_second = Time::new::<T>(V::one()) *
                                   Time::new::<T>(V::one()) *
                                   Time::new::<T>(V::one());

                Test::assert_approx_eq(&AngularJerk::new::<R>(V::one()),
                    &(Angle::new::<A>(V::one()) / cubic_second).into());
            }
        }
    }
}
