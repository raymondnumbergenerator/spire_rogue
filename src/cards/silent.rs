use specs::prelude::*;
use specs::saveload::{SimpleMarker, MarkedBuilder};
use rltk::{RGB, RandomNumberGenerator};

use super::super::{
    Name, Position, Renderable, saveload,
    item, effects, status
};

fn build_card<S: ToString>(ecs: &mut World, name: S, energy_cost: i32) -> EntityBuilder {
    ecs.create_entity()
        .with(Name{ name: name.to_string() })
        .with(item::Item{})
        .with(item::Card{ energy_cost })
        .marked::<SimpleMarker<saveload::SerializeMe>>()
}

fn strike(ecs: &mut World) -> Entity {
    build_card(ecs, "Strike", 1)
        .with(item::Targeted{ range: 2 })
        .with(effects::DealDamage{ amount: 6 })
        .build()
}

fn defend(ecs: &mut World) -> Entity {
    build_card(ecs, "Defend", 1)
        .with(effects::GainBlock{ amount: 5 })
        .build()
}

fn neutralize(ecs: &mut World) -> Entity {
    build_card(ecs, "Neutralize", 0)
        .with(item::Targeted{ range: 2 })
        .with(effects::DealDamage{ amount: 3 })
        .with(status::Weak{ turns: 1 })
        .build()
}

fn survivor(ecs: &mut World) -> Entity {
    build_card(ecs, "Survivor", 1)
        .with(effects::GainBlock{ amount: 8 })
        .with(effects::DiscardCard{ number: 1 })
        .build()
}

pub fn shiv(ecs: &mut World) -> Entity {
    build_card(ecs, "Shiv", 0)
        .with(item::Targeted{ range: 2 })
        .with(effects::DealDamage{ amount: 4 })
        .with(item::Ethereal{})
        .build()
}

fn acrobatics(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Acrobatics", 1)
        .with(effects::DrawCard{ number: 3 })
        .with(effects::DiscardCard{ number: 1 })
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('='),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .build()
}

fn backflip(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Backflip", 1)
        .with(effects::GainBlock{ amount: 5 })
        .with(effects::DrawCard{ number: 2 })
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('='),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .build()
}

fn blade_dance(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Blade Dance", 1)
        .with(effects::GainCard{
            card: effects::GainableCard::Shiv,
            number: 2,
            to_hand: true,
        })
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('='),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .build()
}

fn cloak_and_dagger(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Cloak And Dagger", 1)
        .with(effects::GainBlock{ amount: 6 })
        .with(effects::GainCard{
            card: effects::GainableCard::Shiv,
            number: 1,
            to_hand: true,
        })
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('='),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .build()
}

fn deadly_poison(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Deadly Poison", 1)
        .with(item::Targeted{ range: 3 })
        .with(status::Poison{ turns: 5 })
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('='),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .build()
}

fn poisoned_stab(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Poisoned Stab", 1)
        .with(item::Targeted{ range: 2 })
        .with(effects::DealDamage{ amount: 6 })
        .with(status::Poison{ turns: 3 })
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('='),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .build()
}

fn quick_slash(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Quick Slash", 1)
        .with(item::Targeted{ range: 2 })
        .with(effects::DealDamage{ amount: 8 })
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

fn slice(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Slice", 0)
        .with(item::Targeted{ range: 2 })
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

pub fn random_card(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 8);
    }

    match roll {
        1 => { acrobatics(ecs, x, y); }
        2 => { backflip(ecs, x, y); }
        3 => { blade_dance(ecs, x, y); }
        4 => { cloak_and_dagger(ecs, x, y); }
        5 => { deadly_poison(ecs, x, y); }
        6 => { poisoned_stab(ecs, x, y); }
        7 => { quick_slash(ecs, x, y); }
        _ => { slice(ecs, x, y); }
    }
}

pub fn starter(ecs: &mut World) -> Vec<Entity> {
    let mut init_deck = Vec::new();
    for _ in 0 .. 5 { init_deck.push(strike(ecs)); }
    for _ in 0 .. 5 { init_deck.push(defend(ecs)); }
    init_deck.push(neutralize(ecs));
    init_deck.push(survivor(ecs));

    init_deck
}