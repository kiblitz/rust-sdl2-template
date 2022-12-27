use crate::game;

use std::error::Error;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw(game: &game::Game, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
    let (_width, _height) = canvas.output_size()?;

    let i = game.frame();
    canvas.set_draw_color(Color::RGB(
        if i < 128 { i } else { 255 - i },
        64,
        128 - if i < 128 { i } else { 255 - i },
    ));
    canvas.clear();

    let (x, y) = game.pos();
    let rect = Rect::new(x, y, 100, 100);
    canvas.set_draw_color(Color::RGB(
        if i < 128 { i } else { 255 - i },
        x as u8,
        y as u8,
    ));

    canvas.fill_rect(rect)?;
    overlay(game, canvas)?;
    Ok(())
}

fn overlay(_game: &game::Game, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
    let (_width, _height) = canvas.output_size()?;
    Ok(())
}
