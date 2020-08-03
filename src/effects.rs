use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::{Component, ConvertSaveload};
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct DealDamage {
    pub amount: i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct GainBlock {
    pub amount: i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct DiscardCard {
    pub number: i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct DrawCard {
    pub number: i32
}