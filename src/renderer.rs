use std::time::Duration;

use sdl2::VideoSubsystem;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const FONT_PATH: &str = "../assets/font.ttf"; // TODO: Inject

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

    pub fn draw(&mut self) -> Result<(), String> {
        self.render()?;

        Ok(())
    }
    
    fn render(&mut self) -> Result<(), String> {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let texture_creator = self.canvas.texture_creator();
        let font = ttf_context.load_font(FONT_PATH, 128)?;

        let surface = font
            .render(self.canvas.window().title())
            .blended(Color::WHITE)
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();

        let target = Rect::new(400, 300, 32, 32);
        self.canvas.copy(&texture, None, Some(target))?;

        self.canvas.present();

        Ok(())
    }
}