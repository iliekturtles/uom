//! Moment of inertia (base unit kilogram square meter, kg · m² ).

quantity! {
    /// Moment of inertia (base unit kilogram square meter, kg · m² ).
    quantity: MomentOfInertia; "moment of inertia";
    /// Dimension of moment of inertia, L²M (base unit kilogram square meter, kg · m² ).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @kilogram_square_meter: prefix!(none); "kg · m²", "kilogram square meter",
            "kilogram square meters";
        @dalton_square_angstrom: 1.660_539_066_60_E-27 * 1_E-20; "Da · Å²",
            "dalton square ångström", "dalton square ångströms";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::moment_of_inertia as moi;
        use crate::si::quantities::*;
        use crate::si::length as l;
        use crate::si::mass as m;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: MomentOfInertia<V> = Mass::new::<m::kilogram>(V::one())
                * Length::new::<l::meter>(V::one())
                * Length::new::<l::meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<m::kilogram, l::meter, moi::kilogram_square_meter>();
            test::<m::dalton, l::angstrom, moi::dalton_square_angstrom>();

            fn test<M: m::Conversion<V>, L: l::Conversion<V>, MOI: moi::Conversion<V>>() {
                Test::assert_approx_eq(&MomentOfInertia::new::<MOI>(V::one()),
                    &(Mass::new::<M>(V::one())
                        * Length::new::<L>(V::one())
                        * Length::new::<L>(V::one())));
            }
        }
    }
}
