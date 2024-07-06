use crate::game_object::Bounds;
use crate::util;

use std::error::Error;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Camera {
    canvas: Canvas<Window>,
    bounds: Bounds,
    target_bounds: Bounds,
    x_lerp_constant: f32,
    y_lerp_constant: f32,
}

impl Camera {
    pub fn new(canvas: Canvas<Window>, x: f32, y: f32, width: f32, height: f32) -> Self {
        let bounds = Bounds {
            x,
            y,
            width,
            height,
        };
        Self {
            canvas,
            bounds,
            target_bounds: bounds,
            x_lerp_constant: 0.19,
            y_lerp_constant: 0.065,
        }
    }

    pub fn set_target(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.target_bounds = Bounds {
            x,
            y,
            width,
            height,
        }
    }

    pub fn set_canvas_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }

    pub fn canvas_copy_ex(
        &mut self,
        texture: &Texture,
        src: Option<Rect>,
        dst: Option<Rect>,
        angle: f64,
    ) -> Result<(), Box<dyn Error>> {
        let dst = match dst {
            Some(dst) => Some(self.projected_rect(&dst)?),
            None => None,
        };
        self.canvas
            .copy_ex(texture, src, dst, angle, None, false, false)?;
        Ok(())
    }

    pub fn canvas_fill_rect(&mut self, rect: &Rect) -> Result<(), Box<dyn Error>> {
        self.canvas.fill_rect(self.projected_rect(rect)?)?;
        Ok(())
    }

    pub fn canvas_clear(&mut self) {
        self.canvas.clear();
    }

    pub fn canvas_present(&mut self) {
        self.canvas.present();
    }

    pub fn update(&mut self, delta_time: f32) -> () {
        self.bounds.x = util::lerp(
            self.bounds.x,
            self.target_bounds.x,
            self.x_lerp_constant,
            delta_time,
        );
        self.bounds.y = util::lerp(
            self.bounds.y,
            self.target_bounds.y,
            self.y_lerp_constant,
            delta_time,
        );
        self.bounds.width = util::lerp(
            self.bounds.width,
            self.target_bounds.width,
            self.x_lerp_constant,
            delta_time,
        );
        self.bounds.height = util::lerp(
            self.bounds.height,
            self.target_bounds.height,
            self.y_lerp_constant,
            delta_time,
        );
    }

    #[inline(always)]
    fn projected_rect(&self, rect: &Rect) -> Result<Rect, Box<dyn Error>> {
        let (canvas_width, canvas_height) = self.canvas.output_size()?;
        let scale_x = (canvas_width as f32) / self.bounds.width;
        let scale_y = (canvas_height as f32) / self.bounds.height;
        Ok(Rect::new(
            (scale_x * (rect.x() as f32 - self.bounds.x)) as i32,
            (scale_y * (rect.y() as f32 - self.bounds.y)) as i32,
            (scale_x * (rect.width() as f32)) as u32,
            (scale_y * (rect.height() as f32)) as u32,
        ))
    }
}
