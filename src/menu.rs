use specs::prelude::*;
use rltk::{Rltk, RGB, VirtualKeyCode};

use super::{RunState};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MainMenuSelection {
    NewGame,
    LoadGame,
    Quit,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MainMenuResult {
    NoSelection{ selected: MainMenuSelection },
    Selected{ selected: MainMenuSelection },
}

pub fn main_menu(ecs: &mut World, ctx: &mut Rltk) -> MainMenuResult {
    let runstate = ecs.fetch::<RunState>();
    let x = 5;

    // ctx.print(x, 15, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Spire Rogue");
    let color_selected = RGB::named(rltk::MAGENTA);
    let color_unselected = RGB::named(rltk::WHITE);

    if let RunState::MainMenu{ menu_selection: selection } = *runstate {
        let mut y = super::WINDOWHEIGHT - 8;

        let mut selected = color_unselected;
        if let MainMenuSelection::NewGame = selection { selected = color_selected; };
        ctx.print_color(x, y, selected, RGB::named(rltk::BLACK), "New Game");
        y += 1;
        
        let mut selected = color_unselected;
        if let MainMenuSelection::LoadGame = selection { selected = color_selected; };
        ctx.print_color(x, y, selected, RGB::named(rltk::BLACK), "Load Game");
        y += 1;

        let mut selected = color_unselected;
        if let MainMenuSelection::Quit = selection { selected = color_selected; };
        ctx.print_color(x, y, selected, RGB::named(rltk::BLACK), "Quit");

        match ctx.key {
            None => return MainMenuResult::NoSelection{ selected: selection },
            Some(key) => {
                match key {
                    VirtualKeyCode::Escape => {
                        return MainMenuResult::NoSelection{ selected: MainMenuSelection::Quit }
                    }
                    VirtualKeyCode::Up | VirtualKeyCode::W => {
                        let newselection;
                        match selection {
                            MainMenuSelection::NewGame => newselection = MainMenuSelection::Quit,
                            MainMenuSelection::LoadGame => newselection = MainMenuSelection::NewGame,
                            MainMenuSelection::Quit => newselection = MainMenuSelection::LoadGame,
                        }
                        return MainMenuResult::NoSelection{ selected: newselection }
                    }
                    VirtualKeyCode::Down | VirtualKeyCode::S => {
                        let newselection;
                        match selection {
                            MainMenuSelection::NewGame => newselection = MainMenuSelection::LoadGame,
                            MainMenuSelection::LoadGame => newselection = MainMenuSelection::Quit,
                            MainMenuSelection::Quit => newselection = MainMenuSelection::NewGame,
                        }
                        return MainMenuResult::NoSelection{ selected: newselection }
                    }
                    VirtualKeyCode::Return | VirtualKeyCode::Space => {
                        return MainMenuResult::Selected{ selected: selection }
                    }
                    _ => return MainMenuResult::NoSelection{ selected: selection }
                }
            }
    
        }
    }

    MainMenuResult::NoSelection { selected: MainMenuSelection::NewGame }
}