use crate::gui::Widget;

pub trait Button: Widget {
    fn set_title(&mut self, title: &str);
}