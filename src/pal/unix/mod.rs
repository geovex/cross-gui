use gtk;
use ::gui;
use std::sync::Once;

pub mod window;
pub use self::window::*;
pub mod button;
pub use self::button::*;

static INIT: Once = Once::new();

pub struct GtKGui;

impl GtKGui{
    pub fn new() -> GtKGui {
        INIT.call_once(|| gtk::init().unwrap());
        GtKGui
    }
}

impl gui::Gui for GtKGui {
    fn new_window(&mut self) -> Box<dyn gui::Window> {
        Box::new(Window::new())
    }
    fn new_button(&mut self) -> Box<dyn gui::Button> {
        Box::new(Button::new())
    }
    fn event_loop(&mut self) {
        gtk::main();
    }
}