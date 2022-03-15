use std::fmt::{self, Display, Formatter};
use serde::{Deserialize, Serialize};

use super::Statistics;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Shiptype {
    pub name: String,
    pub stats: Statistics
}

impl Display for Shiptype {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for Shiptype {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Shiptype { }