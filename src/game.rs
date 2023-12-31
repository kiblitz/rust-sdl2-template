use crate::input::EventType;

use std::error::Error;
use std::num::Wrapping;
use std::time::Duration;

use sdl2::keyboard::Keycode::{Down, Escape, Left, Right, Up};
use sdl2::keyboard::Keycode::{A, D, S, W};
use sdl2::keyboard::Mod;

#[derive(Eq, PartialEq)]
pub enum State {
    Play,
    Exit,
}

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

    fn handle_events(&mut self, events: &Vec<EventType>) -> State {
        let mut state = State::Play;
        for event in events {
            match event {
                // Escape AND windows key AND ctrl key
                EventType::KeyDown {
                    keycode: Escape,
                    keymod,
                    ..
                } if keymod.contains(Mod::LGUIMOD | Mod::LCTRLMOD) => {
                    state = State::Exit;
                }
                EventType::KeyDown { keycode: Up, .. } | EventType::KeyDown { keycode: W, .. } => {
                    self.up_down = true;
                }
                EventType::KeyDown { keycode: Down, .. }
                | EventType::KeyDown { keycode: S, .. } => {
                    self.down_down = true;
                }
                EventType::KeyDown { keycode: Left, .. }
                | EventType::KeyDown { keycode: A, .. } => {
                    self.left_down = true;
                }
                EventType::KeyDown { keycode: Right, .. }
                | EventType::KeyDown { keycode: D, .. } => {
                    self.right_down = true;
                }
                EventType::KeyUp { keycode: Up, .. } | EventType::KeyUp { keycode: W, .. } => {
                    self.up_down = false;
                }
                EventType::KeyUp { keycode: Down, .. } | EventType::KeyUp { keycode: S, .. } => {
                    self.down_down = false;
                }
                EventType::KeyUp { keycode: Left, .. } | EventType::KeyUp { keycode: A, .. } => {
                    self.left_down = false;
                }
                EventType::KeyUp { keycode: Right, .. } | EventType::KeyUp { keycode: D, .. } => {
                    self.right_down = false;
                }
                _ => {}
            }
        }
        state
    }
    pub fn update(
        &mut self,
        events: &Vec<EventType>,
        _delta_time: &Duration,
    ) -> Result<State, Box<dyn Error>> {
        let state = self.handle_events(&events);
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
        Ok(state)
    }

    pub fn frame(&self) -> u8 {
        self.frame.0
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
