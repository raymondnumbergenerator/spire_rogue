use specs::prelude::*;
use specs::saveload::{SimpleMarker, MarkedBuilder};
use rltk::RGB;

use super::super::{
    Name, Renderable, saveload,
    effects, item, status
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

pub fn describe_card(ecs: &World, card: Entity) {
    let mut description = Vec::new();

    {
        let targeted = ecs.read_storage::<item::Targeted>();
        let aoe = ecs.read_storage::<item::AreaOfEffect>();
        let fragile = ecs.read_storage::<item::Fragile>();
        let ethereal = ecs.read_storage::<item::Ethereal>();
        if let Some(action) = targeted.get(card) {
            description.push(format!("Range {}.", action.range))
        }
        if let Some(action) = aoe.get(card) {
            description.push(format!("AOE {}.", action.radius))
        }
        if let Some(_) = fragile.get(card) {
            description.push("Fragile.".to_string())
        }
        if let Some(_) = ethereal.get(card) {
            description.push("Ethereal.".to_string())
        }
    }

    {
        let effect_damage = ecs.read_storage::<effects::DealDamage>();
        let effect_block = ecs.read_storage::<effects::GainBlock>();
        if let Some(action) = effect_damage.get(card) {
            description.push(format!("Deal {} damage.", action.amount));
        }
        if let Some(action) = effect_block.get(card) {
            description.push(format!("Gain {} block.", action.amount));
        }
    }

    {
        let buff_strength = ecs.read_storage::<effects::BuffStrength>();
        let buff_dexterity = ecs.read_storage::<effects::BuffDexterity>();
        if let Some(action) = buff_strength.get(card) {
            description.push(format!("Gain {} Strength.", action.amount));
        }
        if let Some(action) = buff_dexterity.get(card) {
            description.push(format!("Gain {} Dexterity.", action.amount));
        }
    }

    {
        let status_weak = ecs.read_storage::<status::Weak>();
        let status_vulnerable = ecs.read_storage::<status::Vulnerable>();
        let status_frail = ecs.read_storage::<status::Frail>();
        let status_poison = ecs.read_storage::<status::Poison>();
        if let Some(action) = status_weak.get(card) {
            description.push(format!("Apply {} Weak.", action.turns));
        }
        if let Some(action) = status_vulnerable.get(card) {
            description.push(format!("Apply {} Vulnerable.", action.turns));
        }
        if let Some(action) = status_frail.get(card) {
            description.push(format!("Apply {} Frail.", action.turns));
        }
        if let Some(action) = status_poison.get(card) {
            description.push(format!("Apply {} Poison.", action.turns));
        }
    }

    {
        let effect_draw = ecs.read_storage::<effects::DrawCard>();
        let effect_discard = ecs.read_storage::<effects::DiscardCard>();
        if let Some(action) = effect_draw.get(card) {
            description.push(format!("Draw {} card(s).", action.number))
        }
        if let Some(action) = effect_discard.get(card) {
            description.push(format!("Discard {} card(s).", action.number))
        }
    }

    {
        let effect_gain = ecs.read_storage::<effects::GainCard>();
        if let Some(action) = effect_gain.get(card) {
            let to_hand = if action.to_hand { "hand" } else { "discard" };
            description.push(format!("Add {} {} to your {}.",
                action.number, action.card.to_name(), to_hand))
        }
    }

    {
        let effect_move = ecs.read_storage::<effects::Teleport>();
        if let Some(_) = effect_move.get(card) {
            description.push("Move to the targeted tile if possible.".to_string());
        }
    }

    for desc in description {
        println!("{}", desc);
    }
}