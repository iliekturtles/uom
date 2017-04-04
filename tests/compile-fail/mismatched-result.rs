extern crate uom;

use uom::si::f32::*;

fn main() {
    let _: Velocity = Length::new::<uom::si::length::meter>(1.0) *
                      Time::new::<uom::si::time::second>(1.0);
    //~^^ ERROR mismatched types [E0308]
}
