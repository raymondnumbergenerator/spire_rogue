use specs::prelude::*;

use super::super::{
    Position, item, effects
};

use super::card::{build_card, Rarity};

pub fn shiv(ecs: &mut World) -> Entity {
    build_card(ecs, "Shiv", 0, Rarity::Common)
        .with(item::Targeted{ range: 1 })
        .with(effects::DealDamage{ amount: 4 })
        .with(item::Ethereal{})
        .build()
}

pub fn slimed(ecs: &mut World) -> Entity {
    build_card(ecs, "Slimed", 1, Rarity::Common)
        .with(item::Fragile{})
        .build()
}

fn finesse(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Finesse", 0, Rarity::Uncommon)
        .with(effects::GainBlock{ amount: 2 })
        .with(effects::DrawCard{ number: 1 })
        .with(Position{ x, y })
        .build()
}

fn flash_of_steel(ecs: &mut World, x: i32, y: i32) -> Entity {
    build_card(ecs, "Flash of Steel", 0, Rarity::Uncommon)
        .with(item::Targeted{ range: 1 })
        .with(effects::DealDamage{ amount: 3 })
        .with(effects::DrawCard{ number: 1 })
        .with(Position{ x, y })
        .build()
}