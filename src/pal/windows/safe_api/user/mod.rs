pub mod wndclass;
pub mod userwnd;

use winapi::minwindef::*;
use winapi::windef::*;
use user32::DefWindowProcW;

pub fn def_window_proc(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, msg, wp, lp) }
}