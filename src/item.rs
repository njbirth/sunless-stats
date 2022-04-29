use std::fmt::{self, Display, Formatter};
use crate::equipment::{Equipment, EquipmentType};
use crate::officers::{Officer, Position};

// TODO: module item, submodules officer and equipment
#[derive(PartialEq, Clone)]
pub enum Item {
    Equipment(Equipment),
    Officer(Officer),
    None
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Item::Equipment(item) => write!(f, "{}", item),
            Item::Officer(item) => write!(f, "{}", item),
            Item::None => write!(f, "None")
        }
    }
}

#[derive(Clone)]
pub enum Slot {
    Equipment(EquipmentType),
    Officer(Position)
}

impl Display for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Slot::Equipment(e) => format!("{}", e),
            Slot::Officer(p) => format!("{}", p)
        };

        write!(f, "{}", s)
    }
}