extern crate sdl2; 

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::{Duration, SystemTime};

mod game;
mod ui;
 
fn run_game() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .fullscreen_desktop()
        .position_centered()
        .build()
        .unwrap();
 
    let refresh_rate = window.display_mode()?.refresh_rate as u32;
    let mut last_update_time = SystemTime::now();
    let mut canvas = window.into_canvas().build().unwrap();
    
    let mut game = game::Game::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let delta_time =
            match SystemTime::now().duration_since(last_update_time) {
                Err(e) => Err(e.to_string())?,
                Ok(d) => d,
            };
        game.update(delta_time)?;
        last_update_time = SystemTime::now();

        ui::draw(&game, &mut canvas)?;

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / refresh_rate));
    }

    Ok(())
}

pub fn main() -> Result<(), String> {
    let result = run_game();
    if let Err(ref _e) = result {
        // TODO handle error logging
    };
    result
}
