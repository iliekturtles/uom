extern crate uom;

fn main() {
    let l1 = uom::si::f32::Area::new::<uom::si::area::square_meter>(8.0).cbrt();
    //~^ ERROR type mismatch resolving `<uom::
    let l2 = uom::si::f32::Length::new::<uom::si::length::meter>(4.0).sqrt();
    //~^ ERROR type mismatch resolving `<uom::
}

