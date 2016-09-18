#[macro_use] pub mod prefix;

pub mod amount_of_substance;
pub mod electric_current;
pub mod length;
pub mod luminous_intensity;
pub mod mass;
pub mod thermodynamic_temperature;
pub mod time;
pub mod velocity;

system!(SI; SIUnits:
    dimensions {
        length: L, LengthUnit;
        mass: M, MassUnit;
        time: T, TimeUnit;
        electric_current: I, ElectricCurrentUnit;
        thermodynamic_temperature: Th, ThermodynamicTemperatureUnit;
        amount_of_substance: N, AmountOfSubstanceUnit;
        luminous_intensity: J, LuminousIntensityUnit;
    });

pub type BaseUnits = SIUnits<
    length::units::meter,
    mass::units::kilogram,
    time::units::second,
    electric_current::units::ampere,
    thermodynamic_temperature::units::kelvin,
    amount_of_substance::units::mole,
    luminous_intensity::units::candela>;

pub mod f32 {
    pub type B = super::BaseUnits;
    pub type V = f32;

    pub type AmountOfSubstance = super::amount_of_substance::AmountOfSubstance<B, V>;
    pub type ElectricCurrent = super::electric_current::ElectricCurrent<B, V>;
    pub type Length = super::length::Length<B, V>;
    pub type LuminousIntensity = super::luminous_intensity::LuminousIntensity<B, V>;
    pub type Mass = super::mass::Mass<B, V>;
    pub type ThermodynamicTemperature = super::thermodynamic_temperature::ThermodynamicTemperature<B, V>;
    pub type Time = super::time::Time<B, V>;
    pub type Velocity = super::velocity::Velocity<B, V>;
}

//quantities!(
//    mods {
//        f32: f32,
//        f64: f32,
//    }
//    quantities {
//        amount_of_substance: AmountOfSubstance,
//        electric_current: ElectricCurrent,
//        length: Length,
//        luminous_intensity: LuminousIntensity,
//        mass: Mass,
//        thermodynamic_temperature: ThermodynamicTemperature,
//        time: Time,
//        velocity: Velocity,
//    });
