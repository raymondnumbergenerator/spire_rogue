use specs::prelude::*;
use specs::saveload::{SimpleMarker, MarkedBuilder};
use rltk::RGB;

use super::super::{
    Name, Renderable, saveload, item
};

pub enum Rarity {
    Common,
    Uncommon,
    Rare,
}

pub fn build_card<S: ToString>(ecs: &mut World, name: S, energy_cost: i32, rarity: Rarity) -> EntityBuilder {
    let color = match rarity {
        Rarity::Common => RGB::named(rltk::LIGHT_GRAY),
        Rarity::Uncommon => RGB::named(rltk::LIGHT_BLUE),
        Rarity::Rare => RGB::named(rltk::LIGHT_YELLOW),
    };

    ecs.create_entity()
        .with(Name{ name: name.to_string() })
        .with(item::Item{})
        .with(item::Card{ energy_cost })
        .with(Renderable{
            glyph: rltk::to_cp437('='),
            fg: color,
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .marked::<SimpleMarker<saveload::SerializeMe>>()
}