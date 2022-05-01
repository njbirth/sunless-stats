#[macro_use] extern crate lazy_static;

use serde::{Deserialize, Serialize};
use std::ops::Add;

pub mod ship;
pub mod data;
pub mod gui;
pub mod item;

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct Skills {
    pub iron: i32,
    pub mirrors: i32,
    pub veils: i32,
    pub pages: i32,
    pub hearts: i32
}

impl Add for Skills {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            iron: self.iron + other.iron,
            mirrors: self.mirrors + other.mirrors,
            veils: self.veils + other.veils,
            pages: self.pages + other.pages,
            hearts: self.hearts + other.hearts
        }
    }
}