use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::{Component, ConvertSaveload};
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Item {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Card {
    pub energy_cost: i32
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Potion {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Ethereal{}

#[derive(Component, Debug, ConvertSaveload)]
pub struct InBackpack {
    pub owner: Entity
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Targeted {
    pub range: i32
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct SelfTargeted {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct AreaOfEffect {
    pub radius: i32
}