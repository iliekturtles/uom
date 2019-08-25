//! Example showing how to create a custom system of quantities.

use length::{foot, meter};

fn main() {
    let l1 = f32::Length::new::<meter>(100.0);

    println!(
        "{:?} {} = {:?} {}",
        l1.get::<meter>(),
        meter::abbreviation(),
        l1.get::<foot>(),
        foot::abbreviation()
    );
}

uom_macros::system! {
    /// System of quantities, Q.
    name: Q;
    /// System of units, U.
    units: U;
    base {
        /// Length.
        length, L;
        /// Mass.
        mass, M;
        /// Time.
        time, T;
    }
    quantities {
        use area::Area;
        use length::Length;
        use mass::Mass;
        use time::Time;
    }
}

mod area {
    uom_macros::quantity! {
        /// Area.
        quantity: Area;
        description: "area";
        /// Area dimension, m<sup>2</sup>.
        dimension: Q<P2 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
        units {
            square_meter: 1.0_E0, "m²", "square meter", "square meters";
        }
    }
}

mod length {
    uom_macros::quantity! {
        /// Length.
        quantity: Length;
        description: "length";
        /// Length dimension, m<sup>1</sup>.
        dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
        units {
            /// The meter is the SI unit of length. It is defined by taking the fixed numerical
            /// value of the speed of light in vacuum *c* to be 299 792 458 when expressed in the
            /// unit m s⁻¹, where the second is defined in terms of the caesium frequency
            /// ∆*ν*<sub>Cs</sub>.
            meter: 1.0_E0, "m", "meter", "meters";
            foot: 3.048_E-1, "ft", "foot", "feet";
        }
    }
}

mod mass {
    uom_macros::quantity! {
        /// Mass.
        quantity: Mass;
        description: "mass";
        /// Mass dimension, kg<sup>1</sup>.
        dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
        units {
            /// The kilogram is the SI unit of mass. It is defined by taking the fixed numerical
            /// value of the Planck constant *h* to be 6.626 070 15 × 10⁻³⁴ when expressed in the
            /// unit J s, which is equal to kg m² s⁻¹, where the meter and the second are defined in
            /// terms of *c* and ∆*ν*<sub>Cs</sub>.
            kilogram: 1.0_E0, "kg", "kilogram", "kilograms";
            gram: 1.0_E-3, "g", "gram", "grams";
        }
    }
}

mod time {
    uom_macros::quantity! {
        /// Time.
        quantity: Time;
        description: "time";
        /// Time dimension, s<sup>1</sup>.
        dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
        units {
            /// The second is the SI unit of time. It is defined by taking the fixed numerical value
            /// of the caesium frequency ∆*ν*<sub>Cs</sub>, the unperturbed ground-state hyperfine
            /// transition frequency of the caesium 133 atom, to be 9 192 631 770 when expressed in
            /// the unit Hz, which is equal to s⁻¹.
            second: 1.0_E0, "s", "second", "seconds";
        }
    }
}
