use crate::event_handler::EventHandler;
use crate::game_object::{Drawable, Updatable};

use std::error::Error;
use std::num::Wrapping;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Game {
    frame: Wrapping<u8>,
    x: f32,
    y: f32,
    speed: f32,
}

impl Updatable for Game {
    fn update(
        &mut self,
        event_handler: &EventHandler,
        delta_time: &f32,
    ) -> Result<(), Box<dyn Error>> {
        if event_handler.up() {
            self.y -= self.speed * delta_time;
        }
        if event_handler.down() {
            self.y += self.speed * delta_time;
        }
        if event_handler.left() {
            self.x -= self.speed * delta_time;
        }
        if event_handler.right() {
            self.x += self.speed * delta_time;
        }
        self.frame += Wrapping(1u8);
        Ok(())
    }
}
impl Drawable for Game {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
        let (_width, _height) = canvas.output_size()?;

        let i = self.frame();
        canvas.set_draw_color(Color::RGB(
            if i < 128 { i } else { 255 - i },
            64,
            128 - if i < 128 { i } else { 255 - i },
        ));
        canvas.clear();

        let (x, y) = self.pos();
        let rect = Rect::new(x as i32, y as i32, 100, 100);
        canvas.set_draw_color(Color::RGB(
            if i < 128 { i } else { 255 - i },
            x as u8,
            y as u8,
        ));

        canvas.fill_rect(rect)?;
        self.overlay(canvas)?;
        Ok(())
    }
}
impl Game {
    pub fn new() -> Self {
        Self {
            frame: Wrapping(0u8),
            x: 0.,
            y: 0.,
            speed: 500.,
        }
    }

    fn overlay(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
        let (_width, _height) = canvas.output_size()?;
        Ok(())
    }

    pub fn frame(&self) -> u8 {
        self.frame.0
    }

    pub fn pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}
