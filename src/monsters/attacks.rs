use specs::prelude::*;
use serde::{Serialize, Deserialize};

use super::super::{
    Name, creature, effects, item, status,
};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Attacks {
    NormalAttack{ name: String, amount: i32, range: i32 },
    GainBlock{ name: String, amount: i32, range: i32 },
    AttackAndBlock{ name: String, damage_amount: i32, block_amount: i32, range: i32 },
    ApplyWeak{ name: String, turns: i32, range: i32 },
}

pub fn build_attack(ecs: &mut World, attack: Attacks) -> EntityBuilder {
    let mut atk = ecs.create_entity()
        .with(creature::Attack{});

    match attack {
        Attacks::NormalAttack{name, amount, range} => {
            atk = atk.with(Name{ name: name.to_string() });
            atk = atk.with(item::Targeted{ range });
            atk = atk.with(effects::DealDamage{ amount });
        }
        Attacks::GainBlock{name, amount, range} => {
            atk = atk.with(Name{ name: name.to_string() });
            atk = atk.with(item::Targeted{ range });
            atk = atk.with(effects::GainBlock{ amount });
        }
        Attacks::AttackAndBlock{name, damage_amount, block_amount, range} => {
            atk = atk.with(Name{ name: name.to_string() });
            atk = atk.with(item::Targeted{ range });
            atk = atk.with(effects::DealDamage{ amount: damage_amount });
            atk = atk.with(effects::GainBlock{ amount: block_amount });
        }
        Attacks::ApplyWeak{name, turns, range} => {
            atk = atk.with(Name{ name: name.to_string() });
            atk = atk.with(item::Targeted{ range });
            atk = atk.with(status::Weak{ turns });
        }
    }

    atk
}