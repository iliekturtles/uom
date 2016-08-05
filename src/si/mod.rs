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

#[allow(non_upper_case_globals)]
pub mod f32 {
    use std::ops::{Div, Mul};
    use ::{Conversion};

    #[derive(Clone, Copy, Debug)]
    pub struct Base;

    pub type V = f32;
    pub type B = Base;

    const L: f32 = 1.0;
    const M: f32 = 1.0;
    const T: f32 = 1.0;
    const I: f32 = 1.0;
    const Th: f32 = 1.0;
    const N: f32 = 1.0;
    const J: f32 = 1.0;

    amount_of_substance!();
    electric_current!();
    length!();
    luminous_intensity!();
    mass!();
    //thermodynamic_temperature!();
    time!();
    velocity!();
}

//#[allow(non_upper_case_globals)]
//pub mod f64 {
//    use std::ops::{Div, Mul};
//    use ::{Conversion};

//    #[derive(Clone, Copy, Debug)]
//    pub struct Base;

//    pub type V = f64;
//    pub type B = Base;

//    const L: f32 = 1.0;
//    const M: f32 = 1.0;
//    const T: f32 = 1.0;
//    const I: f32 = 1.0;
//    const Th: f32 = 1.0;
//    const N: f32 = 1.0;
//    const J: f32 = 1.0;

//    amount_of_substance!();
//    electric_current!();
//    length!();
//    luminous_intensity!();
//    mass!();
//    thermodynamic_temperature!();
//    time!();
//    velocity!();
//}

#[allow(non_upper_case_globals)]
pub mod km_f32 {
    use std::ops::{Div, Mul};
    use ::{Conversion};

    #[derive(Clone, Copy, Debug)]
    pub struct Base;

    pub type V = f32;
    pub type B = Base;

    const L: f32 = 1.0 / prefix!(kilo);
    const M: f32 = 1.0;
    const T: f32 = 1.0;
    const I: f32 = 1.0;
    const Th: f32 = 1.0;
    const N: f32 = 1.0;
    const J: f32 = 1.0;

    amount_of_substance!();
    electric_current!();
    length!();
    luminous_intensity!();
    mass!();
    //thermodynamic_temperature!();
    time!();
    velocity!();
}
