use specs::prelude::*;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub const MAX_HAND_SIZE: usize = 10;

#[derive(Clone)]
pub struct Deck {
    pub hand: Vec<Entity>,
    pub draw: Vec<Entity>,
    pub discard: Vec<Entity>,
}

impl Deck {
    pub fn gain_card(&mut self, c: Entity) {
        self.discard.push(c);
    }

    pub fn gain_to_hand(&mut self, c: Entity) {
        self.draw.push(c);
        self.draw_card();
    }

    pub fn gain_multiple_cards(&mut self, cards: Vec<Entity>) {
        for c in cards {
            self.gain_card(c)
        }
    }

    pub fn reshuffle(&mut self) {
        self.draw = self.discard.clone();
        self.draw.shuffle(&mut thread_rng());
        self.discard.clear();
    }

    pub fn discard_card(&mut self, card: Entity, destroy: bool) {
        let mut i = 0;
        for c in self.hand.iter() {
            if *c == card {
                break;
            }
            i += 1;
        }
        let c = self.hand.remove(i);
        if !destroy {
            self.discard.push(c);
        }
    }

    pub fn draw_card(&mut self) {
        if self.hand.len() < MAX_HAND_SIZE {
            if self.draw.len() == 0 {
                self.reshuffle();
            }
            if let Some(c) = self.draw.pop() {
                self.hand.push(c);
            }
        }
    }
}