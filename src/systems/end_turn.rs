use specs::prelude::*;
use super::super::{Name, Gamelog, RunState, creature, status};

pub struct EndTurnSystem {}

macro_rules! decay_status {
    ($status_storage:expr, $status_name:expr, $log:expr, $player_entity:expr, $player_turn:expr,
        $entities:expr, $names:expr, $monsters:expr) => (
        let mut to_remove = Vec::new();
        for (ent, mut status) in (&$entities, &mut $status_storage).join() {
            if $player_turn {
                if ent == *$player_entity { status.turns -= 1; }
            } else {
                if let Some(_) = $monsters.get(ent) {
                    status.turns -= 1;
                }
            }
            if status.turns < 1 {
                to_remove.push(ent);
            }
        }
        for ent in to_remove {
            if let Some(ent_name) = $names.get(ent) {
                $log.push(format!("{} wears off for {}.", $status_name, ent_name.name.to_string()));
            }
            $status_storage.remove(ent);
        }
    )
}

impl<'a> System<'a> for EndTurnSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        WriteExpect<'a, Gamelog>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, creature::Monster>,
        WriteStorage<'a, creature::CombatStats>,
        WriteStorage<'a, status::Weak>,
        WriteStorage<'a, status::Vulnerable>,
        WriteStorage<'a, status::Frail>,
    );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, player_entity, runstate, mut log, names, monsters, mut combat_stats,
            mut status_weak, mut status_vulnerable, mut status_frail) = data;

        // Skip if not on endturn
        let player_turn: bool;
        match *runstate {
            RunState::EndTurn{player_end_turn} => { player_turn = player_end_turn; }
            _ => { return; }
        }

        // Decay status effects
        decay_status!(status_weak, "Weak", log, player_entity, player_turn, entities, names, monsters);
        decay_status!(status_vulnerable, "Vulnerable", log, player_entity, player_turn, entities, names, monsters);
        decay_status!(status_frail, "Frail", log, player_entity, player_turn, entities, names, monsters);

        // Decay stats
        for (ent, mut stats) in (&entities, &mut combat_stats).join() {
            if player_turn {
                if !(ent == *player_entity) {
                    stats.block = 0;
                    stats.strength = (stats.strength as f32 - ((stats.strength - stats.base_strength) as f32 * 0.25)) as i32;
                }
            } else {
                if ent == *player_entity {
                    stats.block = 0;
                    stats.strength = (stats.strength as f32 - ((stats.strength - stats.base_strength) as f32 * 0.25)) as i32;
                }
            }
        }

    }
}