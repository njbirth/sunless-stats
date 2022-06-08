use std::fmt::{self, Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::item::Slot;

use super::Statistics;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct Shiptype {
    pub name: String,
    pub stats: Statistics,
    pub locked_slots: Vec<Slot>
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