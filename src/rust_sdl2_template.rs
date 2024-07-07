use crate::event_handler::EventHandler;
use crate::game::Game;
use crate::texture_handler::TextureHandler;
use crate::util;

use std::error::Error;
use std::time::SystemTime;

pub fn run_game() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2-template", 0, 0)
        .fullscreen_desktop()
        .position_centered()
        .build()
        .unwrap();

    let mut last_update_time = SystemTime::now();
    let canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    texture_creator.default_pixel_format();
    let texture_handler = TextureHandler::new(&texture_creator)?;

    let mut game = Game::new(canvas, &texture_handler);
    let mut event_handler = EventHandler::new();

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
        game.update(&texture_handler, &event_handler, delta_time)?;
        game.draw()?;
        last_update_time = SystemTime::now();

        std::thread::sleep(util::REFRESH_EVERY);
    }

    Ok(())
}
