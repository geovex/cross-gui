use winapi::winuser::WNDCLASSW;
use winapi::minwindef::*;
use winapi::windef::*;
use user32::{DefWindowProcW, RegisterClassW, UnregisterClassW};
use std::os::raw::c_int;
use std::ptr;

use super::super::utf16z;

pub trait WndProcWrapper {
    fn wndproc(hwnd: HWND, u: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT {
        unsafe { DefWindowProcW(hwnd, u, wp, lp) }
    }
}

pub struct WndclassWrapper {
    wcl: WNDCLASSW,
    menu_name: Vec<u16>,
    class_name: Vec<u16>,
    registered: Option<ATOM>,
}

extern "system" fn wndproc<WP: WndProcWrapper>(hwnd: HWND,
                                               u: UINT,
                                               wp: WPARAM,
                                               lp: LPARAM)
                                               -> LRESULT {
    WP::wndproc(hwnd, u, wp, lp)
}

impl WndclassWrapper {
    pub fn new<WP: WndProcWrapper>(style: UINT,
                                   cbClsExtra: c_int,
                                   cbWndExtra: c_int,
                                   hInstance: HINSTANCE,
                                   hIcon: HICON,
                                   hCursor: HCURSOR,
                                   hbrBackground: HBRUSH,
                                   menu_name: Option<&str>,
                                   class_name: Option<&str>)
                                   -> WndclassWrapper {
        let mut result_wcl = WNDCLASSW {
            style: style,
            lpfnWndProc: Some(wndproc::<WP>),
            cbClsExtra: cbClsExtra,
            cbWndExtra: cbWndExtra,
            hInstance: hInstance,
            hIcon: hIcon,
            hCursor: hCursor,
            hbrBackground: hbrBackground,
            lpszMenuName: ptr::null(),
            lpszClassName: ptr::null(),
        };
        let mut result = WndclassWrapper {
            wcl: result_wcl,
            menu_name: vec![],
            class_name: vec![],
            registered: None,
        };
        if let Some(name) = menu_name {
            result.menu_name = utf16z(name);
            result.wcl.lpszMenuName = result.menu_name.as_ptr();
        };
        if let Some(name) = class_name {
            result.class_name = utf16z(name);
            result.wcl.lpszClassName = result.class_name.as_ptr();
        };
        result
    }

    pub fn register(&mut self) -> ATOM {
        let result = unsafe { RegisterClassW(&self.wcl) };
        if result != 0 {
            self.registered = Some(result);
        };
        result
    }

    pub fn unregister(&mut self) -> BOOL {
        match self.registered {
            Some(atom) => unsafe { UnregisterClassW(self.wcl.lpszClassName, ptr::null_mut()) },
            None => FALSE,
        }
    }
}

impl Drop for WndclassWrapper {
    fn drop(&mut self) {
        self.unregister();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use winapi::winuser::*;
    use kernel32::GetModuleHandleW;
    use user32::LoadCursorW;
    use std::mem;
    use super::super::kernel;
    #[test]
    fn register_class() {
        struct MyWP;
        impl WndProcWrapper for MyWP {};
        let mut wcl =
            WndclassWrapper::new::<MyWP>(CS_HREDRAW | CS_VREDRAW,
                                         0,
                                         0,
                                         unsafe { GetModuleHandleW(ptr::null_mut()) },
                                         ptr::null_mut(),
                                         unsafe { LoadCursorW(ptr::null_mut(), IDC_ARROW) },
                                         ptr::null_mut(), //unsafe { mem::transmute_copy(&COLOR_WINDOW) },
                                         None,
                                         Some("MYWINDOW"));
        assert!(wcl.register() != 0,
                "win_error: {}",
                kernel::error_message(kernel::get_last_error()));
    }
}
