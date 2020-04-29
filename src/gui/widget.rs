use std::any::Any;

pub trait Widget{
    fn upcast(&self) -> Box<dyn Widget>;
    fn get_native(&self) -> Box<dyn Any>;
    fn set_hidden(&mut self, hidden: bool);
    fn move_(&mut self, x: isize, y: isize, w: isize, h: isize);
}
