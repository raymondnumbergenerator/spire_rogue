use specs::prelude::*;
use rltk::{RGB, RandomNumberGenerator};

use super::super::{
    Name, Position, Renderable,
    Item, Card, Targeted, AreaOfEffect,
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

fn clothesline(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Clothesline".to_string() })
        .with(Item{})
        .with(Card{ energy_cost: 2 })
        .with(effects::DealDamage{ amount: 12 })
        .with(status::Weak{ turns: 2 })
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

fn cleave(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Cleave".to_string() })
        .with(Item{})
        .with(Card{ energy_cost: 1 })
        .with(effects::DealDamage{ amount: 8 })
        .with(AreaOfEffect{ radius: 2 })
        .with(Targeted{ range: 0 })
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('='),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .build()
}

fn pommel_strike(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Pommel Strike".to_string() })
        .with(Item{})
        .with(Card{ energy_cost: 1 })
        .with(effects::DealDamage{ amount: 8 })
        .with(Targeted{ range: 2 })
        .with(effects::DrawCard{ number: 1 })
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('='),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .build()
}

pub fn random_card(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 3);
    }

    match roll {
        1 => { clothesline(ecs, x, y); }
        2 => { cleave(ecs, x, y); }
        _ => { pommel_strike(ecs, x, y); }
    }
}

pub fn starter(ecs: &mut World) -> Vec<Entity> {
    let mut init_deck = Vec::new();
    for _ in 0 .. 5 { init_deck.push(strike(ecs)); }
    for _ in 0 .. 4 { init_deck.push(defend(ecs)); }
    init_deck.push(bash(ecs));

    init_deck
}