use crate::gui::Widget;
pub trait Window: Widget {
    fn set_title(&mut self, title: &str);
    fn set_exit_on_close(&mut self, exit: bool);
    fn add_widget(&mut self, widget: Box<dyn Widget>);
}