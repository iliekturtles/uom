//! Example showing how to create a custom system of quantities.
//!
//! Requires compilation with the following feature flags: `--no-default-features --features f32`.

#![cfg(all(feature = "f32", not(feature = "f64"), not(feature = "si")))]

#[macro_use]
extern crate uom;

use length::{foot, meter};

fn main() {
    let l1 = f32::Length::new(100.0, meter);

    println!("{:?} m = {:?} ft", l1.get(meter), l1.get(foot));
}

#[macro_use]
mod length {
    quantity! {
        /// Length (base unit meter, m^(1)).
        quantity: Length;
        /// Length dimension, m^(1).
        dimension: Q<
            P1,  // length
            Z0,  // mass
            Z0>; // time
        units {
            @meter: 1.0E0;
            @foot: 3.048E-1;
        }
    }
}

#[macro_use]
mod mass {
    quantity! {
        /// Mass (base unit kilogram, kg^(1)).
        quantity: Mass;
        /// Mass dimension, kg^(1).
        dimension: Q<
            Z0,  // length
            P1,  // mass
            Z0>; // time
        units {
            @kilogram: 1.0;
        }
    }
}

#[macro_use]
mod time {
    quantity! {
        /// Time (base unit second, s^(1)).
        quantity: Time;
        /// Time dimension, s^(1).
        dimension: Q<
            Z0,  // length
            Z0,  // mass
            P1>; // time
        units {
            @second: 1.0;
        }
    }
}

system! {
    quantities: Q {
        length: meter, L;
        mass: kilogram, M;
        time: second, T;
    }

    units: U {
        Length,
        Mass,
        Time,
    }
}

mod f32 {
    mod s {
        pub use ::*;
    }

    Q!(f32::s, f32);
}
