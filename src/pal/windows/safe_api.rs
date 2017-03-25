use winapi::winuser::MSG;
use winapi::minwindef::{BOOL, LRESULT};
use user32::{DispatchMessageW, TranslateMessage};

pub fn translate_message(msg: &MSG) -> BOOL {
    unsafe {TranslateMessage(msg)}
}

pub fn dispatch_message(msg: &MSG) -> LRESULT {
    unsafe {DispatchMessageW(msg)}
}