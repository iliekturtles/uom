//! Example showing how to use the pre-built SI system.

extern crate uom;

use uom::si::f32::*;
use uom::si::length::{centimeter, kilometer, meter};
use uom::si::time::second;
use uom::si::velocity::{kilometer_per_second, meter_per_second};

fn main() {
    let l1 = Length::new(15.0, meter);
    let l2 = Length::new(10.0, centimeter);
    let t1 = Time::new(50.0, second);
    let v1 = l1 / t1;
    //let error = l1 + t1; // error[E0308]: mismatched types

    println!("{:?} m + {:?} cm = {:?} m", l1.get(meter), l2.get(centimeter), (l1 + l2).get(meter));
    println!("{:?} m + {:?} cm = {:?} km",
             l1.get(meter),
             l2.get(centimeter),
             (l1 + l2).get(kilometer));
    println!("{:?} m / {:?} s = {:?} m/s", l1.get(meter), t1.get(second), v1.get(meter_per_second));
    println!("{:?} m / {:?} s = {:?} km/s",
             l1.get(meter),
             t1.get(second),
             v1.get(kilometer_per_second));
}
