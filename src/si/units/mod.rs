mod length;
mod mass;
mod time;

pub use self::length::*;
pub use self::mass::*;
pub use self::time::*;

pub mod f32 {
    pub type Length = super::Length<f32>;
    pub type Mass = super::Mass<f32>;
    pub type Time = super::Time<f32>;
}

pub mod f64 {
    pub type Length = super::Length<f64>;
    pub type Mass = super::Mass<f64>;
    pub type Time = super::Time<f64>;
}
