use specs::prelude::*;
use rltk::{RGB};

use super::super::{
    Name, Position, Renderable,
    Item, Card, Targeted,
    GainBlock, DealDamage, StatusWeak
};

fn strike(ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Strike".to_string() })
        .with(Item{})
        .with(Card{ energy_cost: 1 })
        .with(DealDamage{ amount: 6 })
        .with(Targeted{ range: 2 })
        .build()
}

fn defend(ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Defend".to_string() })
        .with(Item{})
        .with(Card{ energy_cost: 1 })
        .with(GainBlock{ amount: 5 })
        .build()
}

fn neutralize(ecs: &mut World) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Neutralize".to_string() })
        .with(Item{})
        .with(Card{ energy_cost: 0 })
        .with(DealDamage{ amount: 3 })
        .with(StatusWeak{ turns: 1 })
        .with(Targeted{ range: 2 })
        .build()
}

pub fn slice(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Slice".to_string() })
        .with(Item{})
        .with(Card{ energy_cost: 0 })
        .with(DealDamage{ amount: 5 })
        .with(Targeted{ range: 2 })
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('='),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .build()
}

pub fn starter(ecs: &mut World) -> Vec<Entity> {
    let mut init_deck = Vec::new();
    for _ in 0..5 { init_deck.push(strike(ecs)); }
    for _ in 0..5 { init_deck.push(defend(ecs)); }
    init_deck.push(neutralize(ecs));

    init_deck
}