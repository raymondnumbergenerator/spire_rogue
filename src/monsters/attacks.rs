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
    BuffStrength{ name: String, amount: i32, range: i32 },
    BlockAndBuffStrength{ name: String, block_amount: i32, buff_amount: i32, range: i32},
    AttackAndGiveCard{ name: String, amount: i32, card: effects::GainableCard, number: i32, range: i32 },
}

pub fn build_attack(ecs: &mut World, attack: Attacks) -> EntityBuilder {
    let mut atk = ecs.create_entity()
        .with(creature::Attack{});

    match attack {
        Attacks::NormalAttack{name, amount, range} => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(effects::DealDamage{ amount });
        }
        Attacks::GainBlock{name, amount, range} => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(effects::GainBlock{ amount });
        }
        Attacks::AttackAndBlock{name, damage_amount, block_amount, range} => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(effects::DealDamage{ amount: damage_amount })
                .with(effects::GainBlock{ amount: block_amount });
        }
        Attacks::ApplyWeak{name, turns, range} => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(status::Weak{ turns });
        }
        Attacks::BuffStrength{name, amount, range} => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(effects::BuffStrength{ amount });
        }
        Attacks::BlockAndBuffStrength{name, block_amount, buff_amount, range} => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(effects::GainBlock{ amount: block_amount })
                .with(effects::BuffStrength{ amount: buff_amount });
        }
        Attacks::AttackAndGiveCard{name, amount, card, number, range} => {
            atk = atk.with(Name{ name: name.to_string() })
                .with(item::Targeted{ range })
                .with(effects::DealDamage{ amount })
                .with(effects::GainCard{ card, number, to_hand: false })
        }
    }

    atk
}