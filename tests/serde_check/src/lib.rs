use serde::{Deserialize, Serialize};
use uom::si::rational64::Frequency;

#[derive(Serialize, Deserialize)]
pub struct Container {
    pub frequency: Frequency,
}
