//! Angular Absement (base unit radian second, s).

quantity! {
    /// Angular Absement (base unit radian second, s).
    quantity: AngularAbsement; "angular absement";
    /// Dimension of angular absement, T (base unit radian second, s).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        P1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::AngleKind);
    units {
        @radian_second: prefix!(none); "rad · s", "radian second", "radian seconds";
        @degree_second: 1.745_329_251_994_329_5_E-2; "° · s", "degree second", "degree seconds";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::si::angular_absement as a;
        use crate::si::angle as l;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::tests::Test;
        use crate::num::One;

        #[test]
        fn check_dimension() {
            let _: AngularAbsement<V> = (Angle::new::<l::radian>(V::one())
                * Time::new::<t::second>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<l::radian, t::second, a::radian_second>();
            test::<l::degree, t::second, a::degree_second>();

            fn test<L: l::Conversion<V>, T: t::Conversion<V>, A: a::Conversion<V>>() {
                Test::assert_eq(&AngularAbsement::new::<A>(V::one()),
                    &(Angle::new::<L>(V::one()) * Time::new::<T>(V::one())).into());
            }
        }
    }
}
