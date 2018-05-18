//! Example showing how to use the pre-built SI system.

extern crate uom;

use uom::si::f32::*;
use uom::si::length::{centimeter, kilometer, meter};
use uom::si::time::second;
use uom::si::velocity::{kilometer_per_second, meter_per_second};
use uom::si::Unit;

fn main() {
    let l1 = Length::new::<meter>(15.0);
    let l2 = Length::new::<centimeter>(10.0);
    let t1 = Time::new::<second>(50.0);
    let v1 = l1 / t1;
    //let error = l1 + t1; // error[E0308]: mismatched types

    println!(
        "{:?} {} + {:?} {} = {:?} {}",
        l1.get::<meter>(),
        meter::abbreviation(),
        l2.get::<centimeter>(),
        centimeter::abbreviation(),
        (l1 + l2).get::<meter>(),
        meter::abbreviation()
    );
    println!(
        "{:?} {} + {:?} {} = {:?} {}",
        l1.get::<meter>(),
        meter::abbreviation(),
        l2.get::<centimeter>(),
        centimeter::abbreviation(),
        (l1 + l2).get::<kilometer>(),
        kilometer::abbreviation()
    );
    println!(
        "{:?} {} / {:?} {} = {:?} {}",
        l1.get::<meter>(),
        meter::abbreviation(),
        t1.get::<second>(),
        second::abbreviation(),
        v1.get::<meter_per_second>(),
        meter_per_second::abbreviation()
    );
    println!(
        "{:?} {} / {:?} {} = {:?} {}",
        l1.get::<meter>(),
        meter::abbreviation(),
        t1.get::<second>(),
        second::abbreviation(),
        v1.get::<kilometer_per_second>(),
        kilometer_per_second::abbreviation()
    );
}
