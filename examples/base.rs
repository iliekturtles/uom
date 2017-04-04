//! Example showing how to create a set of `Quantity` type aliases for a different set of base
//! units.

#[macro_use]
extern crate uom;

use uom::si::length::{centimeter, meter};
use uom::si::time::second;

mod cgs {
    ISQ!(uom::si, f32, (centimeter, gram, second, ampere, kelvin, mole, candela));
}

fn main() {
    let l1 = uom::si::f32::Length::new::<meter>(1.0);
    let l2 = cgs::Length::new::<centimeter>(1.0);
    let t1 = uom::si::f32::Time::new::<second>(15.0);

    println!("{}: {:?}", uom::si::length::description(), l1);
    println!("{}: {:?}", uom::si::length::description(), l2);
    println!("{:?} + {:?} = {:?}", l1, l2, (l1 + l2));
    println!("{:?} + {:?} = {:?}", l2, l1, (l2 + l1));
    println!("{:?} / {:?} = {:?}", l2, t1, (l2 / t1));
}
