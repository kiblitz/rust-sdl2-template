extern crate sdl2;

mod event_handler;
mod game;
mod game_object;
mod texture_handler;
mod util;

use game_object::{Drawable, Updatable};

use std::error::Error;
use std::time::SystemTime;

fn run_game() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("dogfight", 800, 600)
        .fullscreen_desktop()
        .position_centered()
        .build()
        .unwrap();

    let mut last_update_time = SystemTime::now();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.default_pixel_format();

    let texture_creator = canvas.texture_creator();
    texture_creator.default_pixel_format();
    let texture_handler = texture_handler::TextureHandler::new(&texture_creator)?;

    let mut game = game::Game::new(&texture_handler);
    let mut event_handler = event_handler::EventHandler::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            if !event_handler.consume(&event) {
                break 'running;
            }
        }

        let delta_time = SystemTime::now()
            .duration_since(last_update_time)
            .map_err(|e| e.to_string())
            .map(|duration| duration.as_secs_f32())?;
        game.update(&event_handler, &delta_time)?;
        game.draw(&mut canvas)?;
        last_update_time = SystemTime::now();

        canvas.present();

        std::thread::sleep(util::REFRESH_EVERY);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let result = run_game();
    if let Err(ref _e) = result {
        // TODO handle error logging
    };
    result
}
