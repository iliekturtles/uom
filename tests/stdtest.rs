extern crate uom;

use uom::si::amount_of_substance::mole;
use uom::si::electric_current::ampere;
use uom::si::f32::*;
use uom::si::length::meter;
use uom::si::luminous_intensity::candela;
use uom::si::mass::kilogram;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::time::second;

#[test]
fn debug_fmt() {
    assert_eq!(format!("{:?} m^1", 1.0), format!("{:?}", Length::new::<meter>(1.0)));
    assert_eq!(format!("{:?} m^-1", 1.0), format!("{:?}", 1.0 / Length::new::<meter>(1.0)));
    assert_eq!(format!("{:.2?} m^1", 1.0), format!("{:.2?}", Length::new::<meter>(1.0)));
    assert_eq!(format!("{:?} m^1 kg^1 s^1 A^1 K^1 mol^1 cd^1", 1.23),
               format!("{:?}",
                       Length::new::<meter>(1.23) * Mass::new::<kilogram>(1.0) *
                       Time::new::<second>(1.0) *
                       ElectricCurrent::new::<ampere>(1.0) *
                       ThermodynamicTemperature::new::<kelvin>(1.0) *
                       AmountOfSubstance::new::<mole>(1.0) *
                       LuminousIntensity::new::<candela>(1.0)));
}
