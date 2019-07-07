//! Angular acceleration (base unit radian per second squared, s⁻²).

quantity! {
    /// Angular acceleration (base unit radian per second squared, s⁻²).
    quantity: AngularAcceleration; "angular acceleration";
    /// Dimension of angular acceleration, T⁻² (base unit radian per second squared, s⁻²).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (::si::marker::AngleKind);
    units {
        /// Derived unit of angular acceleration.
        @radian_per_second_squared: 1.0; "rad/s²", "radian per second squared",
            "radians per second squared";
        @degree_per_second_squared: 1.745_329_251_994_329_5_E-2; "°/s²",
            "degree per second squared", "degrees per second squared";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::angle as a;
        use si::angular_acceleration as aa;
        use si::quantities::*;
        use si::time as t;
        use tests::Test;

        #[test]
        fn check_units() {
            test::<a::radian, t::second, aa::radian_per_second_squared>();
            test::<a::degree, t::second, aa::degree_per_second_squared>();

            fn test<A: a::Conversion<V>, T: t::Conversion<V>, R: aa::Conversion<V>>() {
                let square_second = Time::new::<T>(V::one()) * Time::new::<T>(V::one());

                Test::assert_approx_eq(&AngularAcceleration::new::<R>(V::one()),
                    &(Angle::new::<A>(V::one()) / square_second).into());
            }
        }
    }
}
