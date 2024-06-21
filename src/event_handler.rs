use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct EventHandler {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn consume(&mut self, event: &Event) -> bool {
        match event {
            Event::Quit { .. } => false,
            Event::KeyDown {
                keycode: Some(key_code),
                ..
            } => {
                match key_code {
                    Keycode::W => self.up = true,
                    Keycode::S => self.down = true,
                    Keycode::A => self.left = true,
                    Keycode::D => self.right = true,
                    _ => (),
                };
                true
            }
            Event::KeyUp {
                keycode: Some(key_code),
                ..
            } => {
                match key_code {
                    Keycode::W => self.up = false,
                    Keycode::S => self.down = false,
                    Keycode::A => self.left = false,
                    Keycode::D => self.right = false,
                    _ => (),
                };
                true
            }
            _ => true,
        }
    }

    pub fn up(&self) -> bool {
        return self.up;
    }
    pub fn down(&self) -> bool {
        return self.down;
    }
    pub fn left(&self) -> bool {
        return self.left;
    }
    pub fn right(&self) -> bool {
        return self.right;
    }
}
