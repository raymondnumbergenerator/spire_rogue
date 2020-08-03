use specs::prelude::*;
use specs_derive::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator, SerializeComponents, DeserializeComponents, MarkedBuilder};
use specs::error::NoError;
use serde::{Serialize, Deserialize};

use std::fs::File;

use super::{
    Map, Position, Renderable, Viewshed, BlocksTile, Name, CombatStats, Targeted,
    AreaOfEffect, Item, Card, Potion, InBackpack, Player, Monster, SufferDamage,
    effects, intent, status};

pub struct SerializeMe;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map: Map
}

macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<NoError, SimpleMarker<SerializeMe>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

pub fn save_game(ecs: &mut World) {
    let map_copy = ecs.get_mut::<Map>().unwrap().clone();
    let savehelper = ecs.create_entity()
                        .with(SerializationHelper{ map :map_copy })
                        .marked::<SimpleMarker<SerializeMe>>()
                        .build();

    {
        let data = (ecs.entities(), ecs.read_storage::<SimpleMarker<SerializeMe>>());

        let writer = File::create("./save.json").unwrap();
        let mut serializer = serde_json::Serializer::new(writer);
        serialize_individually!(
            ecs, serializer, data, Position, Renderable, Viewshed, BlocksTile, Name,
            CombatStats, Targeted, AreaOfEffect, Item, Card, Potion, InBackpack, Player,
            Monster, SufferDamage,
            effects::DealDamage, effects::GainBlock, effects::DiscardCard, effects::DrawCard, 
            intent::UseItem, intent::PickupItem, intent::MeleeTarget,
            status::Weak, status::Vulnerable
        );
    }

    ecs.delete_entity(savehelper).expect("Crashed on cleanup");
}