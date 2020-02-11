//! [Curvature](https://en.wikipedia.org/wiki/Curvature) (base unit radian per meter, m⁻¹).

quantity! {
    /// Curvature (base unit radian per meter, m⁻¹).
    quantity: Curvature; "curvature";
    /// Dimension of curvature, L⁻¹ (base unit radian per meter, m⁻¹).
    dimension: ISQ<
        N1,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (::si::marker::AngleKind);
    units {
        @radian_per_meter: 1.0_E0; "rad/m", "radian per meter", "radians per meter";
        @degree_per_meter: 1.745_329_251_994_329_5_E-2; "°/m", "degree per meter",
            "degrees per meter";

        @radian_per_millimeter: 1000.0; "rad/mm", "radian per millimeter", "radians per millimeter";
        @degree_per_millimeter: 1.745_329_251_994_329_5_E1; "°/mm", "degree per millimeter",
            "degrees per millimeter";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::curvature as c;
        use si::length as l;
        use si::angle as a;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Curvature<V> =
                (Angle::new::<a::radian>(V::one())
                    / Length::new::<l::meter>(V::one())).into();
        }

        #[test]
        fn check_units() {
            test::<a::radian, l::meter, c::radian_per_meter>();
            test::<a::degree, l::meter, c::degree_per_meter>();
            test::<a::radian, l::millimeter, c::radian_per_millimeter>();
            test::<a::degree, l::millimeter, c::degree_per_millimeter>();

            fn test<A: a::Conversion<V>, L: l::Conversion<V>, C: c::Conversion<V>>() {
                Test::assert_approx_eq(&Curvature::new::<C>(V::one()),
                    &(Angle::new::<A>(V::one()) / Length::new::<L>(V::one())).into());
            }
        }
    }
}
