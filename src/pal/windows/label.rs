use super::safe_api;
use crate::gui;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use wchar::wch_c;
use winapi::shared::windef::*;
use winapi::um::winuser::*;

#[derive(Clone)]
pub struct Label {
    handle: HWND,
}

impl Label {
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
        Label { handle }
    }
}

#[cross_gui_derive::auto_clone]
impl gui::Label for Label {
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
    fn resize(&mut self, width: isize, height: isize) {
        unsafe {
            let mut old_rect = MaybeUninit::zeroed().assume_init();
            GetWindowRect(self.handle, &mut old_rect);
            let mut point = POINT {
                x: old_rect.top,
                y: old_rect.left
            };
            ScreenToClient(GetParent(self.handle), &mut point);
            safe_api::user32::set_window_pos(
                self.handle,
                null_mut(),
                point.x,
                point.y,
                width as i32,
                height as i32,
                SWP_NOZORDER,
            );
        }
    }
}
