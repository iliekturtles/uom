#[macro_use] pub mod prefix;

#[macro_use] pub mod amount_of_substance;
#[macro_use] pub mod electric_current;
#[macro_use] pub mod length;
#[macro_use] pub mod luminous_intensity;
#[macro_use] pub mod mass;
#[macro_use] pub mod thermodynamic_temperature;
#[macro_use] pub mod time;
#[macro_use] pub mod velocity;

pub use self::amount_of_substance::{AmountOfSubstance};
pub use self::electric_current::{ElectricCurrent};
pub use self::length::{Length};
pub use self::luminous_intensity::{LuminousIntensity};
pub use self::mass::{Mass};
pub use self::thermodynamic_temperature::{ThermodynamicTemperature};
pub use self::time::{Time};
pub use self::velocity::{Velocity};

system!(SI:
    dimensions {
        length: L, m;
        mass: M, kg;
        time: T, s;
        electric_current: I, A;
        thermodynamic_temperature: Th, K;
        amount_of_substance: N, mol;
        luminous_intensity: J, cd;
    });

pub mod f32 {
    use core::ops::{Div, Mul};
    use ::{Conversion};

    pub type U = (super::length::meter, super::mass::kilogram, super::time::second,
        super::electric_current::ampere, super::thermodynamic_temperature::kelvin,
        super::amount_of_substance::mole, super::luminous_intensity::candela);
    pub type V = f32;

    impl ::Units for U {}

    amount_of_substance!();
    electric_current!();
    length!();
    luminous_intensity!();
    mass!();
    //thermodynamic_temperature!();
    time!();
    velocity!();
}

pub mod f64 {
    use core::ops::{Div, Mul};
    use ::{Conversion};

    pub type U = (super::length::meter, super::mass::kilogram, super::time::second,
        super::electric_current::ampere, super::thermodynamic_temperature::kelvin,
        super::amount_of_substance::mole, super::luminous_intensity::candela);
    pub type V = f64;

    impl ::Units for U {}

    amount_of_substance!();
    electric_current!();
    length!();
    luminous_intensity!();
    mass!();
    //thermodynamic_temperature!();
    time!();
    velocity!();
}
