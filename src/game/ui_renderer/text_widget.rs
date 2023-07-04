use sdl2::{rect::Point, render::Canvas, video::Window};

use super::widget::Widget;

pub struct TextWidget {
    text: String,
    pos: Point,
    size: u8
}

impl TextWidget {
    pub fn new(text: &str, pos: Point, size: u8) -> Self {
        Self {
            text: text.to_string(),
            pos,
            size
        }
    }
}

impl Widget for TextWidget {
    fn draw(&self, canvas: &Canvas<Window>) -> Result<(), String> {
        todo!()
    }
}