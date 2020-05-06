use std::ptr::null_mut;
use std::any::Any;
use winapi::shared::windef::*;
use winapi::um::winuser::*;
use crate::gui;

use super::safe_api;
use super::wndclass;

#[derive(Clone)]
pub struct PalWindow {
    handle: HWND,
}

impl PalWindow {
    pub fn new() -> PalWindow {
        let hwnd = unsafe {
            CreateWindowExW(
                0u32,
                wndclass::WNDCLASS_NAME,
                null_mut(),
                WS_VISIBLE | WS_OVERLAPPEDWINDOW,
                0,
                0,
                400,
                400,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            )
        };
        PalWindow { handle: hwnd }
    }
}

impl gui::Widget for PalWindow {
    fn upcast(&self) -> Box<dyn gui::Widget> {
        Box::new(self.clone())
    }
    fn get_native(&self) -> Box<dyn Any> {
        Box::new(self.handle)
    }
    fn set_hidden(&mut self, hidden: bool) {
        unsafe { ShowWindow(self.handle, if hidden { SW_HIDE } else { SW_SHOW }) };
    }
    fn move_(&mut self, x: isize, y: isize, w: isize, h: isize) {
        safe_api::user32::set_window_pos(self.handle, HWND_TOP, x as i32, y as i32, w as i32, h as i32, SWP_NOZORDER);
    }

}

impl gui::Window for PalWindow {
    fn set_title(&mut self, title: &str) {
        safe_api::user32::set_window_text(self.handle, title);
    }
    fn set_exit_on_close(&mut self, exit: bool) {
        let user_data = wndclass::get_user_data(&self.handle);
        user_data.unwrap().exit_on_close = exit;
    }
    fn add_widget(&mut self, widget: Box<dyn gui::Widget>) {
        let native_any = widget.get_native();
        let native: &HWND = native_any.as_ref().downcast_ref().unwrap();
        unsafe { 
            SetWindowLongW(*native, GWL_STYLE, WS_VISIBLE as i32);
            SetParent(*native, self.handle);
            SetWindowPos(*native, HWND_NOTOPMOST, 0, 0, 10, 10, SWP_SHOWWINDOW);
        }
    }
}
