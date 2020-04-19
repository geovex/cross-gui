use ::gui::Gui;
use std::ptr::{null_mut};
use std::marker::PhantomData;

mod safe_api;
mod window;
use self::window::PalWindow;

pub struct PalGui<M> {
    phantom: PhantomData<M>
}

impl<M> Gui for PalGui<M> {
    type Window = PalWindow;
    type Msg = M;
    fn new() -> PalGui<M> {
        safe_api::declare_wndclass();
        PalGui {
            phantom: PhantomData
        }
    }
    fn new_window(&mut self) -> PalWindow {
        PalWindow::new()
    }
    fn event_loop(&mut self) {
        loop {
            match safe_api::get_message(null_mut(), 0u32, 0u32) {
                Ok(msg) => {
                    safe_api::translate_message(&msg);
                    safe_api::dispatch_message(&msg);
                },
                Err(_) => return
            };
        }
    }
    fn send_msg(&mut self, _msg: M) {
        unimplemented!()
    }
}