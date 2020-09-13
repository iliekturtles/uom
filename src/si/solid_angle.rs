//! Solid angle (dimensionless quantity).

quantity! {
    /// Solid angle (dimensionless quantity).
    quantity: SolidAngle; "solid angle";
    /// Dimension of solid angle, 1 (dimensionless).
    dimension: ISQ<
        Z0,     // length
        Z0,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    kind: dyn (crate::si::marker::SolidAngleKind);
    units {
        /// SI derived unit of solid angle is steradians. It is the solid angle subtended at the
        /// center of a unit sphere by a unit area on its surface.
        @steradian: 1.0_E0; "sr", "steradian", "steradians";
        @spat: 1.256_637_061_435_917_3_E1; "sp", "spat", "spats";
        @square_degree: 3.046_174_197_867_086_E-4; "°²", "square degree", "square degrees";
        @square_minute: 8.461_594_994_075_238_9_E-8; "′²", "square minute", "square minutes";
        @square_second: 2.350_443_053_909_788_6_E-11; "″²", "square second", "square seconds";
    }
}

#[cfg(feature = "f32")]
impl SolidAngle<crate::si::SI<f32>, f32> {
    /// The solid angle subtended by a sphere at its center, i.e. with a value 4π as measured in
    /// steradians.
    pub const SPHERE: Self = Self {
        dimension: crate::lib::marker::PhantomData,
        units: crate::lib::marker::PhantomData,
        value: 4. * crate::lib::f32::consts::PI,
    };
}

#[cfg(feature = "f64")]
impl SolidAngle<crate::si::SI<f64>, f64> {
    /// The solid angle subtended by a sphere at its center, i.e. with a value 4π as measured in
    /// steradians.
    pub const SPHERE: Self = Self {
        dimension: crate::lib::marker::PhantomData,
        units: crate::lib::marker::PhantomData,
        value: 4. * crate::lib::f64::consts::PI,
    };
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::lib::f64::consts::PI;
        use crate::num::{FromPrimitive, One};
        use crate::si::quantities::*;
        use crate::si::solid_angle as sa;
        use crate::tests::Test;

        #[test]
        fn check_units() {
            Test::assert_eq(&SolidAngle::new::<sa::steradian>(V::from_f64(4.0 * PI).unwrap()),
                &SolidAngle::new::<sa::spat>(V::one()));
            Test::assert_approx_eq(
                &SolidAngle::new::<sa::square_degree>(V::from_f64(360.0 * 360.0 / PI).unwrap()),
                &SolidAngle::new::<sa::spat>(V::one()));
            Test::assert_approx_eq(
                &SolidAngle::new::<sa::square_minute>(V::from_f64(60.0 * 60.0).unwrap()),
                &SolidAngle::new::<sa::square_degree>(V::one()));
            Test::assert_approx_eq(
                &SolidAngle::new::<sa::square_second>(
                    V::from_f64((60.0 * 60.0) * (60.0 * 60.0)).unwrap()),
                &SolidAngle::new::<sa::square_degree>(V::one()));
        }
    }
}
