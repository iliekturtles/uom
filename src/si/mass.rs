//! Mass (base unit kilogram, kg^(1)).

use si::{ISQ, Quantity};
use typenum::{P1, Z0};

/// Mass dimension, kg^(1).
pub type Dimension = ISQ<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
/// Mass (base unit kilogram, kg^(1)).
pub type Mass<U, V> = Quantity<Dimension, U, V>;
