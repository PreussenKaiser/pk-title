use self::{cache::WidgetCache, widget::Widget, widget_base::WidgetBase};

mod cache;
mod text_widget;
mod widget;
mod widget_base;

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

    pub fn add_widget(mut self, widget: WidgetBase) {
        self.widgets.add(widget);
    }

    pub fn stop(mut self) {
        self.running = false;
    }
}