use serde::{Deserialize, Serialize};

use crate::Skills;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct EquipmentSlots {
    pub deck: Option<Equipment>,
    pub forward: Option<Equipment>,
    pub auxiliary: Option<Equipment>,
    pub bridge: Option<Equipment>,
    pub aft: Option<Equipment>,
    pub engine: Option<Equipment>
}

impl EquipmentSlots {
    pub fn skills(&self) -> crate::Skills {
        let mut skills = crate::Skills::default();

        if let Some(equipment) = &self.deck { skills = skills + equipment.skills.clone() };
        if let Some(equipment) = &self.forward { skills = skills + equipment.skills.clone()};
        if let Some(equipment) = &self.auxiliary { skills = skills + equipment.skills.clone() };
        if let Some(equipment) = &self.bridge { skills = skills + equipment.skills.clone() };
        if let Some(equipment) = &self.aft { skills = skills + equipment.skills.clone() };
        if let Some(equipment) = &self.engine { skills = skills + equipment.skills.clone() };

        skills
    }

    pub fn engine_power(&self) -> i32 {
        let mut power = 0;

        if let Some(equipment) = &self.deck { power += equipment.engine_power };
        if let Some(equipment) = &self.forward { power += equipment.engine_power };
        if let Some(equipment) = &self.auxiliary { power += equipment.engine_power };
        if let Some(equipment) = &self.bridge { power += equipment.engine_power };
        if let Some(equipment) = &self.aft { power += equipment.engine_power };
        if let Some(equipment) = &self.engine { power += equipment.engine_power };

        power
    }

    pub fn fuel_efficiency(&self) -> i32 {
        let mut efficiency = 0;

        if let Some(equipment) = &self.deck { efficiency += equipment.engine_power };
        if let Some(equipment) = &self.forward { efficiency += equipment.engine_power };
        if let Some(equipment) = &self.auxiliary { efficiency += equipment.engine_power };
        if let Some(equipment) = &self.bridge { efficiency += equipment.engine_power };
        if let Some(equipment) = &self.aft { efficiency += equipment.engine_power };
        if let Some(equipment) = &self.engine { efficiency += equipment.engine_power };

        efficiency
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct Equipment {
    pub name: String,
    pub slot: Slot,
    pub skills: Skills,
    pub damage: Option<Damage>,
    pub engine_power: i32,
    pub fuel_efficiency: i32,
    pub quarters: i32
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Slot {
    Deck,
    Forward,
    Auxiliary,
    Bridge,
    Aft,
    Engine
}

impl Default for Slot {
    fn default() -> Self { Slot::Auxiliary }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct Damage {
    pub hull: f32,
    pub life: f32,
    pub crew: f32,
    pub warumup: f32,
    pub stagger: f32
}