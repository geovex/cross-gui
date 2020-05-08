use std::ptr::null_mut;

mod safe_api;
mod window;
mod button;
mod label;
mod wndclass;
use self::window::PalWindow;
use self::button::Button;
use self::label::Label;
use crate::gui;

#[derive(Clone)]
pub struct Win32Gui;

impl Win32Gui {
    pub fn new() -> Win32Gui {
        wndclass::declare_wndclass();
        Win32Gui
    }
}

impl gui::Gui for Win32Gui {
    fn cloned(&mut self) -> Box<dyn gui::Gui> {
        Box::new(Win32Gui)
    }
    fn new_window(&mut self) -> Box<dyn gui::Window> {
        Box::new(PalWindow::new())
    }
    fn new_button(&mut self) -> Box<dyn gui::Button> {
        Box::new(Button::new())
    }
    fn new_label(&mut self) -> Box<dyn gui::Label> {
        Box::new(Label::new())
    }
    fn post_quit_message(&mut self) {
        safe_api::user32::post_quit_message(0);
    }
    fn event_loop(&mut self) {
        loop {
            match safe_api::get_message(null_mut(), 0u32, 0u32) {
                Ok(msg) => {
                    safe_api::translate_message(&msg);
                    safe_api::dispatch_message(&msg);
                },
                Err(_) => return
            };
        }
    }
}

pub type NativeWiddget = winapi::shared::windef::HWND;