use crate::input::EventType;

use std::error::Error;
use std::num::Wrapping;
use std::time::Duration;

use sdl2::keyboard::Keycode::Down;
use sdl2::keyboard::Keycode::Escape;
use sdl2::keyboard::Keycode::Left;
use sdl2::keyboard::Keycode::Right;
use sdl2::keyboard::Keycode::Up;
use sdl2::keyboard::Mod;

pub struct Game {
    frame: Wrapping<u8>,
    x: i32,
    y: i32,
    delta: i32,
    up_down: bool,
    down_down: bool,
    left_down: bool,
    right_down: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            frame: Wrapping(0u8),
            x: 0,
            y: 0,
            delta: 10,
            up_down: false,
            down_down: false,
            left_down: false,
            right_down: false,
        }
    }

    pub fn update(
        &mut self,
        events: &mut Vec<EventType>,
        _delta_time: &Duration,
        exit: &mut bool,
    ) -> Result<(), Box<dyn Error>> {
        for event in events {
            match event {
                EventType::KeyDown {
                    keycode: Escape,
                    keymod,
                    ..
                } => {
                    // Windows key AND ctrl key
                    if keymod.contains(Mod::LGUIMOD | Mod::LCTRLMOD) {
                        *exit = true;
                    }
                }
                EventType::KeyDown { keycode: Up, .. } => {
                    self.up_down = true;
                }
                EventType::KeyDown { keycode: Down, .. } => {
                    self.down_down = true;
                }
                EventType::KeyDown { keycode: Left, .. } => {
                    self.left_down = true;
                }
                EventType::KeyDown { keycode: Right, .. } => {
                    self.right_down = true;
                }
                EventType::KeyUp { keycode: Up, .. } => {
                    self.up_down = false;
                }
                EventType::KeyUp { keycode: Down, .. } => {
                    self.down_down = false;
                }
                EventType::KeyUp { keycode: Left, .. } => {
                    self.left_down = false;
                }
                EventType::KeyUp { keycode: Right, .. } => {
                    self.right_down = false;
                }
                _ => {}
            }
        }
        if self.up_down {
            self.y -= self.delta;
        }
        if self.down_down {
            self.y += self.delta;
        }
        if self.left_down {
            self.x -= self.delta;
        }
        if self.right_down {
            self.x += self.delta;
        }
        self.frame += Wrapping(1u8);
        Ok(())
    }

    pub fn frame(&self) -> u8 {
        self.frame.0
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
