use specs::prelude::*;
use super::super::{
    Position, Name, Player, CombatStats, SufferDamage,
    intent, InBackpack, AreaOfEffect,
    gamelog::GameLog, Map, deck::Deck, Card, Potion,
    effects, status
};

pub struct InventorySystem {}
pub struct ItemUseSystem {}

impl<'a> System<'a> for InventorySystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Potion>,
        WriteStorage<'a, intent::PickupItem>,
        WriteStorage<'a, InBackpack>,
        WriteExpect<'a, Deck>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut log, names, mut positions, potions, mut intent_pickup, mut backpack, mut deck) = data;

        for intent in intent_pickup.join() {
            positions.remove(intent.item);
            // Gain potions
            if let Some(_) = potions.get(intent.item) {
                backpack.insert(intent.item, InBackpack{ owner: intent.collected_by }).expect("Unable to pickup item");
                log.push(format!("You pick up the {}.", names.get(intent.item).unwrap().name));
            }
            // Gain cards
            else {
                deck.gain_card(intent.item);
                log.push(format!("You gain {}.", names.get(intent.item).unwrap().name));
            }

        }

        intent_pickup.clear();
    }
}

impl<'a> System<'a> for ItemUseSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, Player>,
        ReadExpect<'a, Map>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Potion>,
        WriteExpect<'a, Deck>,
        ReadStorage<'a, Card>,
        WriteStorage<'a, intent::UseItem>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, AreaOfEffect>,
        ReadStorage<'a, effects::GainBlock>,
        ReadStorage<'a, effects::DealDamage>,
        WriteStorage<'a, status::Weak>,
        WriteStorage<'a, status::Vulnerable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut log, entities, mut player, map, names, 
            potions, mut deck, card, mut intent_use, mut combat_stats, mut suffer_damage, aoe,
            effect_gain_block, effect_deal_damage, mut status_weak, mut status_vulnerable) = data;

        for (entity, intent) in (&entities, &intent_use).join() {
            // Determine affected targets
            let mut targets: Vec<Entity> = Vec::new();
            match intent.target {
                None => { targets.push(*player_entity); }
                Some(target) => {
                    let area_effect = aoe.get(intent.item);
                    match area_effect {
                        None => {
                            let idx = map.xy_idx(target.x, target.y);
                            for mob in map.tile_content[idx].iter() {
                                targets.push(*mob);
                            }
                        }
                        Some(area_effect) => {
                            let mut aoe_tiles = rltk::field_of_view(target, area_effect.radius, &*map);
                            aoe_tiles.retain(|p| p.x > 0 && p.x < map.width-1 && p.y > 0 && p.y < map.height-1 );
                            for tile_idx in aoe_tiles.iter() {
                                let idx = map.xy_idx(tile_idx.x, tile_idx.y);
                                for mob in map.tile_content[idx].iter() {
                                    targets.push(*mob);
                                }
                            }
                        }
                    }
                }
            }

            // Apply gain block to affected targets
            if let Some(item) = effect_gain_block.get(intent.item) {
                for target in targets.iter() {
                    if let Some(stats) = combat_stats.get_mut(*target) {
                        stats.block = stats.block + item.amount;
                    }
                    if entity == *player_entity {
                        log.push(format!("You use {} and gain {} block.", names.get(intent.item).unwrap().name, item.amount))
                    }
                }
            }

            // Deal damage to affected targets
            if let Some(item) = effect_deal_damage.get(intent.item) {
                for target in targets.iter() {
                    let mut dmg = item.amount;

                    // Check for status: vulnerable
                    if let Some(_) = status_vulnerable.get_mut(*target) {
                        dmg = (dmg as f32 * 1.5) as i32;
                    }

                    SufferDamage::new_damage(&mut suffer_damage, *target, dmg);
                    if entity == *player_entity {
                        log.push(format!("You use {} on {} for {} damage.",
                            names.get(intent.item).unwrap().name,
                            names.get(*target).unwrap().name,
                            dmg))
                    }
                }
            }

            // Apply weak to affected targets
            {
                let mut affected_targets = Vec::new();
                if let Some(item) = status_weak.get(intent.item) {
                    for target in targets.iter() {
                        affected_targets.push((*target, item.turns));
                        if entity == *player_entity {
                            log.push(format!("You apply weak to {} for {} turns.",
                                names.get(*target).unwrap().name,
                                item.turns))
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
                if let Some(item) = status_vulnerable.get(intent.item) {
                    for target in targets.iter() {
                        affected_targets.push((*target, item.turns));
                        if entity == *player_entity {
                            log.push(format!("You apply vulnerable to {} for {} turns.",
                                names.get(*target).unwrap().name,
                                item.turns))
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
            
            // Remove used potion or discard used card
            if let Some(_) = potions.get(intent.item) {
                entities.delete(intent.item).expect("Failed to delete entity");
            } else {
                if let Some(player_energy) = player.get_mut(*player_entity) {
                    player_energy.energy -= card.get(intent.item).unwrap().energy_cost;
                }
                deck.discard(intent.item);
            }
        }

        intent_use.clear();
    }
}