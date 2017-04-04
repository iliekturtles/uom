extern crate uom;

fn main() {
    let l1 = uom::si::f32::Length::new::<uom::si::time::second>(15.0);
    //~^ ERROR the trait bound `uom::si::time::second: uom::si::length::Unit<f32>` is not satisfied
}
