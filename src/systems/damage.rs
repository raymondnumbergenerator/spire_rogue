use specs::prelude::*;
use super::super::{gamelog::GameLog, creature, Name};

pub struct DamageSystem {}
pub struct DeadCleanupSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, creature::CombatStats>,
        WriteStorage<'a, creature::SufferDamage>,
    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            let mut total_damage = damage.amount.iter().sum::<i32>();

            // Try to damage block
            if stats.block > 0 {
                let block_damage = i32::min(stats.block, total_damage);
                stats.block -= block_damage;
                total_damage -= block_damage;
            }

            // Try to damage hp
            stats.hp -= total_damage;
        }

        damage.clear();
    }
}

impl<'a>System<'a> for DeadCleanupSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, creature::CombatStats>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player_entity, mut log, names, combat_stats) = data;

        let mut dead: Vec<Entity> = Vec::new();
        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                if entity == *player_entity {
                    ::std::process::exit(0);
                } else {
                    if let Some(victim_name) = names.get(entity) {
                        log.push(format!("{} is dead!", victim_name.name));
                    }
                    dead.push(entity);
                }
            }
        }

        for victim in dead {
            entities.delete(victim).expect("Unable to delete");
        }
    }
}