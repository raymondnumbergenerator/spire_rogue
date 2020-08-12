use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::{Component, ConvertSaveload};
use serde::{Serialize, Deserialize};

use super::super::{monsters};

use rltk::RandomNumberGenerator;

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
    pub block: i32,
    pub base_strength: i32,
    pub strength: i32,
    pub base_dexterity: i32,
    pub dexterity: i32,
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
    pub cycle: usize,

    weights: Option<Vec<i32>>,
    total_weight: i32,
}

impl AttackCycle {
    pub fn new_sequential() -> AttackCycle {
        AttackCycle {
            attacks: Vec::new(),
            cycle: 0,
            weights: None,
            total_weight: 0,
        }
    }

    pub fn new_weighted() -> AttackCycle {
        AttackCycle {
            attacks: Vec::new(),
            cycle: 0,
            weights: Some(Vec::new()),
            total_weight: 0,
        }
    }

    pub fn add_weighted(mut self, attack: monsters::Attacks, weight: i32) -> AttackCycle {
        if let None = self.weights { panic!("Attempted to add weighted attacks to sequential attack cycle.") }

        self.attacks.push(attack);
        let mut new_weights = self.weights.unwrap();
        new_weights.push(weight);
        self.total_weight += weight;
        self.weights = Some(new_weights);

        self
    }

    pub fn add_sequential(mut self, attack: monsters::Attacks) -> AttackCycle {
        if let Some(_) = self.weights { panic!("Attempted to add sequential attacks to weighted attack cycle.") }

        self.attacks.push(attack);

        self
    }

    pub fn next_attack(&mut self, rng: &mut RandomNumberGenerator) {
        match &self.weights {
            Some(w) => {
                let mut roll = rng.roll_dice(1, self.total_weight) - 1;
                let mut next_cycle = 0;

                while roll > 0 {
                    if roll < w[next_cycle] {
                        self.cycle = next_cycle;
                        return;
                    }
                    roll -= w[next_cycle];
                    next_cycle += 1;
                }
            }
            None => {
                self.cycle = (self.cycle + 1) & self.attacks.len();
            }
        }
    }
}