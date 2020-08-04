use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::{Component, ConvertSaveload};
use serde::{Serialize, Deserialize};

#[derive(Component, Debug, ConvertSaveload)]
pub struct PerformAction {
    pub action: Entity,
    pub target: Option<rltk::Point>,
}

#[derive(Component, Debug, ConvertSaveload)]
pub struct PickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct MeleeTarget {
    pub target: Entity
}