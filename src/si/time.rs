//! Time (base unit second, s^(1)).

use Quantity;
use si::ISQ;
use typenum::{P1, Z0};

/// Time dimension, s^(1).
pub type Dimension = ISQ<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
/// Time (base unit second, s^(1)).
pub type Time<U, V> = Quantity<Dimension, U, V>;
