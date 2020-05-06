use ::pal;

pub mod window;
pub use self::window::*;
pub mod widget;
pub use self::widget::*;
pub mod button;
pub use self::button::*;
pub mod label;
pub use self::label::*;

#[cfg(target_os = "linux")]
pub fn default_gui() -> Box<dyn Gui> {
    Box::new(pal::GtKGui::new())
}
#[cfg(target_os = "windows")]
pub fn default_gui() -> Box<dyn Gui> {
    Box::new(pal::Win32Gui::new())
}


pub trait Gui {
    fn new_window(&mut self) -> Box<dyn Window>;
    fn new_button(&mut self) -> Box<dyn Button>;
    fn new_label(&mut self) -> Box<dyn Label>;
    fn event_loop(&mut self);
}