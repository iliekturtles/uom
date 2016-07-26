extern crate typenum;
extern crate uom;

use std::default::{Default};
use uom::{Conversion};
use uom::si::f32::*;
use uom::si::velocity::{VelocitySubunits};

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
    //println!("{:?} m / s = {:?} f / s",
    //    Conversion::<f32, VelocitySubunits>::get(velocity, VelocitySubunits::meter_per_second),
    //    velocity.get(VelocitySubunits::foot_per_second));
}
