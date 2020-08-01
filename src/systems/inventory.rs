use specs::prelude::*;
use super::super::{
    Position, Name, InBackpack, gamelog::GameLog,
    intent, deck::Deck, Potion,
};

pub struct InventorySystem {}

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