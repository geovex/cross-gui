use super::safe_api::kernel;
use super::safe_api::user32;
use std::{mem, ptr::null_mut, sync::Once};
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;

pub struct UserData {
    pub exit_on_close: bool,
}

impl UserData {
    pub fn new() -> UserData {
        UserData {
            exit_on_close: true,
        }
    }
}

pub fn get_user_data(hwnd: &HWND) -> Option<&mut UserData> {
    let ptr: *mut UserData = unsafe { mem::transmute(GetWindowLongPtrW(*hwnd, GWLP_USERDATA)) };
    if ptr.is_null() {
        None
    } else {
        unsafe { mem::transmute(&mut *ptr) }
    }
}

fn put_user_data(hwnd: HWND) {
    let user_data = Box::new(UserData::new());
    let user_data_ptr = Box::into_raw(user_data);
    unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, mem::transmute(user_data_ptr)) };
}
fn drop_user_data(hwnd: HWND) {
    let ptr: *mut UserData = unsafe { mem::transmute(GetWindowLongPtrW(hwnd, GWLP_USERDATA)) };
    if !ptr.is_null() {
        unsafe { Box::from_raw(ptr) };
    };
}

pub fn safe_wndproc(hwnd: HWND, u_msg: UINT, _w_param: WPARAM, _l_param: LPARAM) {
    let userdata = get_user_data(&hwnd);
    match u_msg {
        WM_CREATE => put_user_data(hwnd),
        WM_DESTROY => drop_user_data(hwnd),
        WM_CLOSE => if userdata.unwrap().exit_on_close {
            user32::post_quit_message(0);
        }
        _ => ()
    }
}

unsafe extern "system" fn wndproc(
    hwnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    safe_wndproc(hwnd, u_msg, w_param, l_param);
    DefWindowProcW(hwnd, u_msg, w_param, l_param)
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
            hInstance: kernel::get_module_handle(None),
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: unsafe { std::mem::transmute_copy(&COLOR_WINDOW) },
            lpszMenuName: null_mut(),
            lpszClassName: WNDCLASS_NAME,
        };
        let result = unsafe { RegisterClassW(&window_class) };
        dbg!(result);
    });
}
