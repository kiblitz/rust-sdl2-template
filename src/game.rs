use std::num::Wrapping;
use std::time::Duration;

use crate::input::EventType;
use sdl2::keyboard::Keycode::Escape;
use sdl2::keyboard::Mod;

pub struct Game {
    frame: Wrapping<u8>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            frame: Wrapping(0u8),
        }
    }

    pub fn update(
        &mut self,
        events: &mut Vec<EventType>,
        _delta_time: &Duration,
        exit: &mut bool,
    ) -> Result<(), String> {
        for event in events {
            match event {
                EventType::KeyDown {
                    keycode: Escape,
                    keymod,
                    ..
                } => {
                    if keymod.contains(Mod::LGUIMOD | Mod::LCTRLMOD) {
                        *exit = true;
                    }
                }
                _ => {}
            }
        }
        self.frame += Wrapping(1u8);
        Ok(())
    }

    pub fn frame(&self) -> u8 {
        self.frame.0
    }
}
