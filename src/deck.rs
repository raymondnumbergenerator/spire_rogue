use specs::prelude::*;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub const MAX_HAND_SIZE: usize = 10;

pub struct Deck {
    pub hand: Vec<Entity>,
    pub draw: Vec<Entity>,
    pub discard: Vec<Entity>,
}

impl Deck {
    pub fn gain_card(&mut self, c: Entity) {
        self.discard.push(c);
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

    pub fn discard(&mut self, card: Entity) {
        let mut i = 0;
        for c in self.hand.iter() {
            if *c == card {
                break;
            }
            i += 1;
        }
        self.discard.push(self.hand.remove(i));
    }

    pub fn redraw(&mut self) {
        for i in 0 .. self.hand.len() {
            self.discard.push(self.hand[i])
        }
        self.hand.clear();

        for _ in 0 .. 5 {
            self.draw();
        }
    }

    pub fn draw(&mut self) {
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