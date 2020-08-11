use specs::prelude::*;
use rltk::{RGB, Rltk, VirtualKeyCode};

use std::char;

use super::{
    Map, Name, Position, Point, Gamelog, creature,
    deck::Deck, util::utils, monsters, item, status,
    map::MAPWIDTH, map::MAPHEIGHT, WINDOWHEIGHT, deck::MAX_HAND_SIZE
};

pub const GUISIZE: usize = 14;
const INVENTORYWIDTH: usize = 25;
const INVENTORYPOS: usize = MAPWIDTH - INVENTORYWIDTH - 1;

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();
    let creatures = ecs.read_storage::<creature::Creature>();
    let monsters = ecs.read_storage::<creature::Monster>();
    let combat_stats = ecs.read_storage::<creature::CombatStats>();
    let status_weak = ecs.read_storage::<status::Weak>();
    let status_vulnerable = ecs.read_storage::<status::Vulnerable>();
    let status_poison = ecs.read_storage::<status::Poison>();

    let attack_cycles = ecs.read_storage::<creature::AttackCycle>();

    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= map.width || mouse_pos.1 >= map.height { return; }

    let mut tooltip: Vec<String> = Vec::new();

    let mut draw_intents = false;

    // Push name to tooltips
    for (position, name) in (&positions, &names).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
            tooltip.push(name.name.to_string());
        }
    }

    // Push combat stats to tooltips
    for (position, _, stats) in (&positions, &creatures, &combat_stats).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
            tooltip.push(format!("{}/{}", stats.hp, stats.max_hp));
            tooltip.push(format!("[{}]", stats.block));
        }
    }

    // Push enemy intent to tooltips
    for (position, _, ac, stat) in (&positions, &monsters, &attack_cycles, &combat_stats).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
            match ac.attacks[ac.cycle] {
                monsters::Attacks::NormalAttack{name: _, amount, range} => {
                    let damage = amount + stat.strength;
                    tooltip.push(format!("{}:A{}", range, damage));
                }
                monsters::Attacks::GainBlock{name: _, amount, range: _} => {
                    let block = amount + stat.dexterity;
                    tooltip.push(format!("{}", block));
                }
                monsters::Attacks::AttackAndBlock{name: _, damage_amount, block_amount, range} => {
                    let damage = damage_amount + stat.strength;
                    let block = block_amount + stat.dexterity;
                    tooltip.push(format!("{}:A{},B{}", range, damage, block));
                }
                monsters::Attacks::ApplyWeak{name: _, turns, range} => {
                    tooltip.push(format!("{}:W{}", range, turns));
                }
                monsters::Attacks::BuffStrength{name: _, amount, range: _} => {
                    tooltip.push(format!("S{}", amount));
                }
                monsters::Attacks::BlockAndBuffStrength{name: _, block_amount, buff_amount, range: _} => {
                    let block = block_amount + stat.dexterity;
                    tooltip.push(format!("B{},S{}", block, buff_amount));
                }
                monsters::Attacks::AttackAndGiveCard{name: _, amount, card: _, number: _, range} => {
                    let damage = amount + stat.strength;
                    tooltip.push(format!("{}:A{},#", range, damage));
                }
            }
            draw_intents = true;
        }
    }

    // Push status effects to tooltips
    for (position, _, weak, vulnerable, poison) in (&positions, &creatures, status_weak.maybe(), status_vulnerable.maybe(), status_poison.maybe()).join() {
        let idx = map.xy_idx(position.x, position.y);
        if position.x == mouse_pos.0 && position.y == mouse_pos.1 && map.visible_tiles[idx] {
            if let Some(w) = weak { tooltip.push(format!("W{}", w.turns)); }
            if let Some(v) = vulnerable { tooltip.push(format!("V{}", v.turns)); }
            if let Some(p) = poison { tooltip.push(format!("P{}", p.turns)); }
        }
    }

    if !tooltip.is_empty() {
        let mut width: i32 = 0;
        for s in tooltip.iter() {
            if width < s.len() as i32 { width = s.len() as i32; }
        }
        width += 1;

        let mut tooltip_iter = tooltip.iter();
        let mut x;
        let mut y;
        if mouse_pos.0 > 40 {
            x = mouse_pos.0 - width;
            y = mouse_pos.1;
        } else {
            x = mouse_pos.0 + 2;
            y = mouse_pos.1;
        }

        // Draw entity name
        let mut s = tooltip_iter.next();
        if let Some(s) = s { ctx.print_color(x, y, RGB::named(rltk::WHITE), RGB::named(rltk::GREY), s); }
        y += 1;

        // Draw entity hp
        s = tooltip_iter.next();
        let mut hp_len = 6;
        if let Some(s) = s {
            ctx.print_color(x, y, RGB::named(rltk::RED), RGB::named(rltk::GREY), s);
            hp_len = s.len() as i32;
        }

        // Draw entity block
        s = tooltip_iter.next();
        if let Some(s) = s { ctx.print_color(x + hp_len, y, RGB::named(rltk::CYAN), RGB::named(rltk::GREY), s); }
        y += 1;

        // Draw enemy intents
        if draw_intents {
            s = tooltip_iter.next();
            if let Some(s) = s { ctx.print_color(x, y, RGB::named(rltk::ORANGE), RGB::named(rltk::GREY), s); }
            y += 1;
        }

        // Draw entity status effects
        for s in tooltip_iter {
            let color;
            match s.chars().next().unwrap() {
                'V' => { color = RGB::named(rltk::RED); },
                'W' => { color = RGB::named(rltk::LIGHTBLUE); },
                'P' => { color = RGB::named(rltk::GREEN); },
                _ => { color = RGB::named(rltk::CYAN); }
            }
            ctx.print_color(x, y, color, RGB::named(rltk::GREY), s);
            x += s.len() as i32;
        }
    }
}

pub fn ranged_target(ecs: &World, ctx: &mut Rltk, range: i32, radius: i32) -> (ItemMenuResult, Option<Point>) {
    let player_entity = ecs.fetch::<Entity>();
    let player_pos = ecs.fetch::<Point>();
    let viewsheds = ecs.read_storage::<creature::Viewshed>();
    let mouse_pos = ctx.mouse_pos();
    let mouse_point = Point::new(mouse_pos.0, mouse_pos.1);

    let adjusted_range = range as f32 + { if range <= 1 { 0.5 } else { 0.0 } };

    // Highlight available target cells
    let mut available_cells = Vec::new();
    if let Some(visible) = viewsheds.get(*player_entity) {
        for idx in visible.visible_tiles.iter() {
            let dist = rltk::DistanceAlg::Pythagoras.distance2d(*player_pos, *idx);
            if dist <= adjusted_range as f32 {
                ctx.set_bg(idx.x, idx.y, RGB::named(rltk::YELLOW));
                available_cells.push(idx);
            }
        }
    } else {
        return (ItemMenuResult::Cancel, None);
    }

    // Highlight radius for aoe attacks
    if available_cells.contains(&&mouse_point) {
        if let Some(visible) = viewsheds.get(*player_entity) {
            for idx in visible.visible_tiles.iter() {
                let dist = rltk::DistanceAlg::Pythagoras.distance2d(mouse_point, *idx);
                if dist <= radius as f32 {
                    ctx.set_bg(idx.x, idx.y, RGB::named(rltk::CYAN));
                }
            }
        }
    }

    // Draw mouse cursor
    let mut valid_target = false;
    for idx in available_cells.iter() { if idx.x == mouse_pos.0 && idx.y == mouse_pos.1 { valid_target = true; } }
    if valid_target {
        ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(rltk::CYAN));
        if ctx.left_click {
            return (ItemMenuResult::Selected, Some(Point::new(mouse_pos.0, mouse_pos.1)));
        }
    } else {
        if ctx.left_click {
            return (ItemMenuResult::Cancel, None);
        }
    }

    (ItemMenuResult::NoResponse, None)
}

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    // Draw gui box
    ctx.draw_box(0, MAPHEIGHT, MAPWIDTH - INVENTORYWIDTH - 1, GUISIZE - 1, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    // Draw player stats
    let combat_stats = ecs.read_storage::<creature::CombatStats>();
    let players = ecs.read_storage::<creature::Player>();
    let mut x = 2;
    for (_, stats) in (&players, &combat_stats).join() {
        let health = format!("HP: {} / {}", stats.hp, stats.max_hp);
        let block = format!("[{}]", stats.block);
        ctx.print_color(x, MAPHEIGHT, RGB::named(rltk::RED), RGB::named(rltk::BLACK), &health);
        x += health.len() + 1;
        ctx.print_color(x, MAPHEIGHT, RGB::named(rltk::CYAN), RGB::named(rltk::BLACK), &block);
        x += block.len() + 1;
    }

    // Draw player status effects
    let status_weak = ecs.read_storage::<status::Weak>();
    let status_vulnerable = ecs.read_storage::<status::Vulnerable>();
    for (_, weak, vulnerable) in (&players, status_weak.maybe(), status_vulnerable.maybe()).join() {
        if let Some(w) = weak {
            let weak_text = format!("W{}", w.turns);
            ctx.print_color(x, MAPHEIGHT, RGB::named(rltk::LIGHTBLUE), RGB::named(rltk::BLACK), &weak_text);
            x += weak_text.len()
        }
        if let Some(v) = vulnerable {
            let vulnerable_text = format!("V{}", v.turns);
            ctx.print_color(x, MAPHEIGHT, RGB::named(rltk::RED), RGB::named(rltk::BLACK), &vulnerable_text);
        }
    }

    // Draw message log
    let log = ecs.fetch::<Gamelog>();
    let mut y = MAPHEIGHT + 1;
    for s in log.entries.iter().rev(){
        if y < WINDOWHEIGHT - 1 { ctx.print(2, y, s); }
        y += 1;
    }

    // Draw tooltips
    draw_tooltips(ecs, ctx);

    // Draw player hand
    draw_hand(ecs, ctx);
}

pub fn draw_hand(ecs: &World, ctx: &mut Rltk)  {
    let deck = ecs.write_resource::<Deck>();
    let cards = ecs.read_storage::<item::Card>();
    let names = ecs.read_storage::<Name>();

    let player_entity = ecs.fetch::<Entity>();
    let players = ecs.read_storage::<creature::Player>();
    let player_energy = players.get(*player_entity).unwrap();

    let mut y = MAPHEIGHT as i32;
    // Draw gui box
    ctx.draw_box(INVENTORYPOS, y, INVENTORYWIDTH, MAX_HAND_SIZE + 3, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    // Draw "hand" label and player energy
    ctx.print_color(INVENTORYPOS + 2, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Hand");
    ctx.print_color(INVENTORYPOS + 8, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), format!("[{} / {}]", player_energy.energy, player_energy.max_energy));

    // Draw cards
    let mut hand: Vec<Entity> = Vec::new();
    let mut i = 1;
    for c in deck.hand.iter() {
        ctx.set(INVENTORYPOS + 2, y + 2, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437('('));
        ctx.set(INVENTORYPOS + 3, y + 2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), rltk::to_cp437(char::from_digit(i % 10, 10).unwrap()));
        ctx.set(INVENTORYPOS + 4, y + 2, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437(')'));

        ctx.print(INVENTORYPOS + 6, y + 2, names.get(*c).unwrap().name.to_string());
        ctx.print(INVENTORYPOS + 23, y + 2, cards.get(*c).unwrap().energy_cost.to_string());
        hand.push(*c);
        y += 1;
        i += 1;
    }
}

pub fn pick_card(ecs: &World, selection: i32) -> (ItemMenuResult, Option<Entity>) {
    let cards = ecs.read_storage::<item::Card>();
    let deck = ecs.write_resource::<Deck>();
    let hand = &deck.hand;

    let player_entity = ecs.fetch::<Entity>();
    let players = ecs.read_storage::<creature::Player>();
    let player_energy = players.get(*player_entity).unwrap();

    if selection > -1 && selection < hand.len() as i32
        && cards.get(hand[selection as usize]).unwrap().energy_cost <= player_energy.energy {
        return (ItemMenuResult::Selected, Some(hand[selection as usize]));
    }
    (ItemMenuResult::Cancel, None)
}

pub fn discard_card(ecs: &World, ctx: &mut Rltk, number: i32) -> (ItemMenuResult, Option<Entity>) {
    ctx.print_color(INVENTORYPOS + 2, WINDOWHEIGHT - 1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), format!("Discard {} cards", number));

    let deck = ecs.write_resource::<Deck>();
    let hand = &deck.hand;

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            let selection = utils::number_to_option(key);
            if selection > -1 && selection < hand.len() as i32 {
                return (ItemMenuResult::Selected, Some(hand[selection as usize]));
            }
            (ItemMenuResult::NoResponse, None)
        }
    }
}

pub fn draw_inventory(ecs: &mut World, ctx: &mut Rltk) -> (ItemMenuResult, Option<Entity>) {
    let player_entity = ecs.fetch::<Entity>();
    let names = ecs.read_storage::<Name>();
    let backpack = ecs.read_storage::<item::InBackpack>();
    let entities = ecs.entities();

    let inventory  = (&backpack, &names).join().filter(|item| item.0.owner == *player_entity);
    let count = inventory.count();

    let mut y = (MAPHEIGHT - count) as i32 - 2;
    ctx.draw_box(INVENTORYPOS, y - 2, INVENTORYWIDTH, (count + 3) as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    ctx.print_color(INVENTORYPOS + 2, y - 2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Potions");
    ctx.print_color(INVENTORYPOS + 2, y + count as i32 + 1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "ESC to close");

    let mut equippable: Vec<Entity> = Vec::new();
    let mut c = 0;
    for (entity, _, name) in (&entities, &backpack, &names).join().filter(|item| item.1.owner == *player_entity ) {
        ctx.set(INVENTORYPOS + 2, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437('('));
        ctx.set(INVENTORYPOS + 3, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97 + c as rltk::FontCharType);
        ctx.set(INVENTORYPOS + 4, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437(')'));

        ctx.print(INVENTORYPOS + 6, y, &name.name.to_string());
        equippable.push(entity);
        y += 1;
        c += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { (ItemMenuResult::Cancel, None) }
                _ => {
                    let selection = rltk::letter_to_option(key);
                    if selection > -1 && selection < count as i32 {
                        return (ItemMenuResult::Selected, Some(equippable[selection as usize]));
                    }
                    (ItemMenuResult::NoResponse, None)
                }
            }
        }
    }
}