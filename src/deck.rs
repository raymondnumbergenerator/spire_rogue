use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::ConvertSaveload;
use serde::{Serialize, Deserialize};

use rand::thread_rng;
use rand::seq::SliceRandom;

use super::effects;

pub const MAX_HAND_SIZE: usize = 10;

#[derive(Clone, ConvertSaveload)]
pub struct ToGain {
    pub to_hand: Vec<effects::GainableCard>,
    pub to_discard: Vec<effects::GainableCard>,
}

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

    pub fn discard_card(&mut self, card: Entity, ethereal: bool) {
        let mut i = 0;
        for c in self.hand.iter() {
            if *c == card {
                break;
            }
            i += 1;
        }
        let c = self.hand.remove(i);
        if !ethereal {
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