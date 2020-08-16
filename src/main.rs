// #![windows_subsystem = "windows"]

use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use rltk::Point;

mod util;

mod components;
use components::component::{Name, Position, Renderable};
use components::creature;
use components::effects;
use components::item;
use components::status;

mod gui;
mod menu;

mod gamelog;
use gamelog::Gamelog;
mod map;
use map::Map;
mod player;

mod cards;
mod deck;
mod monsters;
mod spawner;

mod saveload;
mod systems;

mod state;
use state::RunState;
use state::State;

pub const WINDOWWIDTH: usize = 80;
pub const WINDOWHEIGHT: usize = 50;

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple80x50()
        .with_title("spire_rogue")
        .build()?;
    context.with_post_scanlines(true);

    // Create gamestate and register <RunState> resource
    let mut gs = State{ ecs: World::new() };
    gs.ecs.insert(RunState::MainMenu{ menu_selection: menu::MainMenuSelection::NewGame });

    // Register rng <rltk::RandomNumberGenerator> resource
    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    // Register <GameLog> resource
    gs.ecs.insert(Gamelog{ entries: Vec::new() });

    // Register serialize marker resource
    gs.ecs.insert(SimpleMarkerAllocator::<saveload::SerializeMe>::new());

    // Register components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Name>();

    gs.ecs.register::<creature::Creature>();
    gs.ecs.register::<creature::Player>();
    gs.ecs.register::<creature::Monster>();
    gs.ecs.register::<creature::CombatStats>();
    gs.ecs.register::<creature::BlocksTile>();
    gs.ecs.register::<creature::Viewshed>();
    gs.ecs.register::<creature::SufferDamage>();
    gs.ecs.register::<creature::PerformAction>();
    gs.ecs.register::<creature::PickupItem>();
    gs.ecs.register::<creature::Attack>();
    gs.ecs.register::<creature::Intent>();
    gs.ecs.register::<creature::AttackCycle>();

    gs.ecs.register::<SimpleMarker<saveload::SerializeMe>>();
    gs.ecs.register::<saveload::SerializableResources>();
    gs.ecs.register::<saveload::SerializableDeck>();

    gs.ecs.register::<effects::DealDamage>();
    gs.ecs.register::<effects::GainBlock>();
    gs.ecs.register::<effects::DiscardCard>();
    gs.ecs.register::<effects::DrawCard>();
    gs.ecs.register::<effects::GainCard>();
    gs.ecs.register::<effects::BuffStrength>();
    gs.ecs.register::<effects::BuffDexterity>();

    gs.ecs.register::<item::Item>();
    gs.ecs.register::<item::Potion>();
    gs.ecs.register::<item::InBackpack>();
    gs.ecs.register::<item::Card>();
    gs.ecs.register::<item::Ethereal>();
    gs.ecs.register::<item::Fragile>();
    gs.ecs.register::<item::Targeted>();
    gs.ecs.register::<item::AreaOfEffect>();

    gs.ecs.register::<status::Weak>();
    gs.ecs.register::<status::Vulnerable>();
    gs.ecs.register::<status::Frail>();
    gs.ecs.register::<status::Poison>();

    // Create map, mark player spawn position
    let map = Map::new_map_rooms_and_corridors(1);
    let (player_x, player_y) = map.rooms[0].center();

    // Register player position <Point> resource
    gs.ecs.insert(Point::new(player_x, player_y));

    // Create player entity and register player <Entity> resource
    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);
    gs.ecs.insert(player_entity);

    // Spawn mobs
    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.ecs, room, 1);
    }

    // Create deck and register <deck::Deck> resource
    let mut deck = deck::Deck{
        hand: Vec::new(),
        draw: Vec::new(),
        discard: Vec::new(),
    };
    deck.gain_multiple_cards(cards::silent::starter(&mut gs.ecs));
    for _ in 0 .. 5 {
        deck.draw_card();
    }
    gs.ecs.insert(deck);

    // Register <Map> resource
    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}
