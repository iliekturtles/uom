//! Luminous intensity (base unit candela, cd^(1)).

use Quantity;
use si::ISQ;
use typenum::{P1, Z0};

/// Luminous intensity dimension, cd^(1).
pub type Dimension = ISQ<Z0, Z0, Z0, Z0, Z0, Z0, P1>;
/// Luminous intensity (base unit candela, cd^(1)).
pub type LuminousIntensity<U, V> = Quantity<Dimension, U, V>;
