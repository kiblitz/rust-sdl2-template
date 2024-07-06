use crate::camera::Camera;
use crate::event_handler::EventHandler;
use crate::game_event::GameEvent;
use crate::texture_handler::TextureHandler;

use std::error::Error;

use sdl2::rect::Rect;

#[derive(Clone, Copy)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Bounds {
    #[inline(always)]
    pub fn rect(&self) -> Rect {
        Rect::new(
            (self.x - self.width / 2.) as i32,
            (self.y - self.height / 2.) as i32,
            self.width as u32,
            self.height as u32,
        )
    }
}

pub trait Updatable<'texture_handler> {
    fn update(
        &mut self,
        camera: &mut Camera,
        texture_handler: &'texture_handler TextureHandler,
        event_handler: &EventHandler,
        delta_time: f32,
    ) -> Result<GameEvent, Box<dyn Error>>;
}

pub trait Drawable<'texture_handler>: Updatable<'texture_handler> {
    fn draw(&self, camera: &mut Camera) -> Result<(), Box<dyn Error>>;
}
