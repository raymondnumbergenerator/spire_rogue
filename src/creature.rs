use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::{Component, ConvertSaveload};
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Creature {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Player {
    pub max_energy: i32,
    pub energy: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Monster {}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub dexterity: i32,
    pub strength: i32,
    pub block: i32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct BlocksTile {}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Viewshed {
    pub visible_tiles : Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct SufferDamage {
    pub amount: Vec<i32>
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage{ amount: vec![amount] };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}