use sdl2::{render::Canvas, video::Window, pixels::Color};

use self::{cache::WidgetCache, widget::Widget};

pub mod button_widget;
mod cache;
pub mod text_widget;
mod widget;

pub struct UIRenderer {
    widgets: WidgetCache
}

impl UIRenderer {
    pub fn new() -> Self {
        return Self { widgets: WidgetCache::new() };
    }

    pub fn add_widget<T: Widget + 'static>(&mut self, widget: T) -> &mut Self {
        self.widgets.add(widget);

        return self;
    }

    pub fn nav_up(&mut self) {        
        self.widgets.select(1);
    }

    pub fn nav_down(&mut self) {
        self.widgets.select(2);
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