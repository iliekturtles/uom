//! Dynamic viscosity (base unit pascal second, kg · m⁻¹ · s⁻¹).

quantity! {
    /// Dynamic viscosity (base unit pascal second, kg · m⁻¹ · s⁻¹).
    quantity: DynamicViscosity; "dynamic viscosity";
    /// Dimension of dynamic viscosity, L⁻¹MT⁻¹ (base unit pascal second, kg · m⁻¹ · s⁻¹).
    dimension: ISQ<
        N1,     // length
        P1,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @pascal_second: prefix!(none); "Pa · s", "pascal second", "pascal seconds";
        @millipascal_second: prefix!(milli); "mPa · s", "millipascal second", "millipascal seconds";
        @micropascal_second: prefix!(micro); "µPa · s", "micropascal second", "micropascal seconds";
        // poise = 0.1 Pa · s
        @poise: 1.0_E-1; "P", "poise", "poises";
        // centipoise = 1 mPa · s
        @centipoise: prefix!(centi) * 1.0_E-1; "cP", "centipoise", "centipoises";
        @pound_force_second_per_square_foot: 4.448_222_E0 / 3.048_E-1 / 3.048_E-1; "lbf · s/ft²",
            "pound-force second per square foot", "pound-force seconds per square foot";
        @pound_force_second_per_square_inch: 4.448_222_E0 / 2.54_E-2 / 2.54_E-2; "lbf · s/in²",
            "pound-force second per square inch", "pound-force seconds per square inch";
        // Reyn = 1 lbf · s/in²
        @reyn: 4.448_222_E0 / 2.54_E-2 / 2.54_E-2; "reyn", "reyn", "reyns";
        @pound_per_foot_second: 4.535_924_E-1 / 3.048_E-1; "lb/(ft · s)", "pound per foot second",
            "pounds per foot second";
        @pound_per_foot_hour: 4.535_924_E-1 / 3.048_E-1 / 3.6_E3; "lb/(ft · h)",
            "pound per foot hour", "pounds per foot hour";
        @slug_per_foot_second: 1.459_390_E1 / 3.048_E-1; "slug/(ft · s)", "slug per foot second",
            "slugs per foot second";
        @gram_per_centimeter_second: prefix!(milli) / prefix!(centi); "g/(cm · s)",
            "gram per centimeter second", "grams per centimeter second";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::quantities::*;
        use crate::si::dynamic_viscosity as dv;
        use crate::si::time as t;
        use crate::si::mass as m;
        use crate::si::length as l;
        use crate::si::pressure as p;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: DynamicViscosity<V> = Pressure::new::<p::pascal>(V::one())
                * Time::new::<t::second>(V::one());
        }

        #[test]
        fn check_units() {
            test::<p::pascal, t::second, dv::pascal_second>();
            test::<p::millipascal, t::second, dv::millipascal_second>();
            test::<p::micropascal, t::second, dv::micropascal_second>();
            test::<p::dyne_per_square_centimeter, t::second, dv::poise>();
            test::<p::millipascal, t::second,dv::centipoise>();
            test::<p::pound_force_per_square_foot, t::second,
                dv::pound_force_second_per_square_foot>();
            test::<p::pound_force_per_square_inch, t::second,
                dv::pound_force_second_per_square_inch>();
            test::<p::pound_force_per_square_inch, t::second, dv::reyn>();

            fn test<P: p::Conversion<V>, T: t::Conversion<V>, DV: dv::Conversion<V>>() {
                Test::assert_approx_eq(&DynamicViscosity::new::<DV>(V::one()),
                    &(Pressure::new::<P>(V::one())
                        * Time::new::<T>(V::one())));
            }
        }

        #[test]
        fn check_units_mlt() {
            test::<m::pound, l::foot, t::second, dv::pound_per_foot_second>();
            test::<m::pound, l::foot, t::hour, dv::pound_per_foot_hour>();
            test::<m::gram, l::centimeter, t::second, dv::gram_per_centimeter_second>();
            test::<m::slug, l::foot, t::second, dv::slug_per_foot_second>();

            fn test<M: m::Conversion<V>, L: l::Conversion<V>, T: t::Conversion<V>,
                DV: dv::Conversion<V>>()
            {
                Test::assert_approx_eq(&DynamicViscosity::new::<DV>(V::one()),
                    &(Mass::new::<M>(V::one())
                        / Length::new::<L>(V::one())
                        / Time::new::<T>(V::one())));
            }
        }
    }
}
