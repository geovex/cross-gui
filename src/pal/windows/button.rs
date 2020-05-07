use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use winapi::um::winuser::*;
use wchar::wch_c;

use crate::gui;
use super::wndclass::subclass_window;
use super::safe_api;
use std::rc::Rc;
use std::ptr::null_mut;
use std::cell::RefCell;

#[derive(Default)]
struct ButtonContext{
    on_clicked: Option<Box<dyn FnMut()>>
}

#[derive(Clone)]
pub struct Button{
    handle: HWND,
    context: Rc<RefCell<ButtonContext>>
}

impl Button {
    pub fn new() -> Button {
        let context = Rc::new(RefCell::new(ButtonContext::default()));
        let handle = unsafe {
            CreateWindowExW(
                0u32,
                wch_c!("BUTTON").as_ptr(),
                null_mut(),
                WS_VISIBLE | BS_DEFPUSHBUTTON,
                0,
                0,
                50,
                100,
                null_mut(),
                null_mut(),
                null_mut(),
                null_mut(),
            )
        };
        subclass_window(handle, default_window_proc, context.clone());
        Button { handle, context }
    }
}

impl gui::Widget for Button {
    fn upcast(&self) -> Box<dyn gui::Widget> {
        Box::new(self.clone())
    }
    fn get_native(&self) -> Box<dyn std::any::Any> {
        Box::new(self.handle)
    }
    fn set_hidden(&mut self, hidden: bool) {
        safe_api::user32::show_window(self.handle, hidden);
    }
    fn move_(&mut self, x: isize, y: isize, w: isize, h: isize) { 
        safe_api::user32::move_window(self.handle, x as i32, y as i32, w as i32, h as i32, true);

    }
}

impl gui::Button for Button {
    fn set_title(&mut self, title: &str) { 
        safe_api::user32::set_window_text(self.handle, title);
    }
    fn set_on_clicked(&mut self, on_pressed: Box<dyn FnMut()>) {
        self.context.borrow_mut().on_clicked = Some(on_pressed);
    }
}


fn default_window_proc(ctx: &mut Rc<RefCell<ButtonContext>>, _hwnd: HWND, u_msg: UINT, w_param: WPARAM, _l_msg: LPARAM) {
    if (u_msg == WM_COMMAND) && (HIWORD(w_param as u32) == BN_CLICKED) {
        let borrowed_ctx = &mut *ctx.borrow_mut();
        borrowed_ctx.on_clicked.as_mut().map(|f| f());
    }
}