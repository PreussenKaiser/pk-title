use std::time::Duration;

use sdl2::VideoSubsystem;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Renderer {
    canvas: Canvas<Window>
}

impl Renderer {
    pub fn new(video_subsystem: VideoSubsystem) -> Result<Renderer, String> {
        let window = video_subsystem
            .window("Foo", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Renderer { canvas })
    }

    pub fn draw(&mut self) {
        self.render();
    }
    
    fn render(&mut self) {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        
        self.canvas.clear();
        self.canvas.present();
    }
}