use std::num::Wrapping;
use std::time::Duration;

pub struct Game {
    frame : Wrapping<u8>,
}

impl Game {
    pub fn new() -> Self {
        Self { frame: Wrapping(0u8) }
    }

    pub fn update(&mut self, _delta_time : Duration) -> Result<(), String> {
        self.frame += Wrapping(1u8);
        Ok(())
    }

    pub fn frame(&self) -> u8 {
        self.frame.0
    }
}
