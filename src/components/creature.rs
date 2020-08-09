use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::{Component, ConvertSaveload};
use serde::{Serialize, Deserialize};

use super::super::{monsters};

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

#[derive(Component, Debug, ConvertSaveload)]
pub struct PerformAction {
    pub action: Entity,
    pub target: Option<rltk::Point>,
}

#[derive(Component, Debug, ConvertSaveload)]
pub struct PickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Attack{}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Intent {
    pub intent: Entity,
    pub used: bool,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct AttackCycle {
    pub attacks: Vec<monsters::Attacks>,

    // Set weights to None if attacks are sequential otherwise
    // indexes should match with the corresponding item in attacks
    pub weights: Option<Vec<i32>>,
    pub total_weight: i32,
    pub cycle: usize,
}

impl AttackCycle {
    pub fn new_sequential() -> AttackCycle {
        AttackCycle {
            attacks: Vec::new(),
            weights: None,
            total_weight: 0,
            cycle: 0,
        }
    }

    pub fn new_weighted() -> AttackCycle {
        AttackCycle {
            attacks: Vec::new(),
            weights: Some(Vec::new()),
            total_weight: 0,
            cycle: 0,
        }
    }

    pub fn add_weighted(&mut self, attack: monsters::Attacks, weight: i32) -> monsters::Attacks {
        self.attacks.push(attack.clone());
        let mut new_weights = self.weights.clone().unwrap();
        new_weights.push(weight);
        self.total_weight += weight;
        self.weights = Some(new_weights);

        attack
    }

    pub fn add_sequential(&mut self, attack: monsters::Attacks) -> monsters::Attacks {
        self.attacks.push(attack.clone());

        attack
    }
}