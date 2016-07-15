extern crate typenum;
extern crate uom;

use std::default::{Default};
use typenum::{Z0, P1, N1};
use uom::{Scalar};
use uom::si::{SI};
use uom::si::f32::*;

fn main() {
    let mass = Length::default();
    let time = Time::default();
    let velocity: Scalar<SI<P1, Z0, N1, Z0, Z0, Z0, Z0>, f32> = mass / time;
    let more_mass = mass + mass;
    //let error = mass + time; // mismatched types [E0308]

    println!("{:?} / {:?} = {:?}", mass, time, velocity);
    println!("{mass:?} + {mass:?} = {more_mass:?}", mass = mass, more_mass = more_mass);
}
