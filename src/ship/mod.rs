use crate::equipment::{Equipment, EquipmentSlots, EquipmentType};
use crate::officers::{Officer, Officers, Position};
use crate::Skills;
use crate::data::*;

mod shiptype;
pub use shiptype::Shiptype;

mod statistics;
pub use statistics::Statistics;
use crate::item::{Item, Slot};

#[derive(Clone, PartialEq)]
// TODO: Implement Default properly
pub struct Ship {
    pub shiptype: Shiptype,
    pub captain_skills: Skills,
    pub equipment: EquipmentSlots,
    pub officers: Officers,
    pub crew: i32,
}

impl Ship {
    pub fn item(&self, slot: &Slot) -> Item {
        match slot {
            Slot::Officer(p) => if let Some(officer) = self.officer(p) {
                Item::Officer(officer)
            } else { Item::None },
            Slot::Equipment(t) => if let Some(equipment) = self.equipment(t) {
                Item::Equipment(equipment)
            } else { Item::None }
        }
    }

    pub fn set_item(&mut self, slot: &Slot, item: Item) {
        match slot {
            Slot::Officer(p) => if let Item::Officer(officer) = item {
                self.set_officer(p, Some(officer));
            } else { self.set_officer(p, None); },
            Slot::Equipment(t) => if let Item::Equipment(equipment) = item {
                self.set_equipment(t, Some(equipment));
            } else { self.set_equipment(t, None); }
        };
    }

    pub fn officer(&self, p: &Position) -> Option<Officer> {
        match p {
            Position::Cook => self.officers.cook.clone(),
            Position::Engineer => self.officers.engineer.clone(),
            Position::FirstOfficer => self.officers.first_officer.clone(),
            Position::Gunner => self.officers.gunner.clone(),
            Position::Surgeon => self.officers.surgeon.clone(),
            Position::Mascot => self.officers.mascot.clone(),
        }
    }

    pub fn set_officer(&mut self, p: &Position, officer: Option<Officer>) {
        match p {
            Position::Cook => self.officers.cook = officer,
            Position::Engineer => self.officers.engineer = officer,
            Position::FirstOfficer => self.officers.first_officer = officer,
            Position::Gunner => self.officers.gunner = officer,
            Position::Surgeon => self.officers.surgeon = officer,
            Position::Mascot => self.officers.mascot = officer,
        };
    }

    pub fn equipment(&self, t: &EquipmentType) -> Option<Equipment> {
        match t {
            EquipmentType::Deck => self.equipment.deck.clone(),
            EquipmentType::Forward => self.equipment.forward.clone(),
            EquipmentType::Auxiliary => self.equipment.auxiliary.clone(),
            EquipmentType::Bridge => self.equipment.bridge.clone(),
            EquipmentType::Aft => self.equipment.aft.clone(),
            EquipmentType::Engine => self.equipment.engine.clone()
        }
    }

    pub fn set_equipment(&mut self, t: &EquipmentType, equipment: Option<Equipment>) {
        match t {
            EquipmentType::Deck => self.equipment.deck = equipment,
            EquipmentType::Forward => self.equipment.forward = equipment,
            EquipmentType::Auxiliary => self.equipment.auxiliary = equipment,
            EquipmentType::Bridge => self.equipment.bridge = equipment,
            EquipmentType::Aft => self.equipment.aft = equipment,
            EquipmentType::Engine => self.equipment.engine = equipment
        };
    }

    pub fn engine_power(&self) -> i32 {
        self.shiptype.stats.engine_power
            + self.equipment.engine_power()
            + self.officers.engine_power()
    }

    pub fn fuel_efficiency(&self) -> i32 {
        self.equipment.fuel_efficiency()
            + self.officers.fuel_efficiency()
    }

    pub fn quarters(&self) -> i32 {
        self.shiptype.stats.quarters
            + self.equipment.quarters()
    }

    // Fuel consumption in percent/second
    // speed is either 1 or 2
    pub fn fuel_consumption(&self, speed: i8, light: bool) -> f32 {
        let speed = speed as f32;
        let light = if light { 1. } else { 0. };
        let fuel_efficiency = self.fuel_efficiency() as f32 / 100.;
        let engine_power = self.engine_power() as f32;

        (((engine_power * 0.0005) * (1. - fuel_efficiency)) * speed) + light
    }
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            shiptype: SHIPTYPES[1].clone(),
            captain_skills: Skills {
                iron: 25,
                veils: 25,
                mirrors: 25,
                pages: 25,
                hearts: 25
            },
            equipment: EquipmentSlots::default(),
            officers: Officers::default(),
            crew: SHIPTYPES[1].stats.quarters
        }
    }
}