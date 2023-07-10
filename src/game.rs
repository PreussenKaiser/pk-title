use sdl2::{Sdl, video::Window, event::Event, rect::Point};

use crate::game::ui_renderer::text_widget::TextWidget;

use self::{size::Size, ui_renderer::{UIRenderer, button_widget::ButtonWidget}};

pub mod size;
mod ui_renderer;

pub struct Game {
    state: GameState,
    title: String,
    resolution: Size
}

enum GameState { TITLE }

impl Game {
    pub fn new(title: &str, resolution: Size) -> Self {
        return Self {
            title: String::from(title),
            state: GameState::TITLE,
            resolution
        };
    }

    pub fn run(self) -> Result<(), String> {
        let sdl_context = sdl2::init()?;
        let window = sdl_context
            .video()
            ?.window(
                self.title.as_str(),
                self.resolution.w(),
                self.resolution.h()
            )
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        match self.state {
            GameState::TITLE => { self.render_title(&sdl_context, window)?; }
        }

        return Ok(());
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

        return Ok(());
    }

    fn build_title(self) -> UIRenderer {
        let title_widget = TextWidget::new(self.title.as_str(), Point::new(64, 64), 92);
        let play_widget = ButtonWidget::new(TextWidget::new("PLAY", Point::new(64, 164), 64), true);
        let exit_widget = ButtonWidget::new(TextWidget::new("EXIT", Point::new(64, 264), 64), false);
        
        let mut renderer = UIRenderer::new();

        renderer
            .add_widget(title_widget)
            .add_widget(play_widget)
            .add_widget(exit_widget);

        return renderer;
    }
}