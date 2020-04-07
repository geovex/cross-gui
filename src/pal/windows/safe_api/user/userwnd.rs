use super::wndclass::*;
use user32::GetWindowLongPtrW;
use winapi::minwindef::*;
use winapi::windef::*;
use winapi::winuser::*;
use std::mem;
use std::ptr;

pub struct UserWnd<T: UserWndproc> {
    data: *mut T,
}

impl<T: UserWndproc> Drop for UserWnd<T> {
    fn drop(&mut self) {
        let b = unsafe { Box::from_raw(self.data) };
        self.data = ptr::null_mut();
    }
}

pub trait UserWndproc {
    fn wndproc(&mut self, hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT {
        super::def_window_proc(hwnd, msg, wp, lp)
    }
}

impl<T: UserWndproc> WndProcWrapper for UserWnd<T> {
    fn wndproc(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT {
        let ptr: *mut T = unsafe { mem::transmute(GetWindowLongPtrW(hwnd, GWLP_WNDPROC)) };
        let dataref = unsafe { ptr.as_mut().unwrap() };
        let result = dataref.wndproc(hwnd, msg, wp, lp);
        result
    }
}

lazy_static! {
    static ref user_wcl: WndClassWrapper = WndClassWrapper::new<UserWnd>(CS_HREDRAW | CS_VREDRAW,
                                                                         0,
                                                                         0,
                                                                         unsafe { GetModuleHandleW(ptr::null_mut()) },
                                                                         ptr::null_mut(),
                                                                         unsafe { LoadCursorW(ptr::null_mut(), IDC_ARROW) },
                                                                         unsafe { mem::transmute_copy(&COLOR_WINDOW) },
                                                                         None,
                                                                         Some("USERWND"));
}

pub fn create_window<UWP, S>(header: S) -> HWND
    where S: Into<&str>,
          UWP: UserWndproc
{
    unsafe{ ptr::null_mut() }
}