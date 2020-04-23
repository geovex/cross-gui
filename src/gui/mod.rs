use ::pal;

pub mod window;
pub use self::window::*;

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
    fn event_loop(&mut self);
}