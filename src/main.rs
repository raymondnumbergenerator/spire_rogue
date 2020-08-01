use specs::prelude::*;
use rltk::{Rltk, GameState, Point};

mod util;

mod gui;
mod gamelog;
mod map;
use map::Map;
mod player;
use player::*;

mod cards;
mod deck;
mod spawner;

mod effects;
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
        let mut item_use_sys = systems::ItemUseSystem{};
        item_use_sys.run_now(&self.ecs);
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

        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        // State machine
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
                newrunstate = RunState::EndTurn{player_turn: false};
            }
            RunState::ShowInventory => {
                let result = gui::draw_inventory(&mut self.ecs, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut radius = 0;
                        if let Some(r) = self.ecs.read_storage::<AreaOfEffect>().get(item_entity) {
                            radius = r.radius;
                        }

                        if let Some(ranged_item) = self.ecs.read_storage::<Targeted>().get(item_entity) {
                            newrunstate = RunState::ShowTargeting{
                                range: ranged_item.range, 
                                radius: radius,
                                item: item_entity
                            };
                        } else {
                            let mut intent = self.ecs.write_storage::<intent::UseItem>();
                            intent.insert(*self.ecs.fetch::<Entity>(), intent::UseItem{ item: item_entity, target: None }).expect("Unable to insert intent::UseItem");
                            newrunstate = RunState::PlayerTurn;
                        }
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
                        let item_entity = result.1.unwrap();
                        let mut radius = 0;
                        if let Some(r) = self.ecs.read_storage::<AreaOfEffect>().get(item_entity) {
                            radius = r.radius;
                        }

                        if let Some(ranged_item) = self.ecs.read_storage::<Targeted>().get(item_entity) {
                            newrunstate = RunState::ShowTargeting{
                                range: ranged_item.range, 
                                radius: radius,
                                item: item_entity
                            };
                        } else {
                            let mut intent = self.ecs.write_storage::<intent::UseItem>();
                            intent.insert(*self.ecs.fetch::<Entity>(), intent::UseItem{ item: item_entity, target: None }).expect("Unable to insert intent::UseItem");
                            newrunstate = RunState::PlayerTurn;
                        }
                    }
                }
            }
            RunState::ShowTargeting{range, radius, item} => {
                let result = gui::ranged_target(&self.ecs, ctx, range, radius);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {},
                    gui::ItemMenuResult::Selected => {
                        let mut intent = self.ecs.write_storage::<intent::UseItem>();
                        intent.insert(*self.ecs.fetch::<Entity>(), intent::UseItem{ item: item, target: result.1 }).expect("Unable to insert intent::UseItem");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
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

    // Create gamestate and register runstate resource
    let mut gs = State{ ecs: World::new() };
    gs.ecs.insert(RunState::PreRun);

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
    gs.ecs.register::<Card>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Potion>();
    gs.ecs.register::<InBackpack>();
    gs.ecs.register::<Targeted>();
    gs.ecs.register::<AreaOfEffect>();

    gs.ecs.register::<effects::DealDamage>();
    gs.ecs.register::<effects::GainBlock>();
    gs.ecs.register::<effects::DiscardCard>();
    gs.ecs.register::<intent::UseItem>();
    gs.ecs.register::<intent::PickupItem>();
    gs.ecs.register::<intent::MeleeTarget>();
    gs.ecs.register::<status::Weak>();
    gs.ecs.register::<status::Vulnerable>();

    // Register gamelog resource
    gs.ecs.insert(gamelog::GameLog{ entries: Vec::new() });

    // Register rng resource
    gs.ecs.insert(rltk::RandomNumberGenerator::new());

    // Create map, mark player spawn position
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(Point::new(player_x, player_y));

    // Create player entity and register player resource
    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);
    gs.ecs.insert(player_entity);

    // Create deck and register deck resource
    let mut initial_deck = deck::Deck{
        hand: Vec::new(),
        draw: Vec::new(),
        discard: Vec::new(),
    };
    initial_deck.gain_multiple_cards(cards::ironclad::starter(&mut gs.ecs));
    for _ in 0..5 {
        initial_deck.draw();
    }
    gs.ecs.insert(initial_deck);

    // Spawn mobs
    for room in map.rooms.iter().skip(1) {
        spawner::spawn_room(&mut gs.ecs, room);
    }
    gs.ecs.insert(map);

    rltk::main_loop(context, gs)
}
