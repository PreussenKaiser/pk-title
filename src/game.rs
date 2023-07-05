use sdl2::{Sdl, video::Window, event::Event, rect::Point};

use crate::game::ui_renderer::text_widget::TextWidget;

use self::{size::Size, ui_renderer::{UIRenderer, widget_base::WidgetBase}};

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
        let mut renderer = self.build_title();
        let mut event_pump = sdl_context.event_pump()?;

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        'ui: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => { break 'ui }
                    _ => {}
                }
            }

            renderer.render(&mut canvas)?;
        }

        Ok(())
    }

    fn build_title(self) -> UIRenderer {
        let title_widget = TextWidget::new(
            self.title.as_str(),
            Point::new(64, 64),
            128
        );

        let mut renderer = UIRenderer::new();
        renderer.add_widget(WidgetBase::Text(title_widget));

        renderer
    }
}