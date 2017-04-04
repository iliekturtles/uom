extern crate uom;

fn main() {
    let l1 = uom::si::f32::Length::new::<uom::si::length::meter>(15.0);
    let t1 = uom::si::f32::Time::new::<uom::si::time::second>(50.0);
    let error = l1 + t1; //~ ERROR mismatched types [E0308]
}
