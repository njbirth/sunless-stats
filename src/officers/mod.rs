use std::fmt;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default)]
pub struct Officers {
    pub cook: Option<Officer>,
    pub engineer: Option<Officer>,
    pub first_officer: Option<Officer>,
    pub gunner: Option<Officer>,
    pub surgeon: Option<Officer>,
    pub mascot: Option<Officer>
}

impl Officers {
    pub fn skills(&self) -> crate::Skills {
        let mut skills = crate::Skills::default();

        if let Some(officer) = &self.cook { skills = skills + officer.skills.clone() };
        if let Some(officer) = &self.engineer { skills = skills + officer.skills.clone()};
        if let Some(officer) = &self.first_officer { skills = skills + officer.skills.clone() };
        if let Some(officer) = &self.gunner { skills = skills + officer.skills.clone() };
        if let Some(officer) = &self.surgeon { skills = skills + officer.skills.clone() };
        if let Some(officer) = &self.mascot { skills = skills + officer.skills.clone() };

        skills
    }

    pub fn engine_power(&self) -> i32 {
        let mut power = 0;

        if let Some(officer) = &self.cook { power += officer.engine_power };
        if let Some(officer) = &self.engineer { power += officer.engine_power };
        if let Some(officer) = &self.first_officer { power += officer.engine_power };
        if let Some(officer) = &self.gunner { power += officer.engine_power };
        if let Some(officer) = &self.surgeon { power += officer.engine_power };
        if let Some(officer) = &self.mascot { power += officer.engine_power };

        power
    }

    pub fn fuel_efficiency(&self) -> i32 {
        let mut efficiency = 0;

        if let Some(officer) = &self.cook { efficiency += officer.fuel_efficiency };
        if let Some(officer) = &self.engineer { efficiency += officer.fuel_efficiency };
        if let Some(officer) = &self.first_officer { efficiency += officer.fuel_efficiency };
        if let Some(officer) = &self.gunner { efficiency += officer.fuel_efficiency };
        if let Some(officer) = &self.surgeon { efficiency += officer.fuel_efficiency };
        if let Some(officer) = &self.mascot { efficiency += officer.fuel_efficiency };

        efficiency
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct Officer {
    pub name: String,
    pub img: String,
    pub position: Position,
    pub skills: crate::Skills,
    pub engine_power: i32,
    pub fuel_efficiency: i32,
}

impl Display for Officer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Position {
    Cook,
    Engineer,
    FirstOfficer,
    Gunner,
    Surgeon,
    Mascot
}

impl Default for Position {
    fn default() -> Self { Position::Mascot }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Position::Cook => "Cook",
            Position::Engineer => "Chief Engineer",
            Position::FirstOfficer => "First Officer",
            Position::Gunner => "Gunnery Officer",
            Position::Surgeon => "Surgeon",
            Position::Mascot => "Mascot"
        };

        write!(f, "{}", s)
    }
}