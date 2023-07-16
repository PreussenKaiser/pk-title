
use sdl2::{video::Window, render::Canvas, rect::{Point, Rect}};

use super::widget::Widget;

pub struct ButtonWidget<T: Widget> {
    target: T,
    active: bool
}

impl<T: Widget> ButtonWidget<T> {
    pub fn new(target: T, active: bool) -> Self {
        return Self { target, active };
    }

    fn draw_underline(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let rect = Rect::new(
            self.pos().x(),
            self.pos().y() + self.size() as i32,
            self.size() as u32 * 2,
            8
        );

        return canvas.fill_rect(rect);
    }
}

impl<T: Widget> Widget for ButtonWidget<T> {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if self.active {
            self.draw_underline(canvas)?;
        }

        return self.target.draw(canvas);
    }

    fn pos(&self) -> Point { return self.target.pos(); }
    fn size(&self) -> u8 { return self.target.size(); }
    fn handle_event(&self) {
        // TODO: Execute closure.
    }
    
    fn select(&mut self) {
        self.active = true;
    }
    
    fn unselect(&mut self) {
        self.active = false;
    }
}