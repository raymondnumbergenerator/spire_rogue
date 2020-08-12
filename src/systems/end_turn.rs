use specs::prelude::*;
use super::super::{Name, Gamelog, RunState, creature, status};

pub struct EndTurnSystem {}

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

        // Decrement weakness turn counter
        {
            let mut to_remove = Vec::new();
            for (ent, mut weak) in (&entities, &mut status_weak).join() {
                if player_turn {
                    if ent == *player_entity { weak.turns -= 1; }
                } else {
                    if let Some(_) = monsters.get(ent) {
                        weak.turns -= 1;
                    }
                }
                if weak.turns < 1 {
                    to_remove.push(ent);
                }
            }
            for ent in to_remove {
                if let Some(ent_name) = names.get(ent) {
                    log.push(format!("Weak wears off for {}.", ent_name.name.to_string()));
                }
                status_weak.remove(ent);
            }
        }

        // Decrement vulnerable turn counter
        {
            let mut to_remove = Vec::new();
            for (ent, mut vulnerable) in (&entities, &mut status_vulnerable).join() {
                if player_turn {
                    if ent == *player_entity { vulnerable.turns -= 1; }
                } else {
                    if let Some(_) = monsters.get(ent) {
                        vulnerable.turns -= 1;
                    }
                }
                if vulnerable.turns < 1 {
                    to_remove.push(ent);
                }
            }
            for ent in to_remove {
                if let Some(ent_name) = names.get(ent) {
                    log.push(format!("Vulnerable wears off for {}.", ent_name.name.to_string()));
                }
                status_vulnerable.remove(ent);
            }
        }

        // Decrement frail turn counter
        {
            let mut to_remove = Vec::new();
            for (ent, mut frail) in (&entities, &mut status_frail).join() {
                if player_turn {
                    if ent == *player_entity { frail.turns -= 1; }
                } else {
                    if let Some(_) = monsters.get(ent) {
                        frail.turns -= 1;
                    }
                }
                if frail.turns < 1 {
                    to_remove.push(ent);
                }
            }
            for ent in to_remove {
                if let Some(ent_name) = names.get(ent) {
                    log.push(format!("Frail wears off for {}.", ent_name.name.to_string()));
                }
                status_frail.remove(ent);
            }
        }

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