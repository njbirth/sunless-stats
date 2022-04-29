use std::fs;

use crate::ship::Shiptype;
use crate::officers::{Officer, Position};
use crate::equipment::{Equipment, EquipmentType};
use crate::item::{Item, Slot};

lazy_static! {
    pub static ref SHIPTYPES: Vec<Shiptype> = ron::from_str(&read("./data/ships.ron")).unwrap();
    pub static ref OFFICERS: Vec<Officer> = ron::from_str(&read("./data/officers.ron")).unwrap();
    pub static ref EQUIPMENT: Vec<Equipment> = ron::from_str(&read("./data/equipment.ron")).unwrap();
}

fn read(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&format!("Couldn't read file {}", path))
}

pub fn officers(p: &Position) -> Vec<&'static Officer> {
    OFFICERS.iter()
        .filter(|o| {
            &o.position == p
        })
        .collect()
}

pub fn equipment(e: &EquipmentType) -> Vec<&'static Equipment> {
    EQUIPMENT.iter()
        .filter(|eq| {
            &eq.slot == e
        })
        .collect()
}

pub fn items(slot: &Slot) -> Vec<Item> {
    match slot {
        Slot::Officer(p) => {
            officers(p).iter().map(|o| Item::Officer(o.to_owned().to_owned())).collect()
        }
        Slot::Equipment(t) => {
            equipment(t).iter().map(|e| Item::Equipment(e.to_owned().to_owned())).collect()
        }
    }
}