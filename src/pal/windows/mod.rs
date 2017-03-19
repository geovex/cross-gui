use user32::{
    GetMessageW,
    TranslateMessage,
    DispatchMessageW
};
use winapi::winuser::{
    MSG,
    NPWNDCLASSW};
use ::gui::Gui;
use std::mem;
use std::ptr::{null_mut};
use std::marker::PhantomData;

mod window;
use self::window::PalWindow;

pub struct PalGui<M> {
    phantom: PhantomData<M>
}

impl<M> Gui for PalGui<M> {
    type Window = PalWindow;
    type Msg = M;
    fn new() -> PalGui<M> {
        PalGui {
            phantom: PhantomData
        }
    }
    fn new_window(&mut self) -> PalWindow {
        PalWindow::new()
    }
    fn event_loop(&mut self) {
        let mut msg: MSG = unsafe { mem::uninitialized() };
        loop {
            if 0 != unsafe { GetMessageW(&mut msg, null_mut(), 0u32, 0u32) } {
                unsafe {TranslateMessage(&mut msg)};
                unsafe {DispatchMessageW(&mut msg)};
            } else {
                return
            }
        }
    }
    fn send_msg(&mut self, _msg: M) {
        unimplemented!()
    }
}