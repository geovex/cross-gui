use super::Widget;

pub trait Button: Widget {
    fn set_title(&mut self, title: &str);
    fn set_on_clicked(&mut self, on_pressed: Box<dyn FnMut()>);
}