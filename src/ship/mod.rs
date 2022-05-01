use crate::Skills;
use crate::data::*;
use crate::item::{Item, ItemSet, Slot};

mod shiptype;
pub use shiptype::Shiptype;

mod statistics;
pub use statistics::Statistics;

#[derive(Clone, PartialEq)]
// TODO: Implement Default properly
pub struct Ship {
    pub shiptype: Shiptype,
    pub captain_skills: Skills,
    pub items: ItemSet,
    pub crew: i32,
}

impl Ship {
    pub fn item(&self, slot: &Slot) -> Option<Item> {
        self.items.item(slot)
    }

    pub fn set_item(&mut self, slot: &Slot, item: Option<Item>) {
        self.items.set_item(slot, item);
    }

    pub fn engine_power(&self) -> i32 {
        self.shiptype.stats.engine_power + self.items.engine_power()
    }

    pub fn fuel_efficiency(&self) -> i32 {
        self.items.fuel_efficiency()
    }

    pub fn quarters(&self) -> i32 {
        self.shiptype.stats.quarters + self.items.quarters()
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
            items: ItemSet::default(),
            crew: SHIPTYPES[1].stats.quarters
        }
    }
}