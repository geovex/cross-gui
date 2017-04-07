pub mod kernel;

use winapi::winuser::{MSG};
use winapi::minwindef::{BOOL, LRESULT, UINT};
use winapi::windef::HWND;
use user32::{GetMessageW, DispatchMessageW, TranslateMessage};

use std::mem;

pub fn get_message(wnd: HWND, filter_min: UINT, filter_max: UINT) -> Result<MSG, ()> {
    let mut result: MSG;
    unsafe{ result = mem::uninitialized() };
    let status = unsafe { GetMessageW(&mut result, wnd, filter_min, filter_max) };
    if status != 0{
        Ok(result)
    } else {
        Err(())
    }
}

pub fn translate_message(msg: &MSG) -> BOOL {
    unsafe {TranslateMessage(msg)}
}

pub fn dispatch_message(msg: &MSG) -> LRESULT {
    unsafe {DispatchMessageW(msg)}
}

pub fn utf16z<S: Into<String>>(s: S) -> Vec<u16> {
	let mut result: Vec<u16> = s.into().encode_utf16().collect();
	result.push(0u16);
	result
}

#[cfg(test)]
mod test{
	#[test]
	fn utf16z() {
		let test_result = super::utf16z("hi");
		assert_eq!(test_result.len(), 3);
		assert_eq!(test_result[2], 0u16);
	}
}