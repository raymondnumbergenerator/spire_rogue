use specs::prelude::*;
use super::{
    Position, Player, State, Map, Viewshed, RunState,
    item, intent,
    deck::Deck
};

use rltk::{Rltk, VirtualKeyCode, Point};
use std::cmp::{max, min};

pub fn move_player(delta_x: i32, delta_y: i32, ecs: &mut World) -> RunState {
    let mut positions = ecs.write_storage::<Position>();
    let mut ppos = ecs.write_resource::<Point>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        // Move to the tile if it is not blocked and end player turn
        if !map.blocked[destination_idx] {
            pos.x = min(map.width - 1, max(0, pos.x + delta_x));
            pos.y = min(map.height - 1, max(0, pos.y + delta_y));
            ppos.x = pos.x;
            ppos.y = pos.y;

            viewshed.dirty = true;

            return RunState::EndTurn{ player_turn: true };
        }
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
    let mut to_delete: Vec<Entity> = Vec::new();

    // Discard cards
    {
        let ethereal = ecs.read_storage::<item::Ethereal>();
        let mut deck = ecs.write_resource::<Deck>();
    
        let mut to_discard: Vec<Entity> = Vec::new();
        for card in deck.hand.iter() {
            if let Some(_) = ethereal.get(*card) {
                to_delete.push(*card);
            } else {
                to_discard.push(*card);
            }
        }
    
        for card in to_discard.iter() {
            deck.discard.push(*card);
        }
        deck.hand.clear();
    }

    // Delete ethereal cards
    {
        for card in to_delete.iter() {
            ecs.delete_entity(*card).expect("Unable to delete ethereal card");
        }
    }

    // Draw hand
    let mut deck = ecs.write_resource::<Deck>();
    for _ in 0 .. 5 {
        deck.draw_card();
    }
}

fn restore_energy(ecs: &mut World) {
    let player_entity = ecs.fetch::<Entity>();
    let mut player = ecs.write_storage::<Player>();

    if let Some(player_energy) = player.get_mut(*player_entity) {
        player_energy.energy = player_energy.max_energy;
    }
}

fn end_turn(ecs: &mut World) {
    redraw_hand(ecs);
    restore_energy(ecs);
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        None => { return RunState::AwaitingInput }
        Some(key) => match key {
            VirtualKeyCode::A => return move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => return move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::W => return move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => return move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Q => return move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::E => return move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::Z => return move_player(-1, 1, &mut gs.ecs),
            VirtualKeyCode::C => return move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::P => return RunState::ShowInventory,
            VirtualKeyCode::Tab => return RunState::ShowHand,
            VirtualKeyCode::Space => end_turn(&mut gs.ecs),
            VirtualKeyCode::G => get_item(&mut gs.ecs),
            VirtualKeyCode::Escape => return RunState::SaveGame,
            _ => { return RunState::AwaitingInput }
        }
    }

    RunState::EndTurn{ player_turn: true }
}