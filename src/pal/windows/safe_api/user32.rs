use winapi::um::winuser::*;
use winapi::shared::windef::*;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;


pub fn post_quit_message(exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) };
}

pub fn set_window_text(window: HWND, text: &str) -> bool {
    let title_os_string: Vec<u16> = OsStr::new(text).encode_wide().chain(once(0)).collect();
    let status = unsafe { SetWindowTextW(window, title_os_string.as_ptr()) };
    status != 0
}