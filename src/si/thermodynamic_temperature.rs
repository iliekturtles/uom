//! Thermodynamic temperature (base unit kelvin, K^(1)).

use si::{ISQ, Quantity};
use typenum::{P1, Z0};

/// Thermodynamic temperature dimension, K^(1).
pub type Dimension = ISQ<Z0, Z0, Z0, Z0, P1, Z0, Z0>;
/// Thermodynamic temperature (base unit kelvin, K^(1)).
pub type ThermodynamicTemperature<U, V> = Quantity<Dimension, U, V>;
