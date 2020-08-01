use specs::prelude::*;
use specs_derive::Component;
use rltk::{RGB};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles : Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String
}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub def: i32,
    pub atk: i32,
    pub block: i32,
}

#[derive(Component, Debug)]
pub struct Targeted {
    pub range: i32
}

#[derive(Component,  Debug)]
pub struct AreaOfEffect {
    pub radius: i32
}

#[derive(Component, Debug)]
pub struct Item {}

#[derive(Component, Debug)]
pub struct Card {
    pub energy_cost: i32
}

#[derive(Component, Debug)]
pub struct Potion {}

#[derive(Component, Debug)]
pub struct GainBlock {
    pub amount: i32
}

#[derive(Component, Debug)]
pub struct DealDamage {
    pub amount: i32
}

// #[derive(Component, Debug)]
// pub struct StatusWeak {
//     pub turns: i32
// }

// #[derive(Component, Debug)]
// pub struct StatusVulnerable {
//     pub turns: i32
// }

#[derive(Component,Debug)]
pub struct DiscardCard {
    pub number: i32
}

#[derive(Component, Debug, Clone)]
pub struct InBackpack {
    pub owner: Entity
}

#[derive(Component, Debug)]
pub struct Player {
    pub max_energy: i32,
    pub energy: i32,
}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage { amount: vec![amount] };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}