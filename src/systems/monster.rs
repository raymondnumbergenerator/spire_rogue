use specs::prelude::*;
use super::super::{Name, Gamelog, Map, Position, creature, item, RunState, status};

use rltk::{Point};

pub struct MonsterSystem {}

impl<'a> System<'a> for MonsterSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, RunState>,
        WriteExpect<'a, Gamelog>,
        WriteExpect<'a, Map>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, item::Targeted>,
        WriteStorage<'a, creature::Viewshed>,
        ReadStorage<'a, creature::Monster>,
        WriteStorage<'a, creature::CombatStats>,
        WriteStorage<'a, creature::AttackCycle>,
        WriteStorage<'a, creature::Intent>,
        WriteStorage<'a, creature::PerformAction>,
        WriteStorage<'a, status::Poison>
    );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, player_pos, runstate, mut log, mut map, names, mut positions, targeted,
            mut viewshed, monster, mut combat_stats, mut attack_cycles, mut monster_intents,
            mut intent_action, mut status_poison) = data;
        
        // Skip if not on monsterturn
        if *runstate != RunState::MonsterTurn { return; }

        // Process poison and derement poison counter
        {
            let mut to_remove = Vec::new();
            for (ent, mut poison, mut stats) in (&entities, &mut status_poison, &mut combat_stats).join() {
                stats.hp -= poison.turns;
                if let Some(ent_name) = names.get(ent) {
                    log.push(format!("{} takes {} damage from poison.", ent_name.name.to_string(), poison.turns));
                }
                poison.turns -= 1;
                if poison.turns < 1 {
                    to_remove.push(ent);
                }
            }
            for ent in to_remove {
                if let Some(ent_name) = names.get(ent) {
                    log.push(format!("Poison wears off for {}.", ent_name.name.to_string()));
                }
                status_poison.remove(ent);
            }
        }

        for (ent, mut viewshed, mut pos, mut atk, mut intent, _) in (&entities, &mut viewshed, &mut positions, &mut attack_cycles, &mut monster_intents, &monster).join() {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            let range = match targeted.get(intent.intent) {
                Some(r) => { r.range }
                None => { 1 }
            };
            let adjusted_range = range as f32 + { if range <= 1 { 0.5 } else { 0.0 } };

            if distance < adjusted_range as f32 {
                // Perform action if player is in range
                intent_action.insert(ent, creature::PerformAction{ action: intent.intent, target: Some(Point::new(player_pos.x, player_pos.y)) })
                    .expect("Unable to insert intent::PerformAction for monsters");
                atk.cycle = (atk.cycle + 1) % atk.attacks.len();
                intent.used = true;
            } else if viewshed.visible_tiles.contains(&*player_pos) {
                // Move towards the player
                let path = rltk::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map
                );
                if path.success && path.steps.len() > 1 {
                    let mut idx = map.xy_idx(pos.x, pos.y);
                    map.blocked[idx] = false;
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    idx = map.xy_idx(pos.x, pos.y);
                    map.blocked[idx] = true;
                    viewshed.dirty = true;
                }
            }
        }
    }
}