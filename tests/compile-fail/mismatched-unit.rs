extern crate uom;

fn main() {
    let l1 = uom::si::f32::Length::new(15.0, uom::si::time::second);
    //~^ ERROR the trait bound `uom::si::time::second: uom::si::length::Unit<f32>` is not satisfied
}
