use specs::prelude::*;
use super::super::{
    Position, Name, Gamelog,
    item, intent, deck::Deck,
};

pub struct InventorySystem {}

impl<'a> System<'a> for InventorySystem {
    type SystemData = (
        WriteExpect<'a, Deck>,
        WriteExpect<'a, Gamelog>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, intent::PickupItem>,
        ReadStorage<'a, item::Potion>,
        WriteStorage<'a, item::InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut deck, mut log, names, mut positions, mut intent_pickup, potions, mut backpack) = data;

        for intent in intent_pickup.join() {
            positions.remove(intent.item);
            // Gain potions
            if let Some(_) = potions.get(intent.item) {
                backpack.insert(intent.item, item::InBackpack{ owner: intent.collected_by }).expect("Unable to pickup item");
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