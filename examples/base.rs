//! Example showing how to create a set of `Quantity` type aliases for a different set of base
//! units.

#[macro_use]
extern crate uom;

use uom::si::Unit;
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

    println!("{}: {:?}", uom::si::length::description(), l1);
    println!("{}: {:?}", uom::si::length::description(), l2);
    println!("{:?} {} + {:?} {} = {:?} {}",
             l1.get(meter),
             meter.abbreviation(),
             l2.get(centimeter),
             centimeter.abbreviation(),
             (l1 + l2).get(meter),
             meter.abbreviation());
    println!("{:?} {} + {:?} {} = {:?} {}",
             l2.get(centimeter),
             centimeter.abbreviation(),
             l1.get(meter),
             meter.abbreviation(),
             (l2 + l1).get(centimeter),
             centimeter.abbreviation());
    println!("{:?} {} / {:?} {} = {:?} {}",
             l2.get(centimeter),
             centimeter.abbreviation(),
             t1.get(second),
             second.abbreviation(),
             (l2 / t1).get(centimeter_per_second),
             centimeter_per_second.abbreviation());
}
