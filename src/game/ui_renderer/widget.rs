use sdl2::{render::Canvas, video::Window};

pub trait Widget {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}