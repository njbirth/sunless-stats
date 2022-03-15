use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct Statistics {
    pub weight: i32,
    pub hold: i32,
    pub quarters: i32,
    pub hull: i32,
    pub engine_power: i32,
    pub skills: crate::Skills,
}