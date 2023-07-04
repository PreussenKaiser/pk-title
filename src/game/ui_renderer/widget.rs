use sdl2::{render::Canvas, video::Window};

pub trait Widget {
    fn draw(&self, canvas: &Canvas<Window>) -> Result<(), String>;
}