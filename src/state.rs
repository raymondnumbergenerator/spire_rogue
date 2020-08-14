use specs::prelude::*;
use rltk::{Rltk, GameState, Point};

use super::{
    creature, deck, effects, gui, item,
    map, menu, monsters, player, saveload, spawner, systems,
    Position, Renderable, Gamelog, Map,
};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    PreRun,
    AwaitingInput,
    PlayerTurn,
    EndTurn { player_end_turn: bool },
    MonsterTurn,
    ShowInventory,
    ShowHand { selection: i32 },
    ShowTargeting { action:Entity, range: i32, radius: i32 },
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
        let mut monster_sys = systems::MonsterSystem{};
        monster_sys.run_now(&self.ecs);
        systems::action::run(&mut self.ecs);
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

    fn take_action(&mut self, runstate: RunState, result: (gui::ItemMenuResult, Option<Entity>)) -> RunState {
        let mut newrunstate = runstate;

        match result.0 {
            gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
            gui::ItemMenuResult::NoResponse => {},
            gui::ItemMenuResult::Selected => {
                let action = result.1.unwrap();
                let mut radius = 0;
                if let Some(r) = self.ecs.read_storage::<item::AreaOfEffect>().get(action) {
                    radius = r.radius;
                }

                if let Some(targeted_action) = self.ecs.read_storage::<item::Targeted>().get(action) {
                    newrunstate = RunState::ShowTargeting{
                        action,
                        range: targeted_action.range, 
                        radius
                    };
                } else {
                    let mut intent = self.ecs.write_storage::<creature::PerformAction>();
                    intent.insert(*self.ecs.fetch::<Entity>(), creature::PerformAction{ action, target: None }).expect("Unable to insert creature::PerformAction");

                    // Check if action requires discard
                    if let Some(require_discard) = self.ecs.read_storage::<effects::DiscardCard>().get(action) {
                        newrunstate = RunState::DiscardCard{ number: require_discard.number };
                    } else {
                        newrunstate = RunState::PlayerTurn;
                    }
                }
                self.run_systems();
                self.ecs.maintain();
            }
        }

        newrunstate
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
            let mut player_pos = self.ecs.write_resource::<Point>();
            *player_pos = Point::new(player_x, player_y);

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

        let mut log = self.ecs.fetch_mut::<Gamelog>();
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
            RunState::EndTurn{player_end_turn} => {
                self.run_systems();
                self.ecs.maintain();
                if player_end_turn {
                    newrunstate = RunState::MonsterTurn;
                } else {
                    newrunstate = RunState::AwaitingInput;
                }
            }
            RunState::MonsterTurn => {
                self.run_systems();
                self.ecs.maintain();

                // Find all used intents
                let mut to_update: Vec<Option<monsters::Attacks>> = Vec::new();
                {
                    let attack_cycles = { self.ecs.read_storage::<creature::AttackCycle>().clone() };
                    let intents = self.ecs.read_storage::<creature::Intent>();

                    for (ac, intent) in (&attack_cycles, &intents).join() {
                        match intent.used {
                            true => to_update.push(Some(ac.attacks[ac.cycle].clone())),
                            false => to_update.push(None),
                        }
                    }
                }

                // Generate new intents
                let mut new_intents: Vec<Option<Entity>> = Vec::new();
                {
                    for attacks in to_update {
                        match attacks {
                            Some(atk) => new_intents.push(Some(atk.to_attack(&mut self.ecs))),
                            None => new_intents.push(None)
                        }
                    }
                }

                // Assign new intents
                let mut to_delete: Vec<Entity> = Vec::new();
                {
                    let mut monster_intents = self.ecs.write_storage::<creature::Intent>();

                    for (i, mut intent) in (&mut monster_intents).join().enumerate() {
                        match new_intents[i] {
                            Some(new_intent) => {
                                to_delete.push(intent.intent);
                                intent.intent = new_intent;
                            }
                            None => {}
                        }
                    }
                }

                // Delete old intents
                for old in to_delete {
                    self.ecs.delete_entity(old).expect("Unable to delete old monster intent");
                }

                newrunstate = RunState::EndTurn{ player_end_turn: false };
            }
            RunState::ShowInventory => {
                let result = gui::draw_inventory(&mut self.ecs, ctx);
                newrunstate = self.take_action(newrunstate, result);
            }
            RunState::ShowHand{selection} => {
                let result = gui::pick_card(&mut self.ecs, selection);
                newrunstate = self.take_action(newrunstate, result);
            }
            RunState::ShowTargeting{action, range, radius} => {
                let result = gui::ranged_target(&self.ecs, ctx, range, radius);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        {
                            let mut intent = self.ecs.write_storage::<creature::PerformAction>();
                            intent.insert(*self.ecs.fetch::<Entity>(), creature::PerformAction{ action, target: result.1 }).expect("Unable to insert creature::PerformAction");

                            // Check if action requires discard
                            if let Some(require_discard) = self.ecs.read_storage::<effects::DiscardCard>().get(action) {
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