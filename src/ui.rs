use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::game;

pub fn draw(game : &game::Game, canvas : &mut Canvas<Window>) -> Result<(), String> {
    let i = game.frame();
    canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    canvas.clear();

    let rect = Rect::new(50, 50, 100, 100);
    canvas.set_draw_color(Color::RGB(0, 0, 0));

    canvas.fill_rect(rect)?;
    overlay(game, canvas)?;
    Ok(())
}

fn overlay(_game : &game::Game, _canvas : &mut Canvas<Window>) -> Result<(), String> {
    Ok(())
}
