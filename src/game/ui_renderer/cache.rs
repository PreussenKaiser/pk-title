use sdl2::{render::Canvas, video::Window};

use super::{widget::Widget, widget_base::WidgetBase};

pub struct WidgetCache {
    cache: Vec<WidgetBase>,
    current: u32
}

impl WidgetCache {
    pub fn new() -> Self {
        Self {
            cache: Vec::new(),
            current: 0
        }
    }

    pub fn add(&mut self, widget: WidgetBase) {
        self.cache.push(widget);
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let size = self.cache.len();

        for i in 0..size {
            match &self.cache[i] {
                WidgetBase::Text(widget) => {
                    widget.draw(canvas)?;
                }
            }
        }

        Ok(())
    }
}