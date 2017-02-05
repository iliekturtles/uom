//! Length (base unit meter, m^(1)).

use Quantity;
use si::ISQ;
use typenum::{P1, Z0};

/// Length dimension, m^(1).
pub type Dimension = ISQ<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
/// Length (base unit meter, m^(1)).
pub type Length<U, V> = Quantity<Dimension, U, V>;
