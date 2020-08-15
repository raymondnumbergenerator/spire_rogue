use specs::prelude::*;
use rltk::RandomNumberGenerator;

use super::mobs;

#[derive(Copy, Clone)]
pub enum Encounters {
    Cultist,
    JawWorm,
    Louses,
    SmallSlimes
}

impl Encounters {
    pub fn spawn(self, ecs: &mut World) -> Vec<Entity> {
        let mut spawned: Vec<Entity> = Vec::new();
        match self {
            Encounters::Cultist => {
                spawned.push(mobs::cultist(ecs, 0, 0));
            }
            Encounters::JawWorm => {
                spawned.push(mobs::jaw_worm(ecs, 0, 0));
            }
            Encounters::Louses => {
                for _ in 0 .. 2 {
                    let roll = { ecs.write_resource::<RandomNumberGenerator>().range(0, 2) };
                    match roll {
                        0 => spawned.push(mobs::red_louse(ecs, 0, 0)),
                        _ => spawned.push(mobs::green_louse(ecs, 0, 0)),
                    }
                }
            }
            Encounters::SmallSlimes => {
                let roll = { ecs.write_resource::<RandomNumberGenerator>().range(0, 2) };
                match roll {
                    0 => spawned.push(mobs::acid_slime_m(ecs, 0, 0)),
                    _ => spawned.push(mobs::spike_slime_m(ecs, 0, 0)),
                }
                let roll = { ecs.write_resource::<RandomNumberGenerator>().range(0, 2) };
                match roll {
                    0 => spawned.push(mobs::acid_slime_s(ecs, 0, 0)),
                    _ => spawned.push(mobs::spike_slime_s(ecs, 0, 0)),
                }
            }
        }

        return spawned
    }
}