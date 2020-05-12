use crate::gui;
use std::any::Any;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;

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
    fn resize(&mut self, width: isize, height: isize) {
        unsafe {
            let mut old_rect = MaybeUninit::zeroed().assume_init();
            GetClientRect(self.handle, &mut old_rect);
            old_rect.right = old_rect.left + width as i32;
            old_rect.bottom = old_rect.top + height as i32;
            AdjustWindowRect(&mut old_rect, WS_VISIBLE | WS_OVERLAPPEDWINDOW, FALSE);
            safe_api::user32::set_window_pos(
                self.handle,
                null_mut(),
                old_rect.left,
                old_rect.top,
                old_rect.right - old_rect.left,
                old_rect.bottom - old_rect.top,
                SWP_NOZORDER,
            );
        }
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
    fn move_(&mut self, x: isize, y: isize, w: isize, h: isize) {
        let (x, y, w, h) = unsafe {
            let mut adj_rect = RECT {
                left: x as i32,
                top: y as i32,
                right: (x + w) as i32,
                bottom: (y + h) as i32,
            };
            AdjustWindowRect(&mut adj_rect, WS_VISIBLE | WS_OVERLAPPEDWINDOW, FALSE);
            (
                adj_rect.left,
                adj_rect.top,
                adj_rect.right - adj_rect.left,
                adj_rect.bottom - adj_rect.top,
            )
        };
        safe_api::user32::set_window_pos(
            self.handle,
            HWND_TOP,
            x as i32,
            y as i32,
            w as i32,
            h as i32,
            SWP_NOZORDER,
        );
    }
    fn move_child(&mut self, widget: &dyn gui::Widget, x: isize, y: isize, w: isize, h: isize) {
        let native_any = widget.get_native();
        let native: &HWND = native_any.as_ref().downcast_ref().unwrap();
        safe_api::user32::move_window(*native, x as i32, y as i32, w as i32, h as i32, true);
    }
}
