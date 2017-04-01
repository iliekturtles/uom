extern crate uom;

fn main() {
    let l1 = uom::si::f32::Length::new(15.0, uom::si::length::meter);
    let t1 = uom::si::f32::Time::new(50.0, uom::si::time::second);
    let error = l1 + t1; //~ ERROR mismatched types [E0308]
}
