use crate::equipment::EquipmentSlots;
use crate::officers::Officers;
use crate::Skills;

mod shiptype;
pub use shiptype::Shiptype;

mod statistics;
pub use statistics::Statistics;

#[derive(Default, Clone)]
// TODO: Implement Default properly
pub struct Ship {
    pub shiptype: Shiptype,
    pub captain_skills: Skills,
    pub equipment: EquipmentSlots,
    pub officers: Officers,
    pub crew: i32,
}

impl Ship {
    pub fn engine_power(&self) -> i32 {
        self.shiptype.stats.engine_power
            + self.equipment.engine_power()
            + self.officers.engine_power()
    }

    pub fn fuel_efficiency(&self) -> i32 {
        self.equipment.fuel_efficiency()
            + self.officers.fuel_efficiency()
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