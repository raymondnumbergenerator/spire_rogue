use specs::prelude::*;
use super::super::{
    Name, creature, gamelog::GameLog,
    intent, item, deck, Map,
    effects, status
};

pub struct ActionSystem {}

impl<'a> System<'a> for ActionSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        ReadExpect<'a, Map>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, creature::Player>,
        ReadStorage<'a, creature::Monster>,
        WriteStorage<'a, creature::CombatStats>,
        WriteStorage<'a, creature::SufferDamage>,
        WriteExpect<'a, deck::Deck>,
        ReadStorage<'a, item::Potion>,
        ReadStorage<'a, item::Ethereal>,
        ReadStorage<'a, item::SelfTargeted>,
        ReadStorage<'a, item::AreaOfEffect>,
        ReadStorage<'a, item::Card>,
        WriteStorage<'a, intent::PerformAction>,
        ReadStorage<'a, effects::GainBlock>,
        ReadStorage<'a, effects::DealDamage>,
        ReadStorage<'a, effects::DrawCard>,
        ReadStorage<'a, effects::GainCard>,
        WriteExpect<'a, effects::GainCardQueue>,
        WriteStorage<'a, status::Weak>,
        WriteStorage<'a, status::Vulnerable>,
        WriteStorage<'a, status::Poison>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, player_entity, mut log, map, names,
            mut player, monsters, mut combat_stats, mut suffer_damage,
            mut deck, potions, ethereal, self_targeted, aoe, cards, mut intent_action,
            effect_gain_block, effect_deal_damage, effect_draw, gain_card, mut gain_card_queue,
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
                                for mob in map.tile_content[idx].iter() {
                                    if let Some(_) = monsters.get(*mob) { targets.push(*mob); }
                                }
                            }
                            Some(area_effect) => {
                                let mut aoe_tiles = rltk::field_of_view(target, area_effect.radius, &*map);
                                aoe_tiles.retain(|p| p.x > 0 && p.x < map.width-1 && p.y > 0 && p.y < map.height-1 );
                                for tile_idx in aoe_tiles.iter() {
                                    let idx = map.xy_idx(tile_idx.x, tile_idx.y);
                                    for mob in map.tile_content[idx].iter() {
                                        if let Some(_) = monsters.get(*mob) { targets.push(*mob); }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Apply gain block to caster
            if let Some(action) = effect_gain_block.get(intent.action) {
                let mut amount = 0;
                if let Some(stats) = combat_stats.get_mut(entity) {
                    amount = i32::max(0, action.amount + stats.dexterity);
                    stats.block += amount;
                }
                if entity == *player_entity {
                    log.push(format!("You use {} and gain {} block.", names.get(intent.action).unwrap().name, amount))
                }
            }

            // Deal damage to affected targets
            if let Some(action) = effect_deal_damage.get(intent.action) {
                for target in targets.iter() {
                    let stats = combat_stats.get(entity).unwrap();
                    let mut dmg = i32::max(0, action.amount + stats.strength);

                    // Check for status: vulnerable
                    if let Some(_) = status_vulnerable.get_mut(*target) {
                        dmg = (dmg as f32 * 1.5) as i32;
                    }

                    creature::SufferDamage::new_damage(&mut suffer_damage, *target, dmg);
                    if entity == *player_entity {
                        log.push(format!("You use {} on {} for {} damage.",
                            names.get(intent.action).unwrap().name,
                            names.get(*target).unwrap().name,
                            dmg))
                    }
                }
            }

            // Apply weak to affected targets
            {
                let mut affected_targets = Vec::new();
                if let Some(action) = status_weak.get(intent.action) {
                    for target in targets.iter() {
                        affected_targets.push((*target, action.turns));
                        if entity == *player_entity {
                            log.push(format!("You apply weak to {} for {} turns.",
                                names.get(*target).unwrap().name,
                                action.turns))
                        }
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
                        if entity == *player_entity {
                            log.push(format!("You apply vulnerable to {} for {} turns.",
                                names.get(*target).unwrap().name,
                                action.turns))
                        }
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
                            log.push(format!("You apply poison to {} for {} turns.",
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
            {
                if let Some(action) = effect_draw.get(intent.action) {
                    for _ in 0 .. action.number {
                        deck.draw_card();
                    }
                }
            }

            // Add cards to the to_gain card queue
            {
                if let Some(action) = gain_card.get(intent.action) {
                    for _ in 0 .. action.number {
                        match action.to_hand {
                            true => { gain_card_queue.to_hand.push(action.card); }
                            false => { gain_card_queue.to_discard.push(action.card); }
                        }
                    }
                }
            }
            
            // Discard used card or remove used potion
            if let Some(_) = cards.get(intent.action) {
                if let Some(player_energy) = player.get_mut(*player_entity) {
                    player_energy.energy -= cards.get(intent.action).unwrap().energy_cost;
                }
                if let Some(_) = ethereal.get(intent.action) {
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