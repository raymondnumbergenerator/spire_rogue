use specs::prelude::*;
use rltk::RandomNumberGenerator;

use super::super::{
    Position, item, effects, status
};

use super::card::{build_card, Rarity};

fn strike(ecs: &mut World) -> Entity {
    build_card(ecs, "Strike", 1, Rarity::Common)
        .with(item::Targeted{ range: 1 })
        .with(effects::DealDamage{ amount: 6 })
        .build()
}

fn defend(ecs: &mut World) -> Entity {
    build_card(ecs, "Defend", 1, Rarity::Common)
        .with(effects::GainBlock{ amount: 5 })
        .build()
}

fn neutralize(ecs: &mut World) -> Entity {
    build_card(ecs, "Neutralize", 0, Rarity::Common)
        .with(item::Targeted{ range: 1 })
        .with(effects::DealDamage{ amount: 3 })
        .with(status::Weak{ turns: 1 })
        .build()
}

fn survivor(ecs: &mut World) -> Entity {
    build_card(ecs, "Survivor", 1, Rarity::Common)
        .with(effects::GainBlock{ amount: 8 })
        .with(effects::DiscardCard{ number: 1 })
        .build()
}

fn acrobatics(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Acrobatics", 1, Rarity::Common)
        .with(item::Targeted{ range: 1 })
        .with(effects::Teleport{})
        .with(effects::DrawCard{ number: 3 })
        .with(effects::DiscardCard{ number: 1 })
        .with(Position{ x, y })
        .build()
}

fn backflip(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Backflip", 1, Rarity::Common)
        .with(item::Targeted{ range: 1 })
        .with(effects::Teleport{})
        .with(effects::GainBlock{ amount: 5 })
        .with(effects::DrawCard{ number: 2 })
        .with(Position{ x, y })
        .build()
}

fn blade_dance(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Blade Dance", 1, Rarity::Common)
        .with(effects::GainCard{
            card: effects::GainableCard::Shiv,
            number: 2,
            to_hand: true,
        })
        .with(Position{ x, y })
        .build()
}

fn cloak_and_dagger(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Cloak And Dagger", 1, Rarity::Common)
        .with(effects::GainBlock{ amount: 6 })
        .with(effects::GainCard{
            card: effects::GainableCard::Shiv,
            number: 1,
            to_hand: true,
        })
        .with(Position{ x, y })
        .build()
}

fn dagger_throw(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Dagger Throw", 1, Rarity::Common)
        .with(item::Targeted{ range: 2 })
        .with(effects::DealDamage{ amount: 5 })
        .with(effects::DrawCard{ number: 1 })
        .with(effects::DiscardCard{ number: 1 })
        .with(Position{ x, y })
        .build()
}

fn deadly_poison(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Deadly Poison", 1, Rarity::Common)
        .with(item::Targeted{ range: 2 })
        .with(status::Poison{ turns: 5 })
        .with(Position{ x, y })
        .build()
}

fn deflect(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Deflect", 0, Rarity::Common)
        .with(effects::GainBlock{ amount: 4 })
        .with(Position{ x, y })
        .build()
}

fn poisoned_stab(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Poisoned Stab", 1, Rarity::Common)
        .with(item::Targeted{ range: 1 })
        .with(effects::DealDamage{ amount: 6 })
        .with(status::Poison{ turns: 3 })
        .with(Position{ x, y })
        .build()
}

fn quick_slash(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Quick Slash", 1, Rarity::Common)
        .with(item::Targeted{ range: 1 })
        .with(effects::DealDamage{ amount: 8 })
        .with(effects::DrawCard{ number: 1 })
        .with(Position{ x, y })
        .build()
}

fn slice(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Slice", 0, Rarity::Common)
        .with(item::Targeted{ range: 1 })
        .with(effects::DealDamage{ amount: 5 })
        .with(Position{ x, y })
        .build()
}

fn dash(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Dash", 2, Rarity::Uncommon)
        .with(item::Targeted{ range: 1 })
        .with(effects::DealDamage{ amount: 10 })
        .with(effects::GainBlock{ amount: 10 })
        .with(Position{ x, y })
        .build()
}

fn footwork(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Footwork", 1, Rarity::Uncommon)
        .with(effects::BuffDexterity{ amount: 4 })
        .with(Position{ x, y })
        .build()
}

fn leg_sweep(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Leg Sweep", 2, Rarity::Uncommon)
        .with(item::Targeted{ range: 1 })
        .with(effects::GainBlock{ amount: 10 })
        .with(status::Weak{ turns: 2 })
        .with(Position{ x, y })
        .build()
}

pub fn random_card(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 12);
    }

    match roll {
        1 => { acrobatics(ecs, x, y); }
        2 => { backflip(ecs, x, y); }
        3 => { blade_dance(ecs, x, y); }
        4 => { cloak_and_dagger(ecs, x, y); }
        5 => { deadly_poison(ecs, x, y); }
        6 => { poisoned_stab(ecs, x, y); }
        7 => { quick_slash(ecs, x, y); }
        8 => { slice(ecs, x, y); }
        9 => { dash(ecs, x, y); }
        10 => { footwork(ecs, x, y); }
        11 => { deflect(ecs, x, y); }
        12 => { dagger_throw(ecs, x, y); }
        _ => { leg_sweep(ecs, x, y); }
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