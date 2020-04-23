use std::{ffi::OsStr, ptr::null_mut};
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::TRUE;
use winapi::shared::windef::HWND;
use winapi::um::winuser::*;

use super::wndclass;

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

impl ::gui::Window for PalWindow {
    fn set_title(&mut self, title: &str) {
        let title_os_string: Vec<u16> = OsStr::new(title).encode_wide().collect();
        unsafe { SetWindowTextW(self.handle, title_os_string.as_ptr() )};
    }
    fn set_hidden(&mut self, hidden: bool) {
        unsafe { ShowWindow(self.handle, if hidden { SW_HIDE } else { SW_SHOW }) };
    }
    fn move_(&mut self, x: isize, y: isize, w: isize, h: isize) {
        unsafe { MoveWindow(self.handle, x as i32, y as i32, w as i32, h as i32, TRUE) };
    }
    fn set_exit_on_close(&mut self, exit: bool) {
        let user_data = wndclass::get_user_data(&self.handle);
        user_data.unwrap().exit_on_close = exit;
    }
}
