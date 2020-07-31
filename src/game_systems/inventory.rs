use specs::prelude::*;
use super::super::{
    Position, Name, Player, CombatStats, SufferDamage,
    WantsToPickupItem, WantsToUseItem, InBackpack,
    gamelog::GameLog, Map,
    deck::Deck, Card, Potion, GainBlock, DealDamage, AreaOfEffect, StatusWeak
};

pub struct InventorySystem {}
pub struct ItemUseSystem {}

impl<'a> System<'a> for InventorySystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToPickupItem>,
        WriteStorage<'a, InBackpack>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut log, names, mut positions, mut wants_pickup, mut backpack) = data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.item);
            backpack.insert(pickup.item, InBackpack{ owner: pickup.collected_by }).expect("Unable to pickup item");
            if pickup.collected_by == *player_entity {
                log.push(format!("You pick up the {}.", names.get(pickup.item).unwrap().name));
            }
        }

        wants_pickup.clear();
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
        WriteStorage<'a, WantsToUseItem>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, AreaOfEffect>,
        ReadStorage<'a, GainBlock>,
        ReadStorage<'a, DealDamage>,
        WriteStorage<'a, StatusWeak>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut log, entities, mut player, map, names, 
            potions, mut deck, card, mut wants_use, mut combat_stats, mut suffer_damage,
            aoe, gainblock, dealdamage, mut statusweak) = data;

        for (entity, using) in (&entities, &wants_use).join() {
            // Determine affected targets
            let mut targets: Vec<Entity> = Vec::new();
            match using.target {
                None => { targets.push(*player_entity); }
                Some(target) => {
                    let area_effect = aoe.get(using.item);
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
            if let Some(item) = gainblock.get(using.item) {
                for target in targets.iter() {
                    if let Some(stats) = combat_stats.get_mut(*target) {
                        stats.block = stats.block + item.amount;
                    }
                    if entity == *player_entity {
                        log.push(format!("You use {} and gain {} block.", names.get(using.item).unwrap().name, item.amount))
                    }
                }
            }

            // Deal damage to affected targets
            if let Some(item) = dealdamage.get(using.item) {
                for target in targets.iter() {
                    SufferDamage::new_damage(&mut suffer_damage, *target, item.amount);
                    if entity == *player_entity {
                        log.push(format!("You use {} on {} for {} damage.",
                            names.get(using.item).unwrap().name,
                            names.get(*target).unwrap().name,
                            item.amount))
                    }
                }
            }

            // Apply weak to affected targets
            let mut apply_weak = Vec::new();
            if let Some(item) = statusweak.get(using.item) {
                for target in targets.iter() {
                    apply_weak.push((*target, item.turns));
                    if entity == *player_entity {
                        log.push(format!("You apply weak to {} for {} turns.",
                            names.get(*target).unwrap().name,
                            item.turns))
                    }
                }
            }
            for target in apply_weak.iter() {
                statusweak.insert(target.0, StatusWeak{ turns: target.1 }).expect("Unable to insert status");
            }
            
            if let Some(_) = potions.get(using.item) {
                entities.delete(using.item).expect("Failed to delete entity");
            } else {
                if let Some(player_energy) = player.get_mut(*player_entity) {
                    player_energy.energy -= card.get(using.item).unwrap().energy_cost;
                }
                deck.discard(using.item);
            }
        }

        wants_use.clear();
    }
}