use std::fmt::{self, Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::Skills;

mod item_set;
mod slot;

pub use item_set::ItemSet;
pub use slot::Slot;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default)]
pub struct Item {
    pub name: String,
    pub img: String,
    pub slot: Slot,
    pub skills: Skills,
    pub engine_power: i32,
    pub fuel_efficiency: i32,
    pub damage: Option<Damage>,
    pub quarters: i32
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
#[serde(default)]
pub struct Damage {
    pub hull: f32,
    pub life: f32,
    pub crew: f32,
    pub warumup: f32,
    pub stagger: f32
}