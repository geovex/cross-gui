use gtk;
use ::gui;
use std::sync::Once;

pub mod window;
pub use self::window::*;
pub mod button;
pub use self::button::*;
pub mod label;
pub use self::label::*;

static INIT: Once = Once::new();

#[derive(Clone)]
pub struct GtKGui;


impl GtKGui{
    pub fn new() -> GtKGui {
        INIT.call_once(|| gtk::init().unwrap());
        GtKGui
    }
}

#[cross_gui_derive::auto_clone]
impl gui::Gui for GtKGui {
    fn new_window(&mut self) -> Box<dyn gui::Window> {
        Box::new(Window::new())
    }
    fn new_button(&mut self) -> Box<dyn gui::Button> {
        Box::new(Button::new())
    }
    fn new_label(&mut self) -> Box<dyn gui::Label> {
        Box::new(Label::new())
    }
    fn post_quit_message(&mut self) {
        gtk::main_quit();
    }
    fn event_loop(&mut self) {
        gtk::main();
    }
}

pub type NativeWiddget = gtk::Widget;