pub mod kernel;

use std::ptr::{null, null_mut};
use winapi::shared::minwindef::*;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;
use winapi::um::winuser::{MSG, WNDCLASSW};

use std::{sync::Once, mem::MaybeUninit};

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

pub fn utf16z<S: Into<String>>(s: S) -> Vec<u16> {
    let mut result: Vec<u16> = s.into().encode_utf16().collect();
    result.push(0u16);
    result
}

unsafe extern "system" fn wndproc(
    hwnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    unsafe { DefWindowProcW(hwnd, u_msg, w_param, l_param) }
}

static REGISTER_CLASS_ONCE: Once = Once::new();
pub const WNDCLASS_NAME: *const u16 = wchar::wch_c!("cross_gui_wndclass").as_ptr();

pub fn declare_wndclass() {
    REGISTER_CLASS_ONCE.call_once(|| {
        let window_class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: kernel::GetModuleHandle(None),
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: unsafe{ std::mem::transmute_copy(&COLOR_WINDOW) },
            lpszMenuName: null_mut(),
            lpszClassName: WNDCLASS_NAME,
        };
        let result = unsafe { RegisterClassW(&window_class) };
        dbg!(result);
    });
}

#[cfg(test)]
mod test {
    #[test]
    fn utf16z() {
        let test_result = super::utf16z("hi");
        assert_eq!(test_result.len(), 3);
        assert_eq!(test_result[2], 0u16);
    }
}
