use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::{Component, ConvertSaveload};
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Weak {
    pub turns: i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Vulnerable {
    pub turns: i32
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Poison {
    pub turns: i32
}
