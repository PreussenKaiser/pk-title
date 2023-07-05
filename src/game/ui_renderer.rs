use sdl2::{render::Canvas, video::Window, pixels::Color};

use self::{cache::WidgetCache, widget_base::WidgetBase};

mod cache;
pub mod text_widget;
mod widget;
pub mod widget_base;

pub struct UIRenderer {
    widgets: WidgetCache,
    running: bool
}

impl UIRenderer {
    pub fn new() -> Self {
        Self {
            widgets: WidgetCache::new(),
            running: true
         }
    }

    pub fn add_widget(&mut self, widget: WidgetBase) {
        self.widgets.add(widget);
    }

    pub fn stop(mut self) {
        self.running = false;
    }

    pub fn render(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.set_draw_color(Color::WHITE);
        self.widgets.draw(canvas)?;
        canvas.present();

        Ok(())
    }
}