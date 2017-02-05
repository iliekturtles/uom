//! [International System of Units][si] (SI) and [International System of Quantities][isq] (ISQ)
//! implementations.
//!
//! [si]: http://jcgm.bipm.org/vim/en/1.16.html
//! [isq]: http://jcgm.bipm.org/vim/en/1.6.html

pub mod amount_of_substance;
pub mod electric_current;
pub mod length;
pub mod luminous_intensity;
pub mod mass;
pub mod thermodynamic_temperature;
pub mod time;

system! {
    /// [International System of Quantities](http://jcgm.bipm.org/vim/en/1.6.html) (ISQ).
    quantities: ISQ {
        length: L,
        mass: M,
        time: T,
        electric_current: I,
        thermodynamic_temperature: Th,
        amount_of_substance: N,
        luminous_intensity: J,
    }
}
