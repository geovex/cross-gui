use gtk;
use std::marker::PhantomData;
use ::gui::Gui;
use std::sync::Once;

pub mod window;
pub use self::window::*;

static INIT: Once = Once::new();

pub struct PalGui<M> {
    phantom: PhantomData<M>
}

impl<M> Gui for PalGui<M> {
    type Window = ::pal::Window;
    type Msg = M;
    fn new() -> PalGui<M> {
        INIT.call_once(|| gtk::init().unwrap());
        PalGui {
            phantom: PhantomData
        }
    }
    fn new_window(&mut self) -> Self::Window {
        ::pal::Window::new()
    }
    fn event_loop(&mut self) {
        gtk::main();
    }
    fn send_msg(&mut self, _msg: M) {
        unimplemented!()
    }
}