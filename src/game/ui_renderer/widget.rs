use sdl2::{render::Canvas, video::Window, rect::Point};

pub trait Widget {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
    fn pos(&self) -> Point;
    fn size(&self) -> u8;
    fn handle_event(&self);
    fn select(&mut self);
    fn unselect(&mut self);
}
