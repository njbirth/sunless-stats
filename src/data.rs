use std::fs;

use crate::ship::Shiptype;
use crate::officers::Officer;
use crate::equipment::Equipment;

lazy_static! {
    pub static ref SHIPTYPES: Vec<Shiptype> = ron::from_str(&read("./data/ships.ron")).unwrap();
    pub static ref OFFICERS: Vec<Officer> = ron::from_str(&read("./data/officers.ron")).unwrap();
    pub static ref EQUIPMENT: Vec<Equipment> = ron::from_str(&read("./data/equipment.ron")).unwrap();
}

fn read(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&format!("Couldn't read file {}", path))
}