pub mod prefix;

pub mod amount_of_substance;
pub mod electric_current;
pub mod length;
pub mod luminous_intensity;
pub mod mass;
pub mod thermodynamic_temperature;
pub mod time;
pub mod velocity;

pub use self::amount_of_substance::*;
pub use self::electric_current::*;
pub use self::length::*;
pub use self::luminous_intensity::*;
pub use self::mass::*;
pub use self::thermodynamic_temperature::*;
pub use self::time::*;
pub use self::velocity::*;

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
    pub type AmountOfSubstance = super::AmountOfSubstance<f32>;
    pub type ElectricCurrent = super::ElectricCurrent<f32>;
    pub type Length = super::Length<f32>;
    pub type LuminousIntensity = super::LuminousIntensity<f32>;
    pub type Mass = super::Mass<f32>;
    pub type ThermodynamicTemperature = super::ThermodynamicTemperature<f32>;
    pub type Time = super::Time<f32>;
    pub type Velocity = super::Velocity<f32>;
}

pub mod f64 {
    pub type AmountOfSubstance = super::AmountOfSubstance<f64>;
    pub type ElectricCurrent = super::ElectricCurrent<f64>;
    pub type Length = super::Length<f64>;
    pub type LuminousIntensity = super::LuminousIntensity<f64>;
    pub type Mass = super::Mass<f64>;
    pub type ThermodynamicTemperature = super::ThermodynamicTemperature<f64>;
    pub type Time = super::Time<f64>;
    pub type Velocity = super::Velocity<f64>;
}
