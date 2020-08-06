use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator, SerializeComponents, DeserializeComponents, MarkedBuilder, Marker, ConvertSaveload};
use specs::error::NoError;
use specs_derive::{Component, ConvertSaveload};
use serde::{Serialize, Deserialize};

use std::fs;
use std::fs::File;
use std::path::Path;

const SAVE_PATH: &str = "./save.json";

use super::{
    util::entityvec::EntityVec,
    Map, deck, Position, Renderable, Viewshed, BlocksTile, Name, CombatStats,
    Player, Monster, SufferDamage,
    effects, item, intent, status
};

pub struct SerializeMe;

#[derive(Component, ConvertSaveload, Clone)]
pub struct SerializableDeck {
    pub hand: EntityVec<Entity>,
    pub draw: EntityVec<Entity>,
    pub discard: EntityVec<Entity>,
    pub to_gain: deck::ToGain,
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
    Path::new(SAVE_PATH).exists()
}

pub fn save_game(ecs: &mut World) {
    // Helper to serialize map data
    let map_copy = ecs.get_mut::<Map>().unwrap().clone();
    let map_helper = ecs.create_entity()
                        .with(SerializableMap{
                            map: map_copy,
                        })
                        .marked::<SimpleMarker<SerializeMe>>()
                        .build();

    // Helper to serialize deck data
    let to_gain = { ecs.get_mut::<deck::ToGain>().unwrap().clone() };
    let deck = ecs.get_mut::<deck::Deck>().unwrap();
    let deck_copy = SerializableDeck{
        hand: EntityVec::with_existing(deck.hand.clone()),
        draw: EntityVec::with_existing(deck.draw.clone()),
        discard: EntityVec::with_existing(deck.discard.clone()),
        to_gain: to_gain,
    };
    let deck_helper = ecs.create_entity()
                        .with(deck_copy)
                        .marked::<SimpleMarker<SerializeMe>>()
                        .build();

    // Serialize and save data
    {
        let data = (ecs.entities(), ecs.read_storage::<SimpleMarker<SerializeMe>>());

        let writer = File::create(SAVE_PATH).unwrap();
        let mut serializer = serde_json::Serializer::new(writer);
        serialize_individually!(
            ecs, serializer, data, SerializableMap, SerializableDeck, Position, Renderable, Viewshed,
            BlocksTile, Name, CombatStats, Player, Monster, SufferDamage,
            item::Item, item::Card, item::Potion, item::Ethereal, item::InBackpack, item::Targeted, item::SelfTargeted, item::AreaOfEffect,
            effects::DealDamage, effects::GainBlock, effects::DiscardCard, effects::DrawCard, effects::GainCard,
            intent::PerformAction, intent::PickupItem, intent::MeleeTarget,
            status::Weak, status::Vulnerable, status::Poison
        );
    }

    ecs.delete_entity(map_helper).expect("Crashed on cleanup");
    ecs.delete_entity(deck_helper).expect("Crashed on cleanup");
}

pub fn load_game(ecs: &mut World) {
    // Delete everything
    {
        let mut to_delete = Vec::new();
        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            ecs.delete_entity(*del).expect("Unable to delete");
        }
    }

    {
        let save_data = fs::read_to_string(SAVE_PATH).unwrap();
        let mut deserializer = serde_json::Deserializer::from_str(&save_data);
        let mut data = (&mut ecs.entities(), &mut ecs.write_storage::<SimpleMarker<SerializeMe>>(), &mut ecs.write_resource::<SimpleMarkerAllocator<SerializeMe>>());
        deserialize_individually!(
            ecs, deserializer, data, SerializableMap, SerializableDeck, Position, Renderable, Viewshed,
            BlocksTile, Name, CombatStats, Player, Monster, SufferDamage,
            item::Item, item::Card, item::Potion, item::Ethereal, item::InBackpack, item::Targeted, item::SelfTargeted, item::AreaOfEffect,
            effects::DealDamage, effects::GainBlock, effects::DiscardCard, effects::DrawCard, effects::GainCard,
            intent::PerformAction, intent::PickupItem, intent::MeleeTarget,
            status::Weak, status::Vulnerable, status::Poison
        );
    }

    let mut to_delete: [Option<Entity>; 2] = [None, None];
    {
        let entities = ecs.entities();
        let map_helper = ecs.read_storage::<SerializableMap>();
        let deck_helper = ecs.read_storage::<SerializableDeck>();
        let player = ecs.read_storage::<Player>();
        let position = ecs.read_storage::<Position>();

        // Load map resource
        for (e, m) in (&entities, &map_helper).join() {
            let mut map = ecs.write_resource::<Map>();
            *map = m.map.clone();
            map.tile_content = vec![Vec::new(); super::map::MAPSIZE];
            to_delete[0] = Some(e);
        }

        // Load deck resource
        for (e, d) in (&entities, &deck_helper).join() {
            let mut deck = ecs.write_resource::<deck::Deck>();
            let mut to_gain = ecs.write_resource::<deck::ToGain>();
            deck.hand = d.hand.vec.clone();
            deck.draw = d.draw.vec.clone();
            deck.discard = d.discard.vec.clone();
            *to_gain = d.to_gain.clone();
            to_delete[1] = Some(e);
        }

        // Load player resources
        for (e, _, pos) in (&entities, &player, &position).join() {
            let mut ppos = ecs.write_resource::<rltk::Point>();
            *ppos = rltk::Point::new(pos.x, pos.y);
            let mut player_resource = ecs.write_resource::<Entity>();
            *player_resource = e;
        }
    }

    for del in to_delete.iter() {
        ecs.delete_entity(del.unwrap()).expect("Crashed on cleanup");
    }
}