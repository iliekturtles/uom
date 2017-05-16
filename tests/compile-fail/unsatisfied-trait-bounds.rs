extern crate uom;

fn main() {
    let l1 = uom::si::f32::Length::new::<uom::si::length::meter>(4.0).sqrt();
    //~^ ERROR the trait bound `uom::si::ISQ<
}

