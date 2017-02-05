//! Amount of substance (base unit mole, mol^(1)).

use si::{ISQ, Quantity};
use typenum::{P1, Z0};

/// Amount of substance dimension, mol^(1).
pub type Dimension = ISQ<Z0, Z0, Z0, Z0, Z0, P1, Z0>;
/// Amount of substance (base unit mole, mol^(1)).
pub type AmountOfSubstance<U, V> = Quantity<Dimension, U, V>;
