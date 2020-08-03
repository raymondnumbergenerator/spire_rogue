use specs::prelude::*;
use specs::saveload::{SimpleMarker, MarkedBuilder};
use rltk::{RGB};

use super::super::{
    Name, Position, Renderable, saveload,
    Item, Card, Targeted,
    effects, status
};

fn card_builder<S: ToString>(ecs: &mut World, name: S, energy_cost: i32) -> EntityBuilder {
    ecs.create_entity()
        .with(Name{ name: name.to_string() })
        .with(Item{})
        .with(Card{ energy_cost })
        .marked::<SimpleMarker<saveload::SerializeMe>>()
}

fn strike(ecs: &mut World) -> Entity {
    card_builder(ecs, "Strike", 1)
        .with(Targeted{ range: 2 })
        .with(effects::DealDamage{ amount: 6 })
        .build()
}

fn defend(ecs: &mut World) -> Entity {
    card_builder(ecs, "Defend", 1)
        .with(effects::GainBlock{ amount: 5 })
        .build()
}

fn neutralize(ecs: &mut World) -> Entity {
    card_builder(ecs, "Neutralize", 0)
        .with(Targeted{ range: 2 })
        .with(effects::DealDamage{ amount: 3 })
        .with(status::Weak{ turns: 1 })
        .build()
}

pub fn slice(ecs: &mut World, x: i32, y: i32) -> Entity {
    card_builder(ecs, "Slice", 0)
        .with(Targeted{ range: 2 })
        .with(effects::DealDamage{ amount: 5 })
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
    for _ in 0 .. 5 { init_deck.push(strike(ecs)); }
    for _ in 0 .. 5 { init_deck.push(defend(ecs)); }
    init_deck.push(neutralize(ecs));

    init_deck
}