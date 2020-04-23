pub trait Window {
    fn set_title(&mut self, title: &str);
    fn set_exit_on_close(&mut self, exit: bool);
    fn set_hidden(&mut self, hidden: bool);
    fn move_(&mut self, x: isize, y: isize, w: isize, h: isize);
}