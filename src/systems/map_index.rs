use specs::prelude::*;
use super::super::{Map, Position, creature::BlocksTile};

pub struct MapIndexSystem {}

impl<'a> System<'a> for MapIndexSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, position, blockers, entities) = data;

        map.populate_blocked();
        map.clear_content_index();
        for (ent, position) in (&entities, &position).join() {
            let idx = map.xy_idx(position.x, position.y);

            // If entity is a Blocker, update the blocked list
            let _p: Option<&BlocksTile> = blockers.get(ent);
            if let Some(_p) = _p {
                map.blocked[idx] = true;
            }

            map.tile_content[idx].push(ent);
        }
    }
}