//! Example showing how to create a set of `Quantity` type aliases for a different set of base
//! units.

#[macro_use]
extern crate uom;

use uom::si::length::{centimeter, meter};
use uom::si::time::second;
use uom::si::velocity::centimeter_per_second;

mod cgs {
    ISQ!(uom::si, f32, (centimeter, gram, second, ampere, kelvin, mole, candela));
}

fn main() {
    let l1 = uom::si::f32::Length::new(1.0, meter);
    let l2 = cgs::Length::new(1.0, centimeter);
    let t1 = uom::si::f32::Time::new(15.0, second);

    println!("{:?}", l1);
    println!("{:?}", l2);
    println!("{:?} m + {:?} cm = {:?} m", l1.get(meter), l2.get(centimeter), (l1 + l2).get(meter));
    println!("{:?} cm + {:?} m = {:?} cm",
             l2.get(centimeter),
             l1.get(meter),
             (l2 + l1).get(centimeter));
    println!("{:?} cm / {:?} s = {:?} cm/s",
             l2.get(centimeter),
             t1.get(second),
             (l2 / t1).get(centimeter_per_second));
}
