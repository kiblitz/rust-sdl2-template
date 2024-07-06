use crate::camera::Camera;
use crate::event_handler::EventHandler;
use crate::game_event::GameEvent;
use crate::game_object::Bounds;
use crate::texture_handler::TextureHandler;

use std::error::Error;
use std::num::Wrapping;

use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Game<'game, 'texture_handler> {
    frame: Wrapping<u8>,
    camera: Camera,
    player_bounds: Bounds,
    speed: f32,
    background: &'game Texture<'texture_handler>,
}

impl<'texture_handler> Game<'_, 'texture_handler> {
    pub fn new(canvas: Canvas<Window>, texture_handler: &'texture_handler TextureHandler) -> Self {
        Self {
            frame: Wrapping(0u8),
            camera: Camera::new(canvas, 0., 0., 2000., 1200.),
            player_bounds: Bounds {
                x: 100.,
                y: 100.,
                width: 100.,
                height: 100.,
            },
            speed: 500.,
            background: texture_handler.stars_background(),
        }
    }

    pub fn frame(&self) -> u8 {
        self.frame.0
    }

    pub fn update(
        &mut self,
        _: &'texture_handler TextureHandler,
        event_handler: &EventHandler,
        delta_time: f32,
    ) -> Result<GameEvent, Box<dyn Error>> {
        if *event_handler.up() {
            self.player_bounds.y -= self.speed * delta_time;
        }
        if *event_handler.down() {
            self.player_bounds.y += self.speed * delta_time;
        }
        if *event_handler.left() {
            self.player_bounds.x -= self.speed * delta_time;
        }
        if *event_handler.right() {
            self.player_bounds.x += self.speed * delta_time;
        }
        self.frame += Wrapping(1u8);
        Ok(GameEvent::None)
    }

    pub fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        let i = self.frame();
        self.camera.set_canvas_draw_color(Color::RGB(
            if i < 128 { i } else { 255 - i },
            64,
            128 - if i < 128 { i } else { 255 - i },
        ));
        self.camera.canvas_clear();
        self.camera
            .canvas_copy_ex(self.background, None, None, 0.)?;

        let rect = self.player_bounds.rect();
        self.camera.set_canvas_draw_color(Color::RGB(
            if i < 128 { i } else { 255 - i },
            self.player_bounds.x as u8,
            self.player_bounds.y as u8,
        ));

        self.camera.canvas_fill_rect(&rect)?;
        self.camera.canvas_present();
        Ok(())
    }
}
