use sdl2::keyboard::{Keycode, Mod};

pub enum EventType {
    KeyUp {
        keycode: Keycode,
        keymod: Mod,
        repeat: bool,
    },
    KeyDown {
        keycode: Keycode,
        keymod: Mod,
        repeat: bool,
    },
}
