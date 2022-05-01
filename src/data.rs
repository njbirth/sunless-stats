use crate::ship::Shiptype;
use crate::item::{Item, Slot};

lazy_static! {
    pub static ref SHIPTYPES: Vec<Shiptype> = ron::from_str(&read("./data/ships.ron")).unwrap();
    pub static ref ITEMS: Vec<Item> = {
        let mut items: Vec<Item> = ron::from_str(&read("./data/equipment.ron")).unwrap();
        items.append(&mut ron::from_str(&read("./data/officers.ron")).unwrap());

        items
    };
}

fn read(path: &str) -> String {
    std::fs::read_to_string(path)
        .expect(&format!("Couldn't read file {}", path))
}

pub fn items(slot: &Slot) -> Vec<&'static Item> {
    ITEMS.iter()
        .filter(|item| &item.slot == slot)
        .collect()
}