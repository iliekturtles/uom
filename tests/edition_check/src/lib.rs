//! Validate that the `system!`, `quantity!`, and `$quantities!` macros generate valid code for the
//! 2018 edition.

#[macro_use]
extern crate uom;

mod mks {
    mod length {
        quantity! {
            /// Length (base unit meter, m).
            quantity: Length; "length";
            /// Length dimension, m.
            dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
            units {
                @meter: 1.0E0; "m", "meter", "meters";
                @foot: 3.048E-1; "ft", "foot", "feet";
            }
        }
    }

    mod mass {
        quantity! {
            /// Mass (base unit kilogram, kg).
            quantity: Mass; "mass";
            /// Mass dimension, kg.
            dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
            units {
                @kilogram: 1.0; "kg", "kilogram", "kilograms";
            }
        }
    }

    mod time {
        quantity! {
            /// Time (base unit second, s).
            quantity: Time; "time";
            /// Time dimension, s.
            dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
            units {
                @second: 1.0; "s", "second", "seconds";
            }
        }
    }

    system! {
        /// System of quantities, Q.
        quantities: Q {
            length: meter, L;
            mass: kilogram, M;
            time: second, T;
        }
        /// System of units, U.
        units: U {
            mod length::Length,
            mod mass::Mass,
            mod time::Time,
        }
    }

    mod f32 {
        Q!(crate::mks, f32 /*, (centimeter, gram, second)*/);
    }
}
