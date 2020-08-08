use specs::prelude::*;
use super::{
    Position, map, Map, RunState,
    creature, item, intent,
    deck::Deck
};

use rltk::{Rltk, VirtualKeyCode, Point};
use std::cmp::{max, min};

pub fn move_player(delta_x: i32, delta_y: i32, ecs: &mut World) -> RunState {
    let mut positions = ecs.write_storage::<Position>();
    let mut player_pos = ecs.write_resource::<Point>();
    let mut players = ecs.write_storage::<creature::Player>();
    let mut viewsheds = ecs.write_storage::<creature::Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        // Move to the tile if it is not blocked and end player turn
        if !map.blocked[destination_idx] {
            pos.x = min(map.width - 1, max(0, pos.x + delta_x));
            pos.y = min(map.height - 1, max(0, pos.y + delta_y));
            player_pos.x = pos.x;
            player_pos.y = pos.y;

            viewshed.dirty = true;

            return RunState::EndTurn{ player_end_turn: true };
        }
    }

    RunState::AwaitingInput
}

fn go_next_level(ecs: &mut World) -> RunState {
    let player_pos = ecs.fetch::<Point>();
    let map = ecs.fetch::<Map>();
    let player_idx = map.xy_idx(player_pos.x, player_pos.y);

    if map.tiles[player_idx] == map::TileType::DownStairs {
        return RunState::NextLevel;
    }
    RunState::AwaitingInput
}

fn get_item(ecs: &mut World) {
    let player_pos = ecs.fetch::<Point>();
    let player_entity = ecs.fetch::<Entity>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<item::Item>();
    let positions = ecs.read_storage::<Position>();

    // Check if there are any items under the player
    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, position) in (&entities, &items, &positions).join() {
        if player_pos.x == position.x && player_pos.y == position.y {
            target_item = Some(item_entity);
        }
    }

    // Pickup the item under the player
    if let Some(target_item) = target_item {
        let mut intent = ecs.write_storage::<intent::PickupItem>();
        intent.insert(*player_entity, intent::PickupItem{ collected_by: *player_entity, item: target_item }).expect("Unable to insert intent::PickupItem");
    }
}

fn redraw_hand(ecs: &mut World) {
    let ethereal = ecs.read_storage::<item::Ethereal>();
    let hand = { ecs.fetch::<Deck>().hand.clone() };
    let mut deck = ecs.write_resource::<Deck>();
    for card in hand.iter() {
        if let Some(_) = ethereal.get(*card) {
            deck.discard_card(*card, true);
        } else {
            deck.discard_card(*card, false);
        }
    }

    // Draw hand
    for _ in 0 .. 5 {
        deck.draw_card();
    }
}

fn restore_energy(ecs: &mut World) {
    let player_entity = ecs.fetch::<Entity>();
    let mut player = ecs.write_storage::<creature::Player>();

    if let Some(player_energy) = player.get_mut(*player_entity) {
        player_energy.energy = player_energy.max_energy;
    }
}

fn end_turn(ecs: &mut World) {
    redraw_hand(ecs);
    restore_energy(ecs);
}

pub fn player_input(ecs: &mut World, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::AwaitingInput }
        Some(key) => match key {
            VirtualKeyCode::A => return move_player(-1, 0, ecs),
            VirtualKeyCode::D => return move_player(1, 0, ecs),
            VirtualKeyCode::W => return move_player(0, -1, ecs),
            VirtualKeyCode::S => return move_player(0, 1, ecs),
            VirtualKeyCode::Q => return move_player(-1, -1, ecs),
            VirtualKeyCode::E => return move_player(1, -1, ecs),
            VirtualKeyCode::Z => return move_player(-1, 1, ecs),
            VirtualKeyCode::C => return move_player(1, 1, ecs),
            VirtualKeyCode::Period => return go_next_level(ecs),
            VirtualKeyCode::P => return RunState::ShowInventory,
            VirtualKeyCode::Key1 => return RunState::ShowHand{ selection: 0 },
            VirtualKeyCode::Key2 => return RunState::ShowHand{ selection: 1 },
            VirtualKeyCode::Key3 => return RunState::ShowHand{ selection: 2 },
            VirtualKeyCode::Key4 => return RunState::ShowHand{ selection: 3 },
            VirtualKeyCode::Key5 => return RunState::ShowHand{ selection: 4 },
            VirtualKeyCode::Key6 => return RunState::ShowHand{ selection: 5 },
            VirtualKeyCode::Key7 => return RunState::ShowHand{ selection: 6 },
            VirtualKeyCode::Key8 => return RunState::ShowHand{ selection: 7 },
            VirtualKeyCode::Key9 => return RunState::ShowHand{ selection: 8 },
            VirtualKeyCode::Key0 => return RunState::ShowHand{ selection: 9 },
            VirtualKeyCode::Space => end_turn(ecs),
            VirtualKeyCode::G => get_item(ecs),
            VirtualKeyCode::Escape => return RunState::SaveGame,
            _ => { return RunState::AwaitingInput }
        }
    }

    RunState::EndTurn{ player_end_turn: true }
}