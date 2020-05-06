use winapi::shared::windef::*;
use winapi::um::winuser::*;
use wchar::wch_c;
use std::ptr::null_mut;
use crate::gui;
use super::safe_api;

#[derive(Clone)]
pub struct Label{
    handle: HWND
}

impl Label{
    pub fn new() -> Label {
        let handle = unsafe {
            CreateWindowExW(
                0u32,
                wch_c!("STATIC").as_ptr(),
                null_mut(),
                WS_VISIBLE | BS_DEFPUSHBUTTON,
                0,
                0,
                50,
                100,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            )
        };
        Label { handle  }
    }
}

impl gui::Label for Label{
    fn set_title(&mut self, title: &str) {
        safe_api::user32::set_window_text(self.handle, title);
    }
}

impl gui::Widget for Label {
    fn upcast(&self) -> Box<dyn gui::Widget> {
        Box::new(self.clone())
    }
    fn get_native(&self) -> Box<dyn std::any::Any> {
        Box::new(self.handle)
    }
    fn set_hidden(&mut self, hidden: bool) {
        safe_api::user32::show_window(self.handle, hidden);
    }
    fn move_(&mut self, x: isize, y: isize, w: isize, h: isize) {
        safe_api::user32::move_window(self.handle, x as i32, y as i32, w as i32, h as i32, true);
    }
    
}