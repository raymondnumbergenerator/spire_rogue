use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator, SerializeComponents, DeserializeComponents, MarkedBuilder, Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::{Component, ConvertSaveload};
use serde::{Serialize, Deserialize};

use std::fs::File;
use std::path::Path;

use super::{
    util::entityvec::EntityVec,
    Map, deck::Deck, Position, Renderable, Viewshed, BlocksTile, Name, CombatStats,
    Targeted, AreaOfEffect, Item, Card, Potion, InBackpack, Player, Monster, SufferDamage,
    effects, intent, status};

pub struct SerializeMe;

#[derive(Component, ConvertSaveload, Clone)]
pub struct SerializableDeck {
    pub hand: EntityVec<Entity>,
    pub draw: EntityVec<Entity>,
    pub discard: EntityVec<Entity>,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct SerializableMap {
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

macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
        $(
        DeserializeComponents::<NoError, _>::deserialize(
            &mut ( &mut $ecs.write_storage::<$type>(), ),
            &mut $data.0,
            &mut $data.1,
            &mut $data.2,
            &mut $de,
        )
        .unwrap();
        )*
    };
}

pub fn save_exists() -> bool {
    Path::new("./save.json").exists()
}

pub fn save_game(ecs: &mut World) {
    let map_copy = ecs.get_mut::<Map>().unwrap().clone();
    let map_helper = ecs.create_entity()
                        .with(SerializableMap{
                            map: map_copy,
                        })
                        .marked::<SimpleMarker<SerializeMe>>()
                        .build();

    let deck = ecs.get_mut::<Deck>().unwrap();
    let deck_copy = SerializableDeck{
        hand: EntityVec::with_existing(deck.hand.clone()),
        draw: EntityVec::with_existing(deck.draw.clone()),
        discard: EntityVec::with_existing(deck.discard.clone()),
    };
    let deck_helper = ecs.create_entity()
                        .with(deck_copy)
                        .marked::<SimpleMarker<SerializeMe>>()
                        .build();

    {
        let data = (ecs.entities(), ecs.read_storage::<SimpleMarker<SerializeMe>>());

        let writer = File::create("./save.json").unwrap();
        let mut serializer = serde_json::Serializer::new(writer);
        serialize_individually!(
            ecs, serializer, data, SerializableMap, SerializableDeck, Position, Renderable, Viewshed,
            BlocksTile, Name, CombatStats, Targeted, AreaOfEffect, Item, Card, Potion, InBackpack, Player,
            Monster, SufferDamage,
            effects::DealDamage, effects::GainBlock, effects::DiscardCard, effects::DrawCard, 
            intent::UseItem, intent::PickupItem, intent::MeleeTarget,
            status::Weak, status::Vulnerable
        );
    }

    ecs.delete_entity(map_helper).expect("Crashed on cleanup");
    ecs.delete_entity(deck_helper).expect("Crashed on cleanup");
}

pub fn load_game(ecs: &mut World) {

}