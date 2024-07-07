extern crate sdl2;

mod camera;
mod event_handler;
mod game;
mod game_event;
mod game_object;
mod rust_sdl2_template;
mod texture_handler;
mod util;

use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let result = rust_sdl2_template::run_game();
    if let Err(ref _e) = result {
        // TODO handle error logging
    };
    result
}
