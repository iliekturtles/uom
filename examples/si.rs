extern crate typenum;
extern crate uom;

use std::default::{Default};
use uom::si::f32::*;

fn main() {
    let mass = Length::default();
    let time = Time::default();
    let velocity: Velocity = mass / time;
    let more_mass = mass + mass;
    //let error = mass + time; // mismatched types [E0308]

    println!("{:?} / {:?} = {:?}", mass, time, velocity);
    println!("{mass:?} + {mass:?} = {more_mass:?}", mass = mass, more_mass = more_mass);
}
