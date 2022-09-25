extern crate sdl2;

use sdl2::event::Event;
use std::time::{Duration, SystemTime};

mod game;
mod input;
mod ui;

fn run_game() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .fullscreen_desktop()
        .position_centered()
        .build()
        .unwrap();

    let refresh_rate = window.display_mode()?.refresh_rate as u32;
    let mut last_update_time = SystemTime::now();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut game = game::Game::new();
    let mut exit = false;
    let mut events = Vec::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    keymod,
                    repeat,
                    ..
                } => events.push(input::EventType::KeyDown {
                    keycode,
                    keymod,
                    repeat,
                }),
                Event::KeyUp {
                    keycode: Some(keycode),
                    keymod,
                    repeat,
                    ..
                } => events.push(input::EventType::KeyUp {
                    keycode,
                    keymod,
                    repeat,
                }),
                _ => {}
            }
        }

        let delta_time = match SystemTime::now().duration_since(last_update_time) {
            Err(e) => Err(e.to_string())?,
            Ok(d) => d,
        };
        game.update(&mut events, &delta_time, &mut exit)?;
        last_update_time = SystemTime::now();

        ui::draw(&game, &mut canvas)?;

        canvas.present();

        if exit {
            break 'running;
        }
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
