extern crate typenum;
extern crate uom;

use std::default::{Default};

fn main() {
    #[allow(unused_imports)]
    use typenum::{Z0, P1, P2, N1, N2};
    use uom::si::{System};

    let unitless = System::<f32, Z0, Z0, Z0, Z0, Z0, Z0, Z0>::default();
    let mass = System::<f32, P1, Z0, Z0, Z0, Z0, Z0, Z0>::default();
    let time = System::<f32, Z0, Z0, P1, Z0, Z0, Z0, Z0>::default();
    let velocity = mass / time;

    println!("unitless: {:?}", unitless);
    println!("mass: {:?}", mass);
    println!("time: {:?}", time);
    println!("velocity: {:?}", velocity);
}
