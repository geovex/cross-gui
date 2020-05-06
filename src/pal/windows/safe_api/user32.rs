use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::windef::*;
use winapi::um::winuser::*;

pub fn post_quit_message(exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) };
}

pub fn set_window_text(window: HWND, text: &str) -> bool {
    let title_os_string: Vec<u16> = OsStr::new(text).encode_wide().chain(once(0)).collect();
    let status = unsafe { SetWindowTextW(window, title_os_string.as_ptr()) };
    status != 0
}

pub fn show_window(window: HWND, hidden: bool) -> bool {
    let status = unsafe { ShowWindow(window, hidden as i32) };
    status != 0
}

pub fn move_window(window: HWND, x: i32, y: i32, w: i32, h: i32, repaint: bool) -> bool {
    let status = unsafe { MoveWindow(window, x, y, w, h, repaint as i32) };
    status != 0
}

pub fn set_window_pos(
    window: HWND,
    insert_after: HWND,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    flags: u32,
) -> bool {
    let status = unsafe { SetWindowPos(window, insert_after, x, y, w, h, flags) };
    status != 0
}
