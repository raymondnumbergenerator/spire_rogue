use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::{Component, ConvertSaveload};
use serde::{Serialize, Deserialize};

// Items can be picked up off the ground
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Item {}

// Potions are added to the inventory when acquired and are consumable
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Potion {}

#[derive(Component, Debug, ConvertSaveload)]
pub struct InBackpack {
    pub owner: Entity
}

// Cards are added to the deck when acquired and can be played
#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Card {
    pub energy_cost: i32
}

// Ethereal cards are destroyed when discarded
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Ethereal{}

// Fragile cards are destroyed when played
#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Fragile{}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Targeted {
    pub range: i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct AreaOfEffect {
    pub radius: i32
}