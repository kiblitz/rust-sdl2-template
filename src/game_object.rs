use crate::event_handler::EventHandler;

use std::error::Error;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Updatable {
    fn update(
        &mut self,
        event_handler: &EventHandler,
        delta_time: &f32,
    ) -> Result<(), Box<dyn Error>>;
}

pub trait Drawable: Updatable {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>>;
}
