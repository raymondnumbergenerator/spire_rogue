use specs::prelude::*;
use specs::saveload::{SimpleMarker, MarkedBuilder};
use rltk::{RGB, RandomNumberGenerator};

use super::{
    Name, Position, Renderable, saveload,
    creature, effects, cards, item, monsters,
    util::rect::Rect, map::MAPWIDTH,
};

pub fn player(ecs: &mut World, x: i32, y: i32) -> Entity {
    ecs.create_entity()
        .with(Name{ name: "Silent".to_string() })
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('@'),
            fg: RGB::from_f32(0.1, 0.8, 0.1),
            bg: RGB::named(rltk::BLACK),
            render_order: 0,
        })
        .with(creature::Creature{})
        .with(creature::Player{ max_energy: 3, energy: 3 })
        .with(creature::CombatStats{ max_hp: 70, hp: 70, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(creature::Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .marked::<SimpleMarker<saveload::SerializeMe>>()
        .build()
}

fn potion_block(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('v'),
            fg: RGB::named(rltk::CYAN),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Block Potion".to_string() })
        .with(item::Item{})
        .with(item::Potion{})
        .with(effects::GainBlock{ amount: 12 })
        .marked::<SimpleMarker<saveload::SerializeMe>>()
        .build();
}

fn potion_fire(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('*'),
            fg: RGB::named(rltk::ORANGE),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Fire Potion".to_string() })
        .with(item::Item{})
        .with(item::Potion{})
        .with(item::Targeted{ range: 3 })
        .with(effects::DealDamage{ amount: 20 })
        .marked::<SimpleMarker<saveload::SerializeMe>>()
        .build();
}

fn potion_explosive(ecs: &mut World, x: i32, y: i32) {
    ecs.create_entity()
        .with(Position{ x, y })
        .with(Renderable{
            glyph: rltk::to_cp437('*'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        })
        .with(Name{ name: "Explosive Potion".to_string() })
        .with(item::Item{})
        .with(item::Potion{})
        .with(item::Targeted{ range: 5 })
        .with(item::AreaOfEffect{ radius: 1 })
        .with(effects::DealDamage{ amount: 10 })
        .marked::<SimpleMarker<saveload::SerializeMe>>()
        .build();
}

pub fn random_potion(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 3);
    }

    match roll {
        1 => { potion_block(ecs, x, y) }
        2 => { potion_explosive(ecs, x, y) }
        _ => { potion_fire(ecs, x, y) }
    }
}

/// Fills a room with monsters and items
pub fn spawn_room(ecs: &mut World, room: &Rect) {
    let mut monster_spawn_points: Vec<usize> = Vec::new();
    let mut item_spawn_points: Vec<usize> = Vec::new();
    let mut card_spawn_points: Vec<usize> = Vec::new();
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        let num_monsters = rng.range(0, 3);
        let num_items = rng.range(0, 2);
        let num_cards = rng.range(0, 4);

        // Decide monster spawn points
        for _ in 0 .. num_monsters {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !monster_spawn_points.contains(&idx) {
                    monster_spawn_points.push(idx);
                    added = true;
                }
            }
        }

        // Decide item spawn points
        for _ in 0 .. num_items {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !item_spawn_points.contains(&idx) {
                    item_spawn_points.push(idx);
                    added = true;
                }
            }
        }

        // Decide card spawn points
        for _ in 0 .. num_cards {
            let mut added = false;
            while !added {
                let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
                let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
                let idx = (y * MAPWIDTH) + x;
                if !card_spawn_points.contains(&idx) {
                    card_spawn_points.push(idx);
                    added = true;
                }
            }
        }
    }

    // Spawn monsters
    for idx in monster_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        monsters::random_monster(ecs, x as i32, y as i32)
    }

    // Spawn items
    for idx in item_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        random_potion(ecs, x as i32, y as i32)
    }

    // Spawn cards
    for idx in card_spawn_points.iter() {
        let x = *idx % MAPWIDTH;
        let y = *idx / MAPWIDTH;
        cards::silent::random_card(ecs, x as i32, y as i32)
    }

}