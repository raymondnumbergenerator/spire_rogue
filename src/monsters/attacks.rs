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

impl Attacks {
    // Creates and returns the associated atttack Entity
    pub fn to_attack(self, ecs: &mut World) -> Entity {
        let mut attack = ecs.create_entity()
            .with(creature::Attack{});
    
        match self {
            Attacks::NormalAttack{name, range, amount } => {
                attack = attack.with(Name{ name: name.to_string() })
                    .with(item::Targeted{ range })
                    .with(effects::DealDamage{ amount });
            }
            Attacks::GainBlock{name, range, amount } => {
                attack = attack.with(Name{ name: name.to_string() })
                    .with(item::Targeted{ range })
                    .with(effects::GainBlock{ amount });
            }
            Attacks::AttackAndBlock{name, range, damage_amount, block_amount } => {
                attack = attack.with(Name{ name: name.to_string() })
                    .with(item::Targeted{ range })
                    .with(effects::DealDamage{ amount: damage_amount })
                    .with(effects::GainBlock{ amount: block_amount });
            }
            Attacks::ApplyWeak{name, range, turns } => {
                attack = attack.with(Name{ name: name.to_string() })
                    .with(item::Targeted{ range })
                    .with(status::Weak{ turns });
            }
            Attacks::ApplyFrail{name, range, turns } => {
                attack = attack.with(Name{ name: name.to_string() })
                    .with(item::Targeted{ range })
                    .with(status::Frail{ turns });
            }
            Attacks::BuffStrength{name, range, amount } => {
                attack = attack.with(Name{ name: name.to_string() })
                    .with(item::Targeted{ range })
                    .with(effects::BuffStrength{ amount });
            }
            Attacks::BlockAndBuffStrength{name, range, block_amount, buff_amount } => {
                attack = attack.with(Name{ name: name.to_string() })
                    .with(item::Targeted{ range })
                    .with(effects::GainBlock{ amount: block_amount })
                    .with(effects::BuffStrength{ amount: buff_amount });
            }
            Attacks::AttackAndGiveCard{name, range, amount, card, number } => {
                attack = attack.with(Name{ name: name.to_string() })
                    .with(item::Targeted{ range })
                    .with(effects::DealDamage{ amount })
                    .with(effects::GainCard{ card, number, to_hand: false })
            }
        }
    
        attack.build()
    }
}