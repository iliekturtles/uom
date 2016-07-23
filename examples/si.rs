extern crate typenum;
extern crate uom;

use std::default::{Default};
use uom::si::f32::*;

fn main() {
    let mass = Mass::default();
    let length = Length::default();
    let time = Time::default();
    let velocity: Velocity = length / time;
    let more_mass = length + length;
    //let error = length + time; // mismatched types [E0308]
    //let error: Velocity = mass / time; // mismatched types [E0308]

    println!("{:?} / {:?} = {:?}", length, time, velocity);
    println!("{length:?} + {length:?} = {more_mass:?}", length = length, more_mass = more_mass);
}
