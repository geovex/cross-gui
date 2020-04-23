use std::ptr::{null_mut};
use std::marker::PhantomData;

mod safe_api;
mod window;
mod wndclass;
use self::window::PalWindow;
use crate::gui;

pub struct Win32Gui;

impl Win32Gui {
    pub fn new() -> Win32Gui {
        wndclass::declare_wndclass();
        Win32Gui
    }
}

impl gui::Gui for Win32Gui {
    fn new_window(&mut self) -> Box<dyn gui::Window> {
        Box::new(PalWindow::new())
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