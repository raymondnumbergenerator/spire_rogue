use rltk::{VirtualKeyCode};

pub fn number_to_option(key: VirtualKeyCode) -> i32 {
    match key {
        VirtualKeyCode::Key1 => { 1 }
        VirtualKeyCode::Key2 => { 2 }
        VirtualKeyCode::Key3 => { 3 }
        VirtualKeyCode::Key4 => { 4 }
        VirtualKeyCode::Key5 => { 5 }
        VirtualKeyCode::Key6 => { 6 }
        VirtualKeyCode::Key7 => { 7 }
        VirtualKeyCode::Key8 => { 8 }
        VirtualKeyCode::Key9 => { 9 }
        VirtualKeyCode::Key0 => { 10 }
        _ => { -1 }
    }
}