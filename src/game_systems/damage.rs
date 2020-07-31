use specs::prelude::*;
use super::super::{gamelog::GameLog, CombatStats, SufferDamage, Player, Name};

use rltk::{console};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
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

pub fn delete_dead(ecs: &mut World) {
    let mut dead : Vec<Entity> = Vec::new();
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let names = ecs.read_storage::<Name>();
        let players = ecs.read_storage::<Player>();
        let entities = ecs.entities();
        let mut log = ecs.write_resource::<GameLog>();

        for (entity, stats) in (&entities, &combat_stats).join() {
            if stats.hp < 1 {
                if let Some(_) = players.get(entity) {
                    console::log("You are dead");
                } else {
                    if let Some(victim_name) = names.get(entity) {
                        log.push(format!("{} is dead!", victim_name.name));
                    }
                    dead.push(entity);
                }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}