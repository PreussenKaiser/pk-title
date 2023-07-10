use sdl2::{render::Canvas, video::Window};

use super::widget::Widget;

pub struct WidgetCache {
    cache: Vec<Box<dyn Widget>>,
    selected: u8 // 255 is enough for now.
}

impl WidgetCache {
    pub fn new() -> Self {
        return Self {
            cache: Vec::new(),
            selected: 0
        };
    }

    pub fn selected(self) -> u8 { return self.selected; }

    pub fn select(&mut self, index: u8) -> Option<&Box<dyn Widget>> {
        let widget = self.cache.get(index as usize);

        if widget.is_some() {
            self.selected = index;

            let unwrapped = widget.unwrap();
        }

        return widget;
    }

    pub fn add<T: Widget + 'static>(&mut self, widget: T) {
        self.cache.push(Box::new(widget));
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let size = self.cache.len();

        for i in 0..size {
            let widget = &self.cache[i];

            widget.draw(canvas)?;
        }

        return Ok(());
    }
}