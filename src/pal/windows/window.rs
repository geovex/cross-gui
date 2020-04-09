use winapi::um::winuser::CreateWindowExW;
use winapi::um::winuser::{WS_OVERLAPPEDWINDOW, WS_VISIBLE};
use winapi::shared::windef::HWND;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::ptr::null_mut;

pub struct PalWindow {
    handle: HWND
}

impl PalWindow {
    pub fn new() -> PalWindow {
        let class_name: Vec<u16> = OsStr::new("BUTTON").encode_wide().chain(once(0)).collect();
        let hwnd = unsafe { CreateWindowExW(
            0u32,
            class_name.as_ptr(),
            null_mut(),
            WS_VISIBLE | WS_OVERLAPPEDWINDOW,
            0, 0, 400, 400,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut()
        )};
        PalWindow{
            handle: hwnd
        }
    }
}