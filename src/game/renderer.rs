use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use super::context::{GameContext, Point};
use super::domain::PIXEL_SIZE;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e: sdl2::IntegerOrSdlError| e.to_string())?;
        Ok(Renderer { canvas })
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::BLUE);

        for enemy in context.enemies.iter() {
            self.dot(enemy)?
        }
        self.canvas.set_draw_color(Color::RED);

        self.dot(&context.player_position)?;

        self.canvas.present();
        Ok(())
    }

    pub fn dot(&mut self, point: &Point) -> Result<(), String> {
        let Point { x, y } = point;

        self.canvas.fill_rect(Rect::new(
            x * PIXEL_SIZE as i32,
            y * PIXEL_SIZE as i32,
            PIXEL_SIZE,
            PIXEL_SIZE,
        ))
    }
}
