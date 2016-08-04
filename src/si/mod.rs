pub mod prefix;

#[macro_use]
pub mod amount_of_substance;
pub mod electric_current;
pub mod length;
pub mod luminous_intensity;
pub mod mass;
pub mod thermodynamic_temperature;
pub mod time;
pub mod velocity;

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
    use super::prefix::*;

    pub type V = f32;

    //pub type AmountOfSubstance = super::AmountOfSubstance<f32>;
    //pub type ElectricCurrent = super::ElectricCurrent<f32>;
    //pub type Length = super::Length<f32>;
    //pub type LuminousIntensity = super::LuminousIntensity<f32>;
    //pub type Mass = super::Mass<f32>;
    //pub type ThermodynamicTemperature = super::ThermodynamicTemperature<f32>;
    //pub type Time = super::Time<f32>;
    //pub type Velocity = super::Velocity<f32>;
    amount_of_substance!();
}

//pub mod f64 {
//    pub type AmountOfSubstance = super::AmountOfSubstance<f64>;
//    pub type ElectricCurrent = super::ElectricCurrent<f64>;
//    pub type Length = super::Length<f64>;
//    pub type LuminousIntensity = super::LuminousIntensity<f64>;
//    pub type Mass = super::Mass<f64>;
//    pub type ThermodynamicTemperature = super::ThermodynamicTemperature<f64>;
//    pub type Time = super::Time<f64>;
//    pub type Velocity = super::Velocity<f64>;
//}
