use specs::prelude::*;
use super::super::{CombatStats, WantsToMelee, Name, SufferDamage, StatusWeak, gamelog::GameLog};

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        WriteStorage<'a, StatusWeak>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut log, mut wants_melee, names, combat_stats, mut suffer_damage, mut statusweak) = data;

        for (ent, wants_melee, name, stats) in (&entities, &wants_melee, &names, &combat_stats).join() {
            if stats.hp > 0 {
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                if target_stats.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();
                    let mut damage = i32::max(0, stats.atk);

                    // Check for status: weak
                    if let Some(_) = statusweak.get_mut(ent) {
                        damage = (damage as f32 * 0.75) as i32;
                    }

                    log.push(format!("{} hits {} for {} damage.", &name.name, &target_name.name, damage));
                    SufferDamage::new_damage(&mut suffer_damage, wants_melee.target, damage);
                }
            }
        }

        wants_melee.clear();
    }
}