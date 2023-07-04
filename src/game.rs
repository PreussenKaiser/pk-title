use sdl2::{Sdl, video::Window, event::Event};

use self::{size::Size, ui_renderer::UIRenderer};

pub mod size;
mod ui_renderer;

pub struct Game {
    state: GameState,
    title: String,
    resolution: Size,
    frame_rate: u8
}

enum GameState {
    TITLE,
    PLAYING
}

impl Game {
    pub fn new(title: &str, resolution: Size, frame_rate: u8) -> Self {
        Self {
            title: String::from(title),
            state: GameState::TITLE,
            resolution,
            frame_rate
        }
    }

    pub fn run(self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let window = sdl_context
            .video()
            ?.window(
                self.title.as_str(),
                self.resolution.w(),
                self.resolution.h())
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        match self.state {
            GameState::TITLE => { self.render_title(&sdl_context, window)?; }
            GameState::PLAYING => { /* TODO: Render game */ }
        }

        Ok(())
    }

    fn render_title(self, sdl_context: &Sdl, window: Window) -> Result<(), String> {
        let mut renderer = UIRenderer::new();
        let mut event_pump = sdl_context.event_pump()?;

        'ui: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => { break 'ui }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}