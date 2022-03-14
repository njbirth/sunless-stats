use serde::{Deserialize, Serialize};

use super::Statistics;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Shiptype {
    pub name: String,
    pub stats: Statistics
}