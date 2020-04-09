pub trait Window {
    fn set_hidden(&mut self, hidden: bool);
    fn move_(&mut self, x: isize, y: isize, w: isize, h: isize);
}