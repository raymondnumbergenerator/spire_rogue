use specs::prelude::*;
use super::super::{Name, Monster, gamelog::GameLog, RunState, CombatStats, status};

pub struct EndTurnSystem {}

impl<'a> System<'a> for EndTurnSystem {
    type SystemData = (
        ReadExpect<'a, RunState>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, status::Weak>,
        WriteStorage<'a, status::Vulnerable>,
    );

    fn run(&mut self, data : Self::SystemData) {
        let (runstate, mut log, entities, player_ent, names, monsters, mut stats, mut status_weak, mut status_vulnerable) = data;

        // Skip if not on endturn
        let turn: bool;
        match *runstate {
            RunState::EndTurn{player_turn} => { turn = player_turn; }
            _ => { return; }
        }

        // Decrement weakness turn counter
        {
            let mut affected_targets = Vec::new();
            for (ent, mut weak) in (&entities, &mut status_weak).join() {
                if turn {
                    if ent == *player_ent { weak.turns -= 1; }
                } else {
                    if let Some(_) = monsters.get(ent) {
                        weak.turns -= 1;
                    }
                }
                if weak.turns < 1 {
                    affected_targets.push(ent);
                }
            }
            for to_remove in affected_targets {
                if let Some(ent_name) = names.get(to_remove) {
                    log.push(format!("Weak wears off for {}.", ent_name.name.to_string()));
                }
                status_weak.remove(to_remove);
            }
        }

        // Decrement vulnerable turn counter
        {
            let mut affected_targets = Vec::new();
            for (ent, mut vulnerable) in (&entities, &mut status_vulnerable).join() {
                if turn {
                    if ent == *player_ent { vulnerable.turns -= 1; }
                } else {
                    if let Some(_) = monsters.get(ent) {
                        vulnerable.turns -= 1;
                    }
                }
                if vulnerable.turns < 1 {
                    affected_targets.push(ent);
                }
            }
            for to_remove in affected_targets {
                if let Some(ent_name) = names.get(to_remove) {
                    log.push(format!("Vulnerable wears off for {}.", ent_name.name.to_string()));
                }
                status_vulnerable.remove(to_remove);
            }
        }

        // Decay all block
        for (ent, mut stat) in (&entities, &mut stats).join() {
            if turn {
                if !(ent == *player_ent) { stat.block = 0; }
            } else {
                if ent == *player_ent { stat.block = 0; }
            }
        }

    }
}