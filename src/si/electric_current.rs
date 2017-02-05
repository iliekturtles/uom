//! Electric current (base unit ampere, A^(1)).

use si::{ISQ, Quantity};
use typenum::{P1, Z0};

/// Electric current dimension, A^(1).
pub type Dimension = ISQ<Z0, Z0, Z0, P1, Z0, Z0, Z0>;
/// Electric current (base unit ampere, A^(1)).
pub type ElectricCurrent<U, V> = Quantity<Dimension, U, V>;
