use specs::prelude::*;
use rltk::{RGB};

use super::super::{
    Name, Position, Renderable,
    Item, Card, Targeted,
    effects, status
};

fn strike(ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Strike".to_string() })
        .with(Item{})
        .with(Card{ energy_cost: 1 })
        .with(effects::DealDamage{ amount: 6 })
        .with(Targeted{ range: 2 })
        .build()
}

fn defend(ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Defend".to_string() })
        .with(Item{})
        .with(Card{ energy_cost: 1 })
        .with(effects::GainBlock{ amount: 5 })
        .build()
}

fn bash(ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Bash".to_string() })
        .with(Item{})
        .with(Card{ energy_cost: 2 })
        .with(effects::DealDamage{ amount: 8 })
        .with(status::Vulnerable{ turns: 2 })
        .with(Targeted{ range: 2 })
        .build()
}

pub fn starter(ecs: &mut World) -> Vec<Entity> {
    let mut init_deck = Vec::new();
    for _ in 0..5 { init_deck.push(strike(ecs)); }
    for _ in 0..4 { init_deck.push(defend(ecs)); }
    init_deck.push(bash(ecs));

    init_deck
}