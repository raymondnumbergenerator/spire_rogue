use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct DealDamage {
    pub amount: i32
}

#[derive(Component, Debug)]
pub struct GainBlock {
    pub amount: i32
}

#[derive(Component,Debug)]
pub struct DiscardCard {
    pub number: i32
}