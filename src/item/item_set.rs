use serde::{Deserialize, Serialize};
use super::{Item, Slot};
use crate::Skills;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(default)]
pub struct ItemSet {
    pub deck: Option<Item>,
    pub forward: Option<Item>,
    pub auxiliary: Option<Item>,
    pub bridge: Option<Item>,
    pub aft: Option<Item>,
    pub engine: Option<Item>,
    pub cook: Option<Item>,
    pub engineer: Option<Item>,
    pub first_officer: Option<Item>,
    pub gunner: Option<Item>,
    pub surgeon: Option<Item>,
    pub mascot: Option<Item>
}

impl Default for ItemSet {
    fn default() -> Self {
        ItemSet {
            deck: None,
            forward: None,
            auxiliary: None,
            bridge: None,
            aft: None,
            engine: Some(crate::data::items(&Slot::Engine)[0].clone()),
            cook: None,
            engineer: None,
            first_officer: None,
            gunner: None,
            surgeon: None,
            mascot: None
        }
    }
}

impl ItemSet {
    fn all_items(&self) -> Vec<&Item> {
        let v = vec![
            &self.deck, &self.forward, &self.auxiliary, &self.bridge, &self.aft, &self.engine,
            &self.cook, &self.engineer, &self.first_officer, &self.gunner, &self.surgeon, &self.mascot
        ];

        v.into_iter()
            .filter(|i| i.is_some())
            .map(|i| i.as_ref().unwrap())
            .collect()
    }

    pub fn item(&self, slot: &Slot) -> Option<Item> {
        match slot {
            Slot::Deck => self.deck.clone(),
            Slot::Forward => self.forward.clone(),
            Slot::Auxiliary => self.auxiliary.clone(),
            Slot::Bridge => self.bridge.clone(),
            Slot::Aft => self.aft.clone(),
            Slot::Engine => self.engine.clone(),
            Slot::Cook => self.cook.clone(),
            Slot::Engineer => self.engineer.clone(),
            Slot::FirstOfficer => self.first_officer.clone(),
            Slot::Gunner => self.gunner.clone(),
            Slot::Surgeon => self.surgeon.clone(),
            Slot::Mascot => self.mascot.clone()
        }
    }

    pub fn set_item(&mut self, slot: &Slot, item: Option<Item>) {
        match slot {
            Slot::Deck => self.deck = item,
            Slot::Forward => self.forward = item,
            Slot::Auxiliary => self.auxiliary = item,
            Slot::Bridge => self.bridge = item,
            Slot::Aft => self.aft = item,
            Slot::Engine => self.engine = item,
            Slot::Cook => self.cook = item,
            Slot::Engineer => self.engineer = item,
            Slot::FirstOfficer => self.first_officer = item,
            Slot::Gunner => self.gunner = item,
            Slot::Surgeon => self.surgeon = item,
            Slot::Mascot => self.mascot = item
        };
    }

    pub fn skills(&self) -> Skills {
        self.all_items().iter()
            .fold(Skills::default(), |acc, i| acc + i.skills.clone())
    }

    pub fn engine_power(&self) -> i32 {
        self.all_items().iter().fold(0, |acc, i| acc + i.engine_power)
    }

    pub fn fuel_efficiency(&self) -> i32 {
        self.all_items().iter().fold(0, |acc, i| acc + i.fuel_efficiency)
    }

    pub fn quarters(&self) -> i32 {
        self.all_items().iter().fold(0, |acc, i| acc + i.quarters)
    }
}