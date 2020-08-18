use specs::prelude::*;
use rltk::RandomNumberGenerator;

use super::mobs;

#[derive(Copy, Clone)]
pub enum Encounters {
    Cultist(i32),
    JawWorm(i32),
    Louses(i32),
    SlimePair(i32),
    SmallSlimes(i32),
    LargeSlime(i32),
    GremlinGang(i32),
    Slaver(i32),
    FungiBeast(i32),
    Looter(i32),
    ExordiumThugs(i32),
    ExordiumWildlife(i32),
}

impl Encounters {
    pub fn spawn(self, ecs: &mut World) -> Vec<Entity> {
        let mut spawned: Vec<Entity> = Vec::new();
        match self {
            Encounters::Cultist(num) => {
                for _ in 0 .. num {
                    spawned.push(mobs::cultist(ecs, 0, 0));
                }
            }
            Encounters::JawWorm(num) => {
                for _ in 0 .. num {
                    spawned.push(mobs::jaw_worm(ecs, 0, 0));
                }
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
            Encounters::SlimePair(num) => {
                for _ in 0 .. num {
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
            Encounters::SmallSlimes(num) => {
                for _ in 0 .. num {
                    let roll = { ecs.write_resource::<RandomNumberGenerator>().range(0, 2) };
                    match roll {
                        0 => spawned.push(mobs::acid_slime_s(ecs, 0, 0)),
                        _ => spawned.push(mobs::spike_slime_s(ecs, 0, 0)),
                    }
                }
            }
            Encounters::LargeSlime(num) => {
                for _ in 0 .. num {
                    let roll = { ecs.write_resource::<RandomNumberGenerator>().range(0, 2) };
                    match roll {
                        0 => spawned.push(mobs::acid_slime_l(ecs, 0, 0)),
                        _ => spawned.push(mobs::spike_slime_l(ecs, 0, 0)),
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
            Encounters::Slaver(num) => {
                for _ in 0 .. num {
                    let roll = { ecs.write_resource::<RandomNumberGenerator>().range(0, 2) };
                    match roll {
                        0 => spawned.push(mobs::red_slaver(ecs, 0, 0)),
                        _ => spawned.push(mobs::blue_slaver(ecs, 0, 0)),
                    }
                }
            }
            Encounters::FungiBeast(num) => {
                for _ in 0 .. num {
                    spawned.push(mobs::fungi_beast(ecs, 0, 0));
                }
            }
            Encounters::Looter(num) => {
                for _ in 0 .. num {
                    spawned.push(mobs::looter(ecs, 0, 0));
                }
            }
            Encounters::ExordiumThugs(num) => {
                for _ in 0 .. num {
                    let roll = { ecs.write_resource::<RandomNumberGenerator>().range(0, 3) };
                    match roll {
                        0 => spawned.extend(Encounters::Louses(1).spawn(ecs)),
                        1 => spawned.push(mobs::acid_slime_m(ecs, 0, 0)),
                        _ => spawned.push(mobs::spike_slime_m(ecs, 0, 0)),
                    }
                    let roll = { ecs.write_resource::<RandomNumberGenerator>().range(0, 3) };
                    match roll {
                        0 => spawned.push(mobs::looter(ecs, 0, 0)),
                        1 => spawned.push(mobs::cultist(ecs, 0, 0)),
                        _ => spawned.extend(Encounters::Slaver(1).spawn(ecs)),
                    }
                }
            }
            Encounters::ExordiumWildlife(num) => {
                for _ in 0 .. num {
                    let roll = { ecs.write_resource::<RandomNumberGenerator>().range(0, 2) };
                    match roll {
                        0 => spawned.push(mobs::fungi_beast(ecs, 0, 0)),
                        _ => spawned.push(mobs::jaw_worm(ecs, 0, 0))
                    }
                    let roll = { ecs.write_resource::<RandomNumberGenerator>().range(0, 3) };
                    match roll {
                        0 => spawned.extend(Encounters::Louses(1).spawn(ecs)),
                        1 => spawned.push(mobs::acid_slime_m(ecs, 0, 0)),
                        _ => spawned.push(mobs::spike_slime_m(ecs, 0, 0)),
                    }
                }
            }
        }

        return spawned
    }
}