// #![windows_subsystem = "windows"]

use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use rltk::{Rltk, GameState, Point};

mod util;

mod components;
use components::component::{Name, Position, Renderable};
use components::creature;
use components::effects;
use components::intent;
use components::item;
use components::status;

mod gui;
mod menu;

mod gamelog;
use gamelog::GameLog;
mod map;
use map::Map;
mod player;

mod cards;
mod deck;
mod spawner;

mod saveload;
mod systems;

pub const WINDOWWIDTH: usize = 80;
pub const WINDOWHEIGHT: usize = 50;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    PreRun,
    AwaitingInput,
    PlayerTurn,
    EndTurn { player_turn: bool },
    MonsterTurn,
    ShowInventory,
    ShowHand { selection: i32 },
    ShowTargeting { range: i32, radius: i32, item: Entity },
    DiscardCard { number: i32 },
    MainMenu { menu_selection: menu::MainMenuSelection },
    SaveGame,
    NextLevel,
}

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut visibility_sys = systems::VisibilitySystem{};
        visibility_sys.run_now(&self.ecs);
        let mut inventory_sys = systems::InventorySystem{};
        inventory_sys.run_now(&self.ecs);
        let mut action_sys = systems::ActionSystem{};
        action_sys.run_now(&self.ecs);

        // Resolve effects that gain cards
        {
            let gain_queue_hand = { self.ecs.fetch_mut::<effects::GainCardQueue>().to_hand.clone() };
            let gain_queue_discard = { self.ecs.fetch_mut::<effects::GainCardQueue>().to_discard.clone() };
            {
                let mut gain_queue = self.ecs.fetch_mut::<effects::GainCardQueue>();
                gain_queue.to_hand.clear();
                gain_queue.to_discard.clear();
            }

            let mut gain_to_hand: Vec<Entity> = Vec::new();
            let mut gain_to_discard: Vec<Entity> = Vec::new();
            for card in gain_queue_hand.iter() {
                gain_to_hand.push(effects::gain_card(&mut self.ecs, *card));
            }
            for card in gain_queue_discard.iter() {
                gain_to_discard.push(effects::gain_card(&mut self.ecs, *card));
            }

            let mut deck = self.ecs.fetch_mut::<deck::Deck>();
            for card in gain_to_hand.iter() {
                deck.gain_to_hand(*card);
            }
            for card in gain_to_discard.iter() {
                deck.gain_card(*card);
            }
        }

        let mut monster_sys = systems::MonsterSystem{};
        monster_sys.run_now(&self.ecs);
        let mut melee_combat_sys = systems::MeleeCombatSystem{};
        melee_combat_sys.run_now(&self.ecs);
        let mut damage_sys = systems::DamageSystem{};
        damage_sys.run_now(&self.ecs);
        let mut cleanup_sys = systems::DeadCleanupSystem{};
        cleanup_sys.run_now(&self.ecs);
        let mut end_turn_sys = systems::EndTurnSystem{};
        end_turn_sys.run_now(&self.ecs);
        self.ecs.maintain();
        let mut map_sys = systems::MapIndexSystem{};
        map_sys.run_now(&self.ecs);
    }

    fn to_cleanup(&mut self) -> Vec<Entity> {
        let entities = self.ecs.entities();
        let player_entity = self.ecs.fetch::<Entity>();
        let deck = self.ecs.fetch::<deck::Deck>();
        let player = self.ecs.read_storage::<creature::Player>();
        let backpack = self.ecs.read_storage::<item::InBackpack>();

        let mut to_delete: Vec<Entity> = Vec::new();
        for entity in entities.join() {
            let mut should_delete = true;

            // Don't delete player
            if let Some(_) = player.get(entity) {
                should_delete = false;
            }

            // Don't delete player's inventory
            if let Some(e) = backpack.get(entity) {
                if e.owner == *player_entity {
                    should_delete = false;
                }
            }

            // Don't delete cards in player's deck
            if deck.hand.contains(&entity) || deck.draw.contains(&entity) || deck.discard.contains(&entity) {
                should_delete = false;
            }

            if should_delete {
                to_delete.push(entity);
            }
        }

        to_delete
    }

    fn next_level(&mut self) {
        // Cleanup entities
        let to_delete = self.to_cleanup();
        for entity in to_delete {
            self.ecs.delete_entity(entity).expect("Unable to delete entity");
        }

        // Build a new map
        let map: Map;
        {
            // Create map and update <Map> resource
            let mut map_resource = self.ecs.write_resource::<Map>();
            let current_depth = map_resource.depth;
            *map_resource = Map::new_map_rooms_and_corridors(current_depth + 1);
            map = map_resource.clone();

            // Update player position <Point> resource
            let (player_x, player_y) = map.rooms[0].center();
            let mut ppos = self.ecs.write_resource::<Point>();
            *ppos = Point::new(player_x, player_y);

            // Update player entity with new position
            let mut positions = self.ecs.write_storage::<Position>();
            let player_entity = self.ecs.fetch::<Entity>();
            if let Some(pos) = positions.get_mut(*player_entity) {
                pos.x = player_x;
                pos.y = player_y;
            }

            // Mark player's viewshed as dirty
            let mut viewsheds = self.ecs.write_storage::<creature::Viewshed>();
            if let Some(vs) = viewsheds.get_mut(*player_entity) {
                vs.dirty = true;
            }
        }

        // Spawn mobs
        for room in map.rooms.iter().skip(1) {
            spawner::spawn_room(&mut self.ecs, room);
        }

        let mut log = self.ecs.fetch_mut::<GameLog>();
        log.push("You descend to the next level.".to_string());
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        // State machine
        match newrunstate {
            RunState::MainMenu{..} => {}
            _ => {
                map::draw_map(&self.ecs, ctx);
                {
                    let positions = self.ecs.read_storage::<Position>();
                    let renderables = self.ecs.read_storage::<Renderable>();
                    let map = self.ecs.fetch::<Map>();
            
                    let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
                    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
            
                    for (pos, render) in data.iter() {
                        let idx = map.xy_idx(pos.x, pos.y);
                        if map.visible_tiles[idx] { ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph); }
                    }
        
                    gui::draw_ui(&self.ecs, ctx);
                }
            }
        }
        match newrunstate {
            RunState::PreRun => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player::player_input(&mut self.ecs, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::EndTurn{player_turn} => {
                self.run_systems();
                self.ecs.maintain();
                if player_turn {
                    newrunstate = RunState::MonsterTurn;
                } else {
                    newrunstate = RunState::AwaitingInput;
                }
            }
            RunState::MonsterTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::EndTurn{ player_turn: false };
            }
            RunState::ShowInventory => {
                let result = gui::draw_inventory(&mut self.ecs, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let potion = result.1.unwrap();
                        let mut radius = 0;
                        if let Some(r) = self.ecs.read_storage::<item::AreaOfEffect>().get(potion) {
                            radius = r.radius;
                        }

                        if let Some(targeted_action) = self.ecs.read_storage::<item::Targeted>().get(potion) {
                            newrunstate = RunState::ShowTargeting{
                                range: targeted_action.range, 
                                radius: radius,
                                item: potion
                            };
                        } else {
                            let mut intent = self.ecs.write_storage::<intent::PerformAction>();
                            intent.insert(*self.ecs.fetch::<Entity>(), intent::PerformAction{ action: potion, target: None }).expect("Unable to insert intent::PerformAction");
                            // Check if action requires discard
                            if let Some(require_discard) = self.ecs.read_storage::<effects::DiscardCard>().get(potion) {
                                newrunstate = RunState::DiscardCard{ number: require_discard.number };
                            } else {
                                newrunstate = RunState::PlayerTurn;
                            }
                        }
                        self.run_systems();
                        self.ecs.maintain();
                    }
                }
            }
            RunState::ShowHand{selection} => {
                let result = gui::pick_card(&mut self.ecs, selection);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let card = result.1.unwrap();
                        let mut radius = 0;
                        if let Some(r) = self.ecs.read_storage::<item::AreaOfEffect>().get(card) {
                            radius = r.radius;
                        }

                        if let Some(targeted_action) = self.ecs.read_storage::<item::Targeted>().get(card) {
                            newrunstate = RunState::ShowTargeting{
                                range: targeted_action.range, 
                                radius: radius,
                                item: card
                            };
                        } else {
                            let mut intent = self.ecs.write_storage::<intent::PerformAction>();
                            intent.insert(*self.ecs.fetch::<Entity>(), intent::PerformAction{ action: card, target: None }).expect("Unable to insert intent::PerformAction");
                            // Check if action requires discard
                            if let Some(require_discard) = self.ecs.read_storage::<effects::DiscardCard>().get(card) {
                                newrunstate = RunState::DiscardCard{ number: require_discard.number };
                            } else {
                                newrunstate = RunState::PlayerTurn;
                            }
                        }
                        self.run_systems();
                        self.ecs.maintain();
                    }
                }
            }
            RunState::ShowTargeting{range, radius, item} => {
                let result = gui::ranged_target(&self.ecs, ctx, range, radius);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        {
                            let mut intent = self.ecs.write_storage::<intent::PerformAction>();
                            intent.insert(*self.ecs.fetch::<Entity>(), intent::PerformAction{ action: item, target: result.1 }).expect("Unable to insert intent::PerformAction");
                            // Check if action requires discard
                            if let Some(require_discard) = self.ecs.read_storage::<effects::DiscardCard>().get(item) {
                                newrunstate = RunState::DiscardCard{ number: require_discard.number };
                            } else {
                                newrunstate = RunState::PlayerTurn;
                            }
                        }
                        self.run_systems();
                        self.ecs.maintain();
                    }
                }
            }
            RunState::DiscardCard{number} => {
                let hand_len = self.ecs.read_resource::<deck::Deck>().hand.len();
                if number == 0 || hand_len == 0 {
                    newrunstate = RunState::PlayerTurn;
                } else {
                    let result = gui::discard_card(&mut self.ecs, ctx, number);
                    match result.0 {
                        gui::ItemMenuResult::Selected => {
                            let ethereal = self.ecs.read_storage::<item::Ethereal>();
                            let mut deck = self.ecs.write_resource::<deck::Deck>();
                            if let Some(_) = ethereal.get(result.1.unwrap()) {
                                deck.discard_card(result.1.unwrap(), true);
                            } else {
                                deck.discard_card(result.1.unwrap(), false);
                            }
                            newrunstate = RunState::DiscardCard{ number: number - 1 };
                        }
                        _ => {}
                    }
                }
            }
            RunState::MainMenu{..} => {
                let result = menu::main_menu(&mut self.ecs, ctx);
                match result {
                    menu::MainMenuResult::NoSelection{ selected } => newrunstate = RunState::MainMenu{ menu_selection: selected },
                    menu::MainMenuResult::Selected{ selected } => {
                        match selected {
                            menu::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                            menu::MainMenuSelection::LoadGame => {
                                saveload::load_game(&mut self.ecs);
                                newrunstate = RunState::AwaitingInput;
                            }
                            menu::MainMenuSelection::Quit => { ::std::process::exit(0); }
                        }
                    }
                }
            }
            RunState::SaveGame => {
                saveload::save_game(&mut self.ecs);
                newrunstate = RunState::MainMenu{ menu_selection : menu::MainMenuSelection::LoadGame };
            }
            RunState::NextLevel => {
                self.next_level();
                newrunstate = RunState::PreRun;
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
    }
}

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
    gs.ecs.insert(GameLog{ entries: Vec::new() });

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

    gs.ecs.register::<SimpleMarker<saveload::SerializeMe>>();
    gs.ecs.register::<saveload::SerializableResources>();
    gs.ecs.register::<saveload::SerializableDeck>();

    gs.ecs.register::<effects::DealDamage>();
    gs.ecs.register::<effects::GainBlock>();
    gs.ecs.register::<effects::DiscardCard>();
    gs.ecs.register::<effects::DrawCard>();
    gs.ecs.register::<effects::GainCard>();

    gs.ecs.register::<item::Item>();
    gs.ecs.register::<item::Card>();
    gs.ecs.register::<item::Potion>();
    gs.ecs.register::<item::Ethereal>();
    gs.ecs.register::<item::InBackpack>();
    gs.ecs.register::<item::Targeted>();
    gs.ecs.register::<item::SelfTargeted>();
    gs.ecs.register::<item::AreaOfEffect>();

    gs.ecs.register::<intent::PerformAction>();
    gs.ecs.register::<intent::PickupItem>();
    gs.ecs.register::<intent::MeleeTarget>();

    gs.ecs.register::<status::Weak>();
    gs.ecs.register::<status::Vulnerable>();
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
        spawner::spawn_room(&mut gs.ecs, room);
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

    // Create gain queue and register <deck::ToGain> resource
    let gain_queue = effects::GainCardQueue{
        to_hand: Vec::new(),
        to_discard: Vec::new()
    };
    gs.ecs.insert(gain_queue);

    // Register <Map> resource
    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}
