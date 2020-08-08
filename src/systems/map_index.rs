use specs::prelude::*;
use super::super::{Map, Position, creature::BlocksTile};

pub struct MapIndexSystem {}

impl<'a> System<'a> for MapIndexSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
    );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut map, position, blockers) = data;

        map.populate_blocked();
        map.clear_content_index();
        for (ent, position) in (&entities, &position).join() {
            let idx = map.xy_idx(position.x, position.y);

            // If entity is a Blocker, update the blocked list
            if let Some(_) = blockers.get(ent) {
                map.blocked[idx] = true;
            }

            map.tile_content[idx].push(ent);
        }
    }
}