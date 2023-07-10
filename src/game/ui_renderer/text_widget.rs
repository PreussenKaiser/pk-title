use std::path::Path;

use sdl2::{rect::{Point, Rect}, render::Canvas, video::Window, pixels::Color};

use super::widget::Widget;

const FONT_PATH: &str = "assets/font.ttf";

pub struct TextWidget {
    text: String,
    pos: Point,
    size: u8
}

impl TextWidget {
    pub fn new(text: &str, pos: Point, size: u8) -> Self {
        return Self {
            text: text.to_string(),
            pos,
            size
        };
    }
}

impl Widget for TextWidget {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let font = ttf_context.load_font(Path::new(FONT_PATH), 128)?;

        let surface = font
            .render(self.text.as_str())
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let size = self.size as u32;
        let target = Rect::new(
            self.pos.x(),
            self.pos.y(),
            size * 2,
            size
        );

        canvas.copy(&texture, None, Some(target))?;

        return Ok(());
    }

    fn pos(&self) -> Point {
        return self.pos;
    }

    fn size(&self) -> u8 {
        return self.size;
    }
}