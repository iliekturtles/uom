use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dimensions {
    length: i8,
    mass: i8,
    time: i8,
    electric_current: i8,
    thermodynamic_temperature: i8,
    amount_of_substance: i8,
    luminous_intensity: i8,
}

impl Add<Dimensions> for Dimensions {
    type Output = Dimensions;

    fn add(self, rhs: RHS) -> Self::Output {
        Dimensions {
            length: self.length + rhs.length,
            mass: self.mass + rhs.mass,
            time: self.time + rhs.time,
            electric_current: self.electric_current + rhs.electric_current,
            thermodynamic_temperature: self.thermodynamic_temperature + rhs.thermodynamic_temperature,
            amount_of_substance: self.amount_of_substance + rhs.amount_of_substance,
            luminous_intensity: self.luminous_intensity + rhs.luminous_intensity,
        }
    }
}

impl<'a> Add<Dimensions> for &'a Dimensions {
    type Output =
}

impl AddAssign for Dimensions {
    fn add_assign(&mut self, rhs: Rhs) {
        self.length += rhs.length;
        self.mass += rhs.mass;
        self.time += rhs.time;
        self.electric_current += rhs.electric_current;
        self.thermodynamic_temperature += rhs.thermodynamic_temperature;
        self.amount_of_substance += rhs.amount_of_substance;
        self.luminous_intensity += rhs.luminous_intensity;
    }
}
