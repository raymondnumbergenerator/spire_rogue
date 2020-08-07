use rltk::{VirtualKeyCode};

pub fn number_to_option(key: VirtualKeyCode) -> i32 {
    match key {
        VirtualKeyCode::Key1 => { 0 }
        VirtualKeyCode::Key2 => { 1 }
        VirtualKeyCode::Key3 => { 2 }
        VirtualKeyCode::Key4 => { 3 }
        VirtualKeyCode::Key5 => { 4 }
        VirtualKeyCode::Key6 => { 5 }
        VirtualKeyCode::Key7 => { 6 }
        VirtualKeyCode::Key8 => { 7 }
        VirtualKeyCode::Key9 => { 8 }
        VirtualKeyCode::Key0 => { 9 }
        _ => { -1 }
    }
}