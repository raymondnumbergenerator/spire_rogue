use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::{Component, ConvertSaveload};
use serde::{Serialize, Deserialize};

use super::super::cards;

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

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum GainableCard {
    Shiv,
    Slimed,
}

pub fn gain_card(ecs: &mut World, card: GainableCard) -> Entity {
    match card {
        GainableCard::Shiv => { cards::neutral::shiv(ecs) }
        GainableCard::Slimed => { cards::neutral::slimed(ecs) }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GainCardQueue {
    pub to_hand: Vec<GainableCard>,
    pub to_discard: Vec<GainableCard>,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct GainCard {
    pub card: GainableCard,
    pub number: i32,
    pub to_hand: bool,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct BuffStrength {
    pub amount: i32
}