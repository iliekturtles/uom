extern crate typenum;
extern crate uom;

fn main() {
    use std::marker::{PhantomData};
    use typenum::{Z0, P1, P2, N1, N2};
    use uom::si::{Dimensions};

    let unitless = Dimensions::<>::new();
    let mass = Dimensions::<P1, Z0, Z0, Z0, Z0, Z0, Z0>::new();
    let time = Dimensions::<Z0, Z0, P1, Z0, Z0, Z0, Z0>::new();
    let velocity = mass / time;

    println!("unitless: {:?}", unitless);
    println!("mass: {:?}", mass);
    println!("time: {:?}", time);
    println!("velocity: {:?}", velocity);
}
