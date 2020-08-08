use specs::prelude::*;
use super::super::{intent, Name, creature, status, Gamelog};

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Gamelog>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, intent::MeleeTarget>,
        ReadStorage<'a, creature::CombatStats>,
        WriteStorage<'a, creature::SufferDamage>,
        WriteStorage<'a, status::Weak>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut log, names, mut intent_melee, combat_stats, mut suffer_damage, mut statusweak) = data;

        for (ent, intent_melee, name, stats) in (&entities, &intent_melee, &names, &combat_stats).join() {
            if stats.hp > 0 {
                let target_stats = combat_stats.get(intent_melee.target).unwrap();
                if target_stats.hp > 0 {
                    let target_name = names.get(intent_melee.target).unwrap();
                    let mut damage = i32::max(0, stats.strength);

                    // Check for status: weak
                    if let Some(_) = statusweak.get_mut(ent) {
                        damage = (damage as f32 * 0.75) as i32;
                    }

                    log.push(format!("{} hits {} for {} damage.", &name.name, &target_name.name, damage));
                    creature::SufferDamage::new_damage(&mut suffer_damage, intent_melee.target, damage);
                }
            }
        }

        intent_melee.clear();
    }
}