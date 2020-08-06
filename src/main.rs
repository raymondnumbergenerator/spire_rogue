// #![windows_subsystem = "windows"]

use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
use rltk::{Rltk, GameState, Point};

mod util;

mod gui;
mod gamelog;
mod map;
use map::Map;
mod menu;
mod player;
use player::*;
mod saveload;

mod cards;
mod deck;
mod spawner;

mod effects;
mod item;
mod intent;
mod status;
mod components;
pub use components::*;

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
    ShowHand,
    ShowTargeting { range: i32, radius: i32, item: Entity },
    DiscardCard { number: i32 },
    MainMenu { menu_selection: menu::MainMenuSelection },
    SaveGame
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
            let gain_to_hand_clone = { self.ecs.fetch_mut::<deck::ToGain>().to_hand.clone() };
            let gain_to_discard_clone = { self.ecs.fetch_mut::<deck::ToGain>().to_discard.clone() };
            {
                let mut to_gain = self.ecs.fetch_mut::<deck::ToGain>();
                to_gain.to_hand.clear();
                to_gain.to_discard.clear();
            }

            let mut gain_to_hand: Vec<Entity> = Vec::new();
            let mut gain_to_discard: Vec<Entity> = Vec::new();
            for card in gain_to_hand_clone.iter() {
                gain_to_hand.push(effects::gain_card(&mut self.ecs, *card));
            }
            for card in gain_to_discard_clone.iter() {
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
                newrunstate = player_input(self, ctx);
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
            RunState::ShowHand => {
                let result = gui::draw_hand(&mut self.ecs, ctx);
                gui::draw_play_hand(ctx);
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
                    let result = gui::draw_hand(&mut self.ecs, ctx);
                    gui::draw_discard_hand(ctx, number);
                    match result.0 {
                        gui::ItemMenuResult::Selected => {
                            let mut deck = self.ecs.write_resource::<deck::Deck>();
                            deck.discard_card(result.1.unwrap(), false);
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

    // Register components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<SimpleMarker<saveload::SerializeMe>>();
    gs.ecs.register::<saveload::SerializableMap>();
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

    // Register <gamelog::GameLog> resource
    gs.ecs.insert(gamelog::GameLog{ entries: Vec::new() });

    // Register rng <rltk::RandomNumberGenerator> resource
    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    // Register serialize marker resource
    gs.ecs.insert(SimpleMarkerAllocator::<saveload::SerializeMe>::new());

    // Create map, mark player spawn position
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    // Register player position <Point> resource
    gs.ecs.insert(Point::new(player_x, player_y));

    // Create player entity and register player <Entity> resource
    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);
    gs.ecs.insert(player_entity);

    // Create deck and register <deck::Deck> resource
    let mut initial_deck = deck::Deck{
        hand: Vec::new(),
        draw: Vec::new(),
        discard: Vec::new(),
    };
    initial_deck.gain_multiple_cards(cards::silent::starter(&mut gs.ecs));
    for _ in 0 .. 5 {
        initial_deck.draw_card();
    }
    gs.ecs.insert(initial_deck);

    // Create gain queue and register <deck::ToGain> resource
    let gain_queue = deck::ToGain{
        to_hand: Vec::new(),
        to_discard: Vec::new()
    };
    gs.ecs.insert(gain_queue);

    // Spawn mobs
    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.ecs, room);
    }

    // Register map resource
    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}
