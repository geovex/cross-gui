use std::any::Any;

pub trait Widget{
    fn upcast(&self) -> Box<dyn Widget>;
    fn get_native(&self) -> Box<dyn Any>;
    fn set_hidden(&mut self, hidden: bool);
    fn resize(&mut self, width: isize, height: isize);
}
