use specs::prelude::*;
use super::super::{
    Name, creature, Gamelog,
    item, deck, Map,
    effects, status
};

pub struct ActionSystem {}

impl<'a> System<'a> for ActionSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Gamelog>,
        ReadExpect<'a, Map>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, creature::Player>,
        ReadStorage<'a, creature::Creature>,
        WriteStorage<'a, creature::CombatStats>,
        WriteStorage<'a, creature::SufferDamage>,
        WriteStorage<'a, creature::PerformAction>,
        WriteExpect<'a, deck::Deck>,
        ReadStorage<'a, item::Potion>,
        ReadStorage<'a, item::Card>,
        ReadStorage<'a, item::Ethereal>,
        ReadStorage<'a, item::Fragile>,
        ReadStorage<'a, item::SelfTargeted>,
        ReadStorage<'a, item::AreaOfEffect>,
        ReadStorage<'a, effects::GainBlock>,
        ReadStorage<'a, effects::DealDamage>,
        ReadStorage<'a, effects::DrawCard>,
        ReadStorage<'a, effects::GainCard>,
        WriteExpect<'a, effects::GainCardQueue>,
        ReadStorage<'a, effects::BuffStrength>,
        WriteStorage<'a, status::Weak>,
        WriteStorage<'a, status::Vulnerable>,
        WriteStorage<'a, status::Poison>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player_entity, mut log, map, names,
            mut player, creatures, mut combat_stats, mut suffer_damage,
            mut intent_action, mut deck, potions, cards,
            card_ethereal, card_fragile, self_targeted, aoe,
            effect_block, effect_damage, effect_draw, gain_card, mut gain_card_queue,
            effect_strength,
            mut status_weak, mut status_vulnerable, mut status_poison) = data;

        for (entity, intent) in (&entities, &intent_action).join() {
            // Determine affected targets
            let mut targets: Vec<Entity> = Vec::new();
            if let Some(_) = self_targeted.get(intent.action) {
                targets.push(entity)
            } else {
                match intent.target {
                    None => { targets.push(entity); }
                    Some(target) => {
                        let area_effect = aoe.get(intent.action);
                        match area_effect {
                            None => {
                                let idx = map.xy_idx(target.x, target.y);
                                for creature in map.tile_content[idx].iter() {
                                    if let Some(_) = creatures.get(*creature) {
                                        if *creature != entity { targets.push(*creature); }
                                    }
                                }
                            }
                            Some(area_effect) => {
                                let mut aoe_tiles = rltk::field_of_view(target, area_effect.radius, &*map);
                                aoe_tiles.retain(|p| p.x > 0 && p.x < map.width-1 && p.y > 0 && p.y < map.height-1 );
                                for tile_idx in aoe_tiles.iter() {
                                    let idx = map.xy_idx(tile_idx.x, tile_idx.y);
                                    for creature in map.tile_content[idx].iter() {
                                        if let Some(_) = creatures.get(*creature) {
                                            if *creature != entity { targets.push(*creature); }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Apply gain block to caster
            if let Some(action) = effect_block.get(intent.action) {
                let mut amount = 0;
                if let Some(stats) = combat_stats.get_mut(entity) {
                    amount = i32::max(0, action.amount + stats.dexterity);
                    stats.block += amount;
                }
                if entity == *player_entity {
                    log.push(format!("{} uses {} and gain {} block.",
                        names.get(entity).unwrap().name,
                        names.get(intent.action).unwrap().name,
                        amount))
                }
            }

            // Deal damage to affected targets
            if let Some(action) = effect_damage.get(intent.action) {
                for target in targets.iter() {
                    let stats = combat_stats.get(entity).unwrap();
                    let mut dmg = i32::max(0, action.amount + stats.strength);

                    // Check for status::Weak
                    if let Some(_) = status_weak.get(entity) {
                        dmg = (dmg as f32 * 0.75) as i32;
                    }

                    // Check for status::Vulnerable
                    if let Some(_) = status_vulnerable.get(*target) {
                        dmg = (dmg as f32 * 1.5) as i32;
                    }

                    creature::SufferDamage::new_damage(&mut suffer_damage, *target, dmg);
                    log.push(format!("{} uses {} on {} for {} damage.",
                        names.get(entity).unwrap().name,
                        names.get(intent.action).unwrap().name,
                        names.get(*target).unwrap().name,
                        dmg))
                }
            }

            // Apply strength buff to caster
            if let Some(action) = effect_strength.get(intent.action) {
                if let Some(stats) = combat_stats.get_mut(entity) {
                    stats.strength += action.amount;
                    log.push(format!("{} uses {} and gains {} strength.",
                        names.get(entity).unwrap().name,
                        names.get(intent.action).unwrap().name,
                        action.amount))
                }
            }

            // Apply weak to affected targets
            {
                let mut affected_targets = Vec::new();
                if let Some(action) = status_weak.get(intent.action) {
                    for target in targets.iter() {
                        affected_targets.push((*target, action.turns));
                        log.push(format!("{} applies weak to {} for {} turns.",
                            names.get(entity).unwrap().name,
                            names.get(*target).unwrap().name,
                            action.turns))
                    }
                }
                for target in affected_targets.iter() {
                    if let Some(already_affected) = status_weak.get_mut(target.0) {
                        already_affected.turns += target.1;
                    } else {
                        status_weak.insert(target.0, status::Weak{ turns: target.1 }).expect("Unable to insert status");
                    }
                }
            }

            // Apply vulnerable to affected targets
            {
                let mut affected_targets = Vec::new();
                if let Some(action) = status_vulnerable.get(intent.action) {
                    for target in targets.iter() {
                        affected_targets.push((*target, action.turns));
                        log.push(format!("{} applies vulnerable to {} for {} turns.",
                            names.get(entity).unwrap().name,
                            names.get(*target).unwrap().name,
                            action.turns))
                    }
                }
                for target in affected_targets.iter() {
                    if let Some(already_affected) = status_vulnerable.get_mut(target.0) {
                        already_affected.turns += target.1;
                    } else {
                        status_vulnerable.insert(target.0, status::Vulnerable{ turns: target.1 }).expect("Unable to insert status");
                    }
                }
            }

            // Apply poison to affected targets
            {
                let mut affected_targets = Vec::new();
                if let Some(action) = status_poison.get(intent.action) {
                    for target in targets.iter() {
                        affected_targets.push((*target, action.turns));
                        if entity == *player_entity {
                            log.push(format!("{} applies poison to {} for {} turns.",
                                names.get(entity).unwrap().name,
                                names.get(*target).unwrap().name,
                                action.turns))
                        }
                    }
                }
                for target in affected_targets.iter() {
                    if let Some(already_affected) = status_poison.get_mut(target.0) {
                        already_affected.turns += target.1;
                    } else {
                        status_poison.insert(target.0, status::Poison{ turns: target.1 }).expect("Unable to insert status");
                    }
                }
            }

            // Draw cards
            if let Some(action) = effect_draw.get(intent.action) {
                for _ in 0 .. action.number {
                    deck.draw_card();
                }
            };

            // Add cards to the to_gain card queue
            if let Some(action) = gain_card.get(intent.action) {
                for _ in 0 .. action.number {
                    match action.to_hand {
                        true => { gain_card_queue.to_hand.push(action.card); }
                        false => { gain_card_queue.to_discard.push(action.card); }
                    }
                }
                log.push(format!("You gain {} {}.",
                    action.number,
                    action.card.to_name()));
            };
            
            // Discard used card or remove used potion
            if let Some(_) = cards.get(intent.action) {
                if let Some(player_energy) = player.get_mut(*player_entity) {
                    player_energy.energy -= cards.get(intent.action).unwrap().energy_cost;
                }

                let mut destroy = false;
                if let Some(_) = card_ethereal.get(intent.action) { destroy = true; }
                if let Some(_) = card_fragile.get(intent.action) { destroy = true; }

                if destroy {
                    deck.discard_card(intent.action, true);
                    entities.delete(intent.action).expect("Failed to delete entity");
                } else {
                    deck.discard_card(intent.action, false);
                }
            } else if let Some(_) = potions.get(intent.action) {
                entities.delete(intent.action).expect("Failed to delete entity");
            }
        }

        intent_action.clear();
    }
}