use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct Weak {
    pub turns: i32
}

#[derive(Component, Debug)]
pub struct Vulnerable {
    pub turns: i32
}