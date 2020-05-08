use super::Widget;

#[cross_gui_derive::gui_trait]
pub trait Label: Widget{
    fn set_title(&mut self, title: &str);
}