extern crate uom;

use uom::si::f32::*;

fn main() {
    let _: Velocity = Length::new(1.0, uom::si::length::meter) *
                      Time::new(1.0, uom::si::time::second);
    //~^^ ERROR mismatched types [E0308]
}
