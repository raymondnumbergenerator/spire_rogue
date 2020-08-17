use specs::prelude::*;
use super::super::{
    Name, Position, creature, Gamelog,
    item, deck, Map,
    effects, status
};

macro_rules! apply_status {
    ($status_storage:expr, $status_effect:ident,
        $entity:expr, $log:expr, $names:expr, $targets:expr, $intent:expr) => {
        let mut affected_targets = Vec::new();
        if let Some(action) = $status_storage.get($intent.action) {
            for target in $targets.iter() {
                affected_targets.push((*target, action.turns));
                $log.push(format!("{} applies {} to {} for {} turns.",
                    $names.get($entity).unwrap().name,
                    stringify!($status_effect),
                    $names.get(*target).unwrap().name,
                    action.turns))
            }
        }
        for target in affected_targets.iter() {
            if let Some(already_affected) = $status_storage.get_mut(target.0) {
                already_affected.turns += target.1;
            } else {
                $status_storage.insert(target.0, status::$status_effect{ turns: target.1 }).expect("Unable to insert status");
            }
        }
    }
}

macro_rules! apply_buff {
    ($buff_type:ty, $buff_stat:ident, $ecs:expr, $entity:expr, $log:expr, $names:expr, $combat_stats:expr, $intent:expr) => {
        let buff = $ecs.read_storage::<$buff_type>();
        if let Some(action) = buff.get($intent.action) {
            if let Some(stats) = $combat_stats.get_mut($entity) {
                stats.$buff_stat += action.amount;
                $log.push(format!("{} uses {} and gains {} {}.",
                    $names.get($entity).unwrap().name,
                    $names.get($intent.action).unwrap().name,
                    action.amount,
                    stringify!($buff_stat)))
            }
        }
    }
}

pub fn run(ecs: &mut World) {
    let mut gain_to_hand_queue: Vec<effects::GainableCard> = Vec::new();
    let mut gain_to_discard_queue: Vec<effects::GainableCard> = Vec::new();

    {
        let entities = ecs.entities();
        let player_entity = ecs.fetch::<Entity>();
        let mut log = ecs.fetch_mut::<Gamelog>();
        let map = ecs.fetch::<Map>();
        let mut deck = ecs.fetch_mut::<deck::Deck>();

        let names = ecs.read_storage::<Name>();

        let mut combat_stats = ecs.write_storage::<creature::CombatStats>();
        let mut intent_action = ecs.write_storage::<creature::PerformAction>();

        let mut status_weak = ecs.write_storage::<status::Weak>();
        let mut status_vulnerable = ecs.write_storage::<status::Vulnerable>();
        let mut status_frail = ecs.write_storage::<status::Frail>();
        let mut status_poison = ecs.write_storage::<status::Poison>();

        for (entity, intent) in (&entities, &intent_action).join() {
            // Determine affected targets
            let mut targets: Vec<Entity> = Vec::new();
            match intent.target {
                None => { targets.push(entity); }
                Some(target) => {
                    let creatures = ecs.read_storage::<creature::Creature>();
                    let aoe = ecs.read_storage::<item::AreaOfEffect>();
                    if let Some(area_effect) = aoe.get(intent.action) {
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
                    } else {
                        let idx = map.xy_idx(target.x, target.y);
                        for creature in map.tile_content[idx].iter() {
                            if let Some(_) = creatures.get(*creature) {
                                if *creature != entity { targets.push(*creature); }
                            }
                        }
                    }

                    // Move caster to targeted location
                    let effect_teleport = ecs.read_storage::<effects::Teleport>();
                    if let Some(_) = effect_teleport.get(intent.action) {
                        let mut positions = ecs.write_storage::<Position>();
                        let mut viewsheds = ecs.write_storage::<creature::Viewshed>();
                        let mut ent_pos = positions.get_mut(entity).unwrap();
                        let dest_idx = map.xy_idx(target.x, target.y);

                        if !map.blocked[dest_idx] {
                            ent_pos.x = target.x;
                            ent_pos.y = target.y;
                            if let Some(viewshed) = viewsheds.get_mut(entity) { viewshed.dirty = true; }
                        }
                        if entity == *player_entity {
                            let mut player_pos = ecs.write_resource::<rltk::Point>();
                            player_pos.x = ent_pos.x;
                            player_pos.y = ent_pos.y;
                        }
                    }
                }
            }

            // Apply block gain to caster
            {
                let effect_block = ecs.read_storage::<effects::GainBlock>();
                if let Some(action) = effect_block.get(intent.action) {
                    let mut amount = 0;
                    if let Some(stats) = combat_stats.get_mut(entity) {
                        amount = i32::max(0, action.amount + stats.dexterity);

                        // Check for status::Frail
                        if let Some(_) = status_frail.get(entity) {
                            amount = (amount as f32 * 0.75) as i32;
                        }
                        stats.block += amount;
                    }
                    if entity == *player_entity {
                        log.push(format!("{} uses {} and gain {} block.",
                            names.get(entity).unwrap().name,
                            names.get(intent.action).unwrap().name,
                            amount))
                    }
                }
            }

            // Deal damage to affected targets
            {
                let effect_damage = ecs.read_storage::<effects::DealDamage>();
                let mut suffer_damage = ecs.write_storage::<creature::SufferDamage>();
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
            }

            // Apply stat buffs to caster
            apply_buff!(effects::BuffStrength, strength, ecs, entity, log, names, combat_stats, intent);
            apply_buff!(effects::BuffDexterity, dexterity, ecs, entity, log, names, combat_stats, intent);

            // Apply status effects to affected targets
            apply_status!(status_weak, Weak, entity, log, names, targets, intent);
            apply_status!(status_vulnerable, Vulnerable, entity, log, names, targets, intent);
            apply_status!(status_frail, Frail, entity, log, names, targets, intent);
            apply_status!(status_poison, Poison, entity, log, names, targets, intent);

            // Draw cards
            {
                let effect_draw = ecs.read_storage::<effects::DrawCard>();
                if let Some(action) = effect_draw.get(intent.action) {
                    for _ in 0 .. action.number {
                        deck.draw_card();
                    }
                };
            }

            // Gain cards
            {
                let effect_gain = ecs.read_storage::<effects::GainCard>();
                if let Some(action) = effect_gain.get(intent.action) {
                    for _ in 0 .. action.number {
                        match action.to_hand {
                            true => { gain_to_hand_queue.push(action.card); }
                            false => { gain_to_discard_queue.push(action.card); }
                        }
                    }
                    log.push(format!("You gain {} {}.",
                        action.number,
                        action.card.to_name()));
                }   
            }

            // Discard used card or remove used potion
            {
                let mut player = ecs.write_storage::<creature::Player>();
                let potions = ecs.read_storage::<item::Potion>();
                let cards = ecs.read_storage::<item::Card>();
                if let Some(_) = cards.get(intent.action) {
                    if let Some(player_energy) = player.get_mut(*player_entity) {
                        player_energy.energy -= cards.get(intent.action).unwrap().energy_cost;
                    }
        
                    let mut destroy = false;
                    let card_ethereal = ecs.read_storage::<item::Ethereal>();
                    let card_fragile = ecs.read_storage::<item::Fragile>();
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
        }
        intent_action.clear();
    }

    // Resolve effects that gain cards
    let mut gain_to_hand: Vec<Entity> = Vec::new();
    let mut gain_to_discard: Vec<Entity> = Vec::new();
    for card in gain_to_hand_queue.iter() {
        gain_to_hand.push(card.to_card(ecs));
    }
    for card in gain_to_discard_queue.iter() {
        gain_to_discard.push(card.to_card(ecs));
    }

    let mut deck = ecs.fetch_mut::<deck::Deck>();
    for card in gain_to_hand.iter() {
        deck.gain_to_hand(*card);
    }
    for card in gain_to_discard.iter() {
        deck.gain_card(*card);
    }
}