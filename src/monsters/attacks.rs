use specs::prelude::*;
use serde::{Serialize, Deserialize};

use super::super::{
    Name, creature, effects, item, status,
};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Attacks {
    NormalAttack{ name: String, range: i32, amount: i32 },
    GainBlock{ name: String, range: i32, amount: i32 },
    AttackAndBlock{ name: String, range: i32, damage_amount: i32, block_amount: i32 },
    ApplyWeak{ name: String, range: i32, turns: i32 },
    ApplyFrail{ name: String, range: i32, turns: i32 },
    BuffStrength{ name: String, range: i32, amount: i32 },
    BlockAndBuffStrength{ name: String, range: i32, block_amount: i32, buff_amount: i32 },
    AttackAndGiveCard{ name: String, range: i32, amount: i32, card: effects::GainableCard, number: i32 },
}

pub fn build_attack(ecs: &mut World, attack: Attacks) -> EntityBuilder {
    let mut atk = ecs.create_entity()
        .with(creature::Attack{});

    match attack {
        Attacks::NormalAttack{name, range, amount } => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(effects::DealDamage{ amount });
        }
        Attacks::GainBlock{name, range, amount } => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(effects::GainBlock{ amount });
        }
        Attacks::AttackAndBlock{name, range, damage_amount, block_amount } => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(effects::DealDamage{ amount: damage_amount })
                .with(effects::GainBlock{ amount: block_amount });
        }
        Attacks::ApplyWeak{name, range, turns } => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(status::Weak{ turns });
        }
        Attacks::ApplyFrail{name, range, turns } => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(status::Frail{ turns });
        }
        Attacks::BuffStrength{name, range, amount } => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(effects::BuffStrength{ amount });
        }
        Attacks::BlockAndBuffStrength{name, range, block_amount, buff_amount } => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(effects::GainBlock{ amount: block_amount })
                .with(effects::BuffStrength{ amount: buff_amount });
        }
        Attacks::AttackAndGiveCard{name, range, amount, card, number } => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(effects::DealDamage{ amount })
                .with(effects::GainCard{ card, number, to_hand: false })
        }
    }

    atk
}