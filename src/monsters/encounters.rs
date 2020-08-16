use specs::prelude::*;
use rltk::RandomNumberGenerator;

use super::mobs;

#[derive(Copy, Clone)]
pub enum Encounters {
    Cultist,
    JawWorm,
    Louses(i32),
    SmallSlimes,
    LotsOfSlimes(i32),
    GremlinGang(i32),
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
            Encounters::Louses(num) => {
                for _ in 0 .. num {
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
            Encounters::LotsOfSlimes(num) => {
                for _ in 0 .. num {
                    let roll = { ecs.write_resource::<RandomNumberGenerator>().range(0, 2) };
                    match roll {
                        0 => spawned.push(mobs::acid_slime_s(ecs, 0, 0)),
                        _ => spawned.push(mobs::spike_slime_s(ecs, 0, 0)),
                    }
                }
            }
            Encounters::GremlinGang(num) => {
                for _ in 0 .. num {
                    let roll = { ecs.write_resource::<RandomNumberGenerator>().range(0, 5) };
                    match roll {
                        0 => {
                            spawned.push(mobs::mad_gremlin(ecs, 0, 0));
                            spawned.push(mobs::mad_gremlin(ecs, 0, 0));
                        }
                        1 => {
                            spawned.push(mobs::sneaky_gremlin(ecs, 0, 0));
                            spawned.push(mobs::sneaky_gremlin(ecs, 0, 0));
                        }
                        2 => {
                            spawned.push(mobs::fat_gremlin(ecs, 0, 0));
                            spawned.push(mobs::fat_gremlin(ecs, 0, 0));
                        }
                        3 => spawned.push(mobs::gremlin_wizard(ecs, 0, 0)),
                        _ => spawned.push(mobs::shield_gremlin(ecs, 0, 0)),
                    }
                }
            }
        }

        return spawned
    }
}