use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::shared::minwindef::TRUE;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{CreateWindowExW, MoveWindow, ShowWindow};
use winapi::um::winuser::{SW_HIDE, SW_SHOW, WS_OVERLAPPEDWINDOW, WS_VISIBLE};

pub struct PalWindow {
    handle: HWND,
}

impl PalWindow {
    pub fn new() -> PalWindow {
        let class_name: Vec<u16> = OsStr::new("BUTTON").encode_wide().chain(once(0)).collect();
        let hwnd = unsafe {
            CreateWindowExW(
                0u32,
                class_name.as_ptr(),
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
    fn set_hidden(&mut self, hidden: bool) {
        unsafe { ShowWindow(self.handle, if hidden { SW_HIDE } else { SW_SHOW }) };
    }
    fn move_(&mut self, x: isize, y: isize, w: isize, h: isize) {
        unsafe { MoveWindow(self.handle, x as i32, y as i32, w as i32, h as i32, TRUE) };
    }
}
