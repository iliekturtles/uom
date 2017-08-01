extern crate uom;

fn main() {
    let l1 = uom::si::f32::Area::new::<uom::si::area::square_meter>(8.0).cbrt();
    //~^ ERROR no method named `cbrt` found for type `uom::si::Quantity<
    let l2 = uom::si::f32::Length::new::<uom::si::length::meter>(4.0).sqrt();
    //~^ ERROR no method named `sqrt` found for type `uom::si::Quantity<
}

