use specs::prelude::*;

use super::super::{
    Name, Card, Ranged,
    GainBlock, DealDamage, StatusWeak
};

fn strike(ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Strike".to_string() })
        .with(Card{ energy_cost: 1 })
        .with(DealDamage{ amount: 6 })
        .with(Ranged{ range: 2 })
        .build()
}

fn defend(ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Defend".to_string() })
        .with(Card{ energy_cost: 1 })
        .with(GainBlock{ amount: 5 })
        .build()
}

fn neutralize(ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Neutralize".to_string() })
        .with(Card{ energy_cost: 0 })
        .with(DealDamage{ amount: 3 })
        .with(StatusWeak{ turns: 1 })
        .with(Ranged{ range: 2 })
        .build()
}

pub fn starter(ecs: &mut World) -> Vec<Entity> {
    let mut init_deck = Vec::new();
    for _ in 0..5 { init_deck.push(strike(ecs)); }
    for _ in 0..5 { init_deck.push(defend(ecs)); }
    init_deck.push(neutralize(ecs));

    init_deck
}