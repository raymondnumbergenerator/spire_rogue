use specs::prelude::*;
use super::super::{Viewshed, Monster, Map, Position, intent, RunState};

use rltk::{Point};

pub struct MonsterSystem {}

impl<'a> System<'a> for MonsterSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, intent::MeleeTarget>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, player_pos, player_entity,
            runstate, entities, mut viewshed,
            monster, mut position, mut intent_melee) = data;
        
        // Skip if not on monsterturn
        if *runstate != RunState::MonsterTurn { return; }

        for (ent, mut viewshed, _monster, mut pos) in (&entities, &mut viewshed, &monster, &mut position).join() {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                // Attack the player if in melee range
                intent_melee.insert(ent, intent::MeleeTarget{ target: *player_entity}).expect("Unable to insert attack");
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