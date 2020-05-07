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
        unsafe {
            Box::from_raw(ptr);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
        };
    };
}

pub fn safe_wndproc(hwnd: HWND, u_msg: UINT, _w_param: WPARAM, _l_param: LPARAM) {
    let userdata = get_user_data(&hwnd);
    match u_msg {
        WM_CREATE => put_user_data(hwnd),
        WM_DESTROY => drop_user_data(hwnd),
        WM_CLOSE => {
            if userdata.unwrap().exit_on_close {
                user32::post_quit_message(0);
            }
        }
        _ => (),
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
        unsafe { RegisterClassW(&window_class) };
    });
}

struct SubclassUserData<C, F>
where
    F: Fn(&mut C, HWND, UINT, WPARAM, LPARAM),
{
    func: F,
    default_proc: WNDPROC,
    context: C,
}

unsafe extern "system" fn subclass_wnd_proc<C, F>(
    hwnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT
where
    F: Fn(&mut C, HWND, UINT, WPARAM, LPARAM),
{
    let data: *mut SubclassUserData<C, F> =
        mem::transmute(GetWindowLongPtrW(hwnd, GWLP_USERDATA));
    if !data.is_null() {
        let context: &mut SubclassUserData<C, F> = mem::transmute(data);
        (context.func)(&mut context.context, hwnd, u_msg, w_param, l_param);
        let result =
            CallWindowProcW(context.default_proc, hwnd, u_msg, w_param, l_param);
        if u_msg == WM_DESTROY {
            SetWindowLongPtrW(hwnd, GWLP_WNDPROC, mem::transmute(context.default_proc));
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
            Box::from_raw(data);
        }
        result
    } else {
        0
    }
}


pub fn subclass_window<C, F>(window: HWND, func: F, context: C)
where
    F: Fn(&mut C, HWND, UINT, WPARAM, LPARAM),
{
    let default_proc = unsafe { mem::transmute(GetWindowLongPtrW(window, GWLP_WNDPROC)) };
    let context = Box::new(SubclassUserData {
        func,
        default_proc,
        context,
    });
    let context_ptr = Box::into_raw(context);
    unsafe {
        SetWindowLongPtrW(window, GWLP_USERDATA, mem::transmute(context_ptr));
        SetWindowLongPtrW(
            window,
            GWLP_WNDPROC,
            mem::transmute(subclass_wnd_proc::<C, F> as isize)
        );
    };
}
