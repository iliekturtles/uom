extern crate uom;

use uom::si::f32::*;
use uom::si::length::{yard};
use uom::si::time::{minute};
use uom::si::velocity::{meter_per_second};

#[inline(never)]
#[no_mangle]
fn print(velocity: Velocity) {
    println!("{:?}", velocity.get(meter_per_second));
}

#[inline(never)]
#[no_mangle]
fn calc() -> Velocity {
    let length = Length::new(100.0, yard);
    let time = Time::new(1.0, minute);

    length / time
}

fn main() {
    print(calc());
}
