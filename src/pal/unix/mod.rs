use gtk;
use std::marker::PhantomData;
use ::gui::Gui;
use std::sync::{Once, ONCE_INIT};

static INIT: Once = ONCE_INIT;

pub struct PalGui<M> {
    phantom: PhantomData<M>
}

impl<M> Gui for PalGui<M> {
    type Window = gtk::Window;
    type Msg = M;
    fn new() -> PalGui<M> {
        INIT.call_once(|| gtk::init().unwrap());
        PalGui {
            phantom: PhantomData
        }
    }
    fn new_window(&mut self) -> gtk::Window {
        use gtk::WidgetExt;
        use gtk::WindowExt;
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_default_size(400,400);
        window.show_all();
        window
    }
    fn event_loop(&mut self) {
        gtk::main();
    }
    fn send_msg(&mut self, _msg: M) {
        unimplemented!()
    }
}