pub mod kernel;
pub mod user32;
use winapi::shared::minwindef::*;
use winapi::shared::windef::HWND;
use winapi::um::winuser::*;
use winapi::um::winuser::MSG;

use std::mem::MaybeUninit;

pub fn get_message(wnd: HWND, filter_min: UINT, filter_max: UINT) -> Result<MSG, ()> {
    let mut result: MSG;
    unsafe { result = MaybeUninit::zeroed().assume_init() };
    let status = unsafe { GetMessageW(&mut result, wnd, filter_min, filter_max) };
    if status != 0 {
        Ok(result)
    } else {
        Err(())
    }
}

pub fn translate_message(msg: &MSG) -> BOOL {
    unsafe { TranslateMessage(msg) }
}

pub fn dispatch_message(msg: &MSG) -> LRESULT {
    unsafe { DispatchMessageW(msg) }
}

