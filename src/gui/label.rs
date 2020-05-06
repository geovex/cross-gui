use super::Widget;

pub trait Label: Widget{
    fn set_title(&mut self, title: &str);
}